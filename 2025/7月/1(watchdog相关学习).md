今日先把师兄的建议给接纳并修改了axcpu

然后是对看门狗驱动的学习编写https://gitee.com/phytium_embedded/phytium-linux-kernel/blob/linux-5.10/drivers/watchdog/sbsa_gwdt.c

```rust
 * ARM SBSA Generic Watchdog has two stage timeouts:
 * the first signal (WS0) is for alerting the system by interrupt,
 * the second one (WS1) is a real hardware reset.
 * More details about the hardware specification of this device:
 * ARM DEN0029B - Server Base System Architecture (SBSA)
 *
 * This driver can operate ARM SBSA Generic Watchdog as a single stage watchdog
 * or a two stages watchdog, it's set up by the module parameter "action".
 * In the single stage mode, when the timeout is reached, your system
 * will be reset by WS1. The first signal (WS0) is ignored.
 * In the two stages mode, when the timeout is reached, the first signal (WS0)
 * will trigger panic. If the system is getting into trouble and cannot be reset
 * by panic or restart properly by the kdump kernel(if supported), then the
 * second stage (as long as the first stage) will be reached, system will be
 * reset by WS1. This function can help administrator to backup the system
 * context info by panic console output or kdump.
```
简单来说，该驱动实现了对 SBSA 规范看门狗硬件的完整支持，其核心创新在于​​两阶段超时机制​​，为系统故障提供了更灵活的响应策略（如先收集日志再复位）。

通过模块参数动态配置工作模式，结合标准看门狗框架接口，确保了与用户态工具（如 /dev/watchdog）的兼容性

下面是一些问题的学习：

## kdump内核是什么
https://www.cnblogs.com/dongxb/p/17297629.html
```
kdump 内核是 Linux 系统中用于在内核崩溃时​​捕获内存转储（vmcore）​​的专用内核，也称为​​捕获内核（Capture Kernel）​​。它与主系统内核（生产内核）协同工作，是内核崩溃转储机制（Kdump）的核心组件。以下是其关键特性详解：

 ​​1. 核心作用与定位​​
​​故障现场保留​​：当主内核因崩溃（如 Panic/Oops）无法运行时，kdump 内核立即接管，将主内核的完整内存状态保存为 vmcore 文件，避免数据丢失。

​​独立运行环境​​：通过预留的内存区域启动，与主内核隔离，确保崩溃时内存内容不被覆盖。


​​2. 工作原理​​
​​内存预留​​：主内核启动时，通过启动参数（如 crashkernel=512M）保留一块专属内存（如 512MB），供 kdump 内核加载使用

​​快速切换（kexec）​​：崩溃时通过 kexec 机制直接跳转到预留内存中的 kdump 内核，跳过 BIOS/UEFI 初始化，实现毫秒级切换。

​​内存转储生成​​：kdump 内核将主内核的内存映射为 /proc/vmcore，并调用工具（如 makedumpfile）将其压缩保存至磁盘（如 /var/crash）。

​3. 关键特性​​

​​极简设计​​：
仅包含必要驱动和功能（如磁盘访问、网络传输）
通常仅启用单 CPU 核心以减少资源占用。

​​依赖组件​​：
​​kexec-tools​​：用户态工具，用于加载 kdump 内核到预留内存。
​​initramfs​​：最小化根文件系统，包含转储脚本和工具（如 crash）。

​4. 配置要点​​
​​内存预留​​：
典型配置：crashkernel=512M（物理内存 >16GB 时建议 ≥1GB）。
支持动态分配（如 crashkernel=auto），按系统内存自动调整。
​​内核支持​​：需启用编译选项：
CONFIG_CRASH_DUMP=y  
CONFIG_DEBUG_INFO=y  # 支持调试符号[1,5](@ref)。

​​服务管理​​：
Ubuntu：通过 kdump-tools.service 控制
RHEL：使用 kdump.service。

​​5. 典型工作流程​​
​​预留内存​​ → 主内核启动时保留 crashkernel 区域。
​​预加载内核​​ → kexec -p 将 kdump 内核加载至预留内存。
​​触发崩溃​​ → 主内核崩溃（如 echo c > /proc/sysrq-trigger）。
​​切换与转储​​ → kdump 内核启动，生成 /var/crash/YYYYMMDD/vmcore。
​​分析调试​​ → 使用 crash 工具分析 vmcore 和带调试符号的 vmlinux。

​总结​​
kdump 内核是 Linux 内核调试的“黑匣子”，通过​​隔离内存+快速切换​​的机制，在内核崩溃时完整保存故障现场。其有效性依赖于预留内存的合理配置、轻量化设计及自动化服务管理。运维与内核开发者需掌握其配置（如 crashkernel 参数）和分析方法（crash 工具），以快速定位系统级故障。
```

## kexec是什么
```
kexec机制
kexec简介
Kexec是基于kexec机制工作的，因此先了解一下Kexec。

kexec是一个快速启动机制，允许通过已经运行的内核的上下文启动一个Linux内核，不需要经过BIOS。（BIOS可能会消耗很多时间，特别是带有众多数量的外设的大型服务器。这种办法可以为经常启动机器的开发者节省很多时间。）

Kexec的实现包括2个组成部分：

** 一是内核空间的系统调用：kexec_load() **，负责在生产内核（production kernel 或 first kernel）启动时将捕获内核（capture kernel或sencond kernel）加载到指定地址。

** 二是用户空间的工具kexec-tools **，他将捕获内核的地址传递给生产内核，从而在系统崩溃的时候能够找到捕获内核的地址并运行。没有kexec就没有kdump。先有kexec实现了在一个内核中可以启动另一个内核，才让kdump有了用武之地。

kexec_load()
kexec 在 kernel 里以一个系统调用 kexec_load()的形式提供给用户。这个系统调用主要用来把另一个内核和其 ramdisk 加载到当前内核中。在 kdump中，捕获内核只能使用事先预留的一小段内存。

生产内核的内存镜像会被以 /proc/vmcore 的形式提供给用户。这是一个 ELF格式的方件，它的头是由用户空间工具 kexec 生成并传递来的。在系统崩溃时，系统最后会调用machine_kexec()。这通常是一个硬件相关的函数。它会引导捕获内核，从而完成 kdump 的过程。

kexec-tools
kdump 的很大一部分工作都是在用户空间内完成的。与 kexec相关的集中在一个叫kexec-tools的工具中的kexec程序中。

该程序主要是为调用 kexec_load()收集各种信息，然后调用之。这些信息主要包括 purgatory 的入口地址，还有一组由 struct kexec_segment描述的信息。
```
## vmcore是什么
```
vmcore 是 ​​Linux 系统内核崩溃时生成的内存转储文件​​，记录了崩溃瞬间的完整物理内存状态及内核关键数据，用于诊断系统崩溃的根本原因。以下是其核心要点：

⚙️ ​​1. 核心定义与作用​​
​​内核故障快照​​：当 Linux 内核发生严重错误（如硬件故障、驱动崩溃或内核代码缺陷）时，vmcore 会捕获崩溃瞬间的物理内存内容、CPU 寄存器状态、进程列表、堆栈跟踪等关键信息。

​​诊断核心工具​​：通过分析 vmcore，可定位崩溃原因（如空指针访问、内存泄漏、死锁等），提升系统稳定性。

⚡ ​​2. 生成机制​​
​​依赖 Kdump 服务​​：
vmcore 由 ​​kdump 机制​​自动生成。主内核崩溃时，kdump 启动预留内存中的 ​​捕获内核​​，将主内核的内存快照保存为 vmcore。
​​配置要求​​：需预留内存（如启动参数 crashkernel=512M）并启用 kdump 服务。
​​存储位置​​：默认保存在 /var/crash/ 目录下，文件大小与物理内存一致（如 64GB 内存生成 64GB 的 vmcore）。

🔍 ​​3. 核心内容​​
vmcore 文件包含崩溃时的完整系统状态：

​​物理内存数据​​：所有内存页的原始内容。
​​进程与线程信息​​：崩溃瞬间的活动进程列表及上下文。
​​硬件状态​​：CPU 寄存器值、设备驱动状态、中断请求（IRQ）记录。
​​内核结构​​：虚拟内存映射、文件系统缓存、网络连接状态等。
​​崩溃元数据​​：错误类型（如 kernel BUG）、触发位置（指令指针 RIP）。

🛠️ ​​4. 分析方法​​
​​必需工具​​：
​​crash 工具​​：主流分析工具，需配合 ​​vmlinux 符号文件​​（含调试信息的内核镜像）解析内存地址。
示例命令：
crash /usr/lib/debug/boot/vmlinux-5.4.0-1-generic /var/crash/vmcore
​​关键命令​​：
bt：查看崩溃时的堆栈跟踪。
ps：列出崩溃时的进程状态。
vm：分析内存使用情况。

​​辅助工具​​：
​​vmcore-dmesg.txt​​：伴随 vmcore 生成的日志，记录崩溃前的内核消息（如 Oops 错误），可通过在线工具（如 Red Hat 的 Kernel Oops Analyzer）快速分析。
​​gdb/SystemTap​​：支持深度调试，但需手动解析符号。

⚠️ ​​5. 注意事项​​
​​符号文件匹配​​：分析 vmcore 时，​​vmlinux 必须与崩溃内核版本完全一致​​，否则无法解析地址。
​​安全与存储​​：
vmcore 可能包含敏感数据（如用户密码、加密密钥），需加密存储和传输。
大内存系统需确保磁盘空间充足（如 1TB 内存需预留 1TB 存储）。
​​性能影响​​：启用 kdump 会预留部分内存（通常 256MB–1GB），可能减少可用内存。

💎 ​​典型应用场景​​
​​场景​​	​​分析目标​​	​​工具/命令​​
内核空指针崩溃	定位 RIP 寄存器指向的代码行	crash + bt
内存泄漏	检查未释放的内存块及分配函数	crash + kmem
死锁问题	分析进程阻塞时的锁持有链	crash + lock
驱动崩溃	查看驱动模块状态及寄存器值	crash + mod

✅ ​​总结​​
vmcore 是 Linux 内核崩溃分析的基石，通过 ​​kdump 机制生成​​，需结合 ​​vmlinux 符号文件​​和 ​​crash 工具​​解析，为诊断硬件故障、内核缺陷提供不可替代的现场数据。运维人员应掌握其生成配置（预留内存、服务启用）和分析方法，以快速恢复系统并修复根因。
```
- 总结：用一句话解释，通过kexec机制在系统预留内存中预加载捕获内核，当生产内核崩溃时立即切换到捕获内核，并将生产内核的内存状态以ELF格式转储为/proc/vmcore文件​​。

# 寄存器部分
```
#define SBSA_GWDT_WRR  0x000  // 刷新帧寄存器（喂狗写任意值）
#define SBSA_GWDT_WCS  0x000  // 控制帧状态寄存器（使能位/中断标志）
#define SBSA_GWDT_WOR  0x008  // 超时值寄存器（设置超时时间）
#define SBSA_GWDT_WCV  0x010  // 当前计数器寄存器（64位）
#define SBSA_GWDT_WCS_EN  BIT(0)  // 使能位
#define SBSA_GWDT_WCS_WS0 BIT(1) // 第一阶段中断标志
#define SBSA_GWDT_WCS_WS1 BIT(2) // 第二阶段复位标志
```

# 设备结构体
```
struct sbsa_gwdt {
    struct watchdog_device wdd;   // 标准看门狗设备
    u32 clk;                      // 系统时钟频率（Hz）
    void __iomem *refresh_base;   // 刷新帧虚拟地址（喂狗）
    void __iomem *control_base;   // 控制帧虚拟地址（配置）
};
```

# 模块参数
```
模块参数
static unsigned int timeout;
module_param(timeout, uint, 0);
MODULE_PARM_DESC(timeout, "Watchdog timeout in seconds. (>=0, default="
		 __MODULE_STRING(DEFAULT_TIMEOUT) ")");
timeout：模块参数，设置看门狗超时时间（单位：秒）。

module_param：表示该参数可以通过模块加载时传递。


static int action;
module_param(action, int, 0);
MODULE_PARM_DESC(action, "after watchdog gets WS0 interrupt, do: "
		 "0 = skip(*)  1 = panic");
action：模块参数，用于决定当看门狗接收到 WS0 信号时采取的行动（0：忽略，1：触发 panic）。


static bool nowayout = WATCHDOG_NOWAYOUT;
module_param(nowayout, bool, S_IRUGO);
MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started (default="
		 __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
nowayout：模块参数，指定是否可以停止看门狗（默认不可停止）。

```

# 具体操作函数
```
设置看门狗超时。
static int sbsa_gwdt_set_timeout(struct watchdog_device *wdd, unsigned int timeout)
{
	struct sbsa_gwdt *gwdt = watchdog_get_drvdata(wdd);

	wdd->timeout = timeout;
	timeout = clamp_t(unsigned int, timeout, 1, wdd->max_hw_heartbeat_ms / 1000);

	if (action)
		writel(gwdt->clk * timeout,
		       gwdt->control_base + SBSA_GWDT_WOR);
	else
		/*
		 * In the single stage mode, The first signal (WS0) is ignored,
		 * the timeout is (WOR * 2), so the WOR should be configured
		 * to half value of timeout.
		 */
		writel(gwdt->clk / 2 * timeout,
		       gwdt->control_base + SBSA_GWDT_WOR);

	return 0;
}
```

```
获取看门狗剩余时间。
static unsigned int sbsa_gwdt_get_timeleft(struct watchdog_device *wdd)
{
	struct sbsa_gwdt *gwdt = watchdog_get_drvdata(wdd);
	u64 timeleft = 0;

	/*
	 * In the single stage mode, if WS0 is deasserted
	 * (watchdog is in the first stage),
	 * timeleft = WOR + (WCV - system counter)
	 */
	if (!action &&
	    !(readl(gwdt->control_base + SBSA_GWDT_WCS) & SBSA_GWDT_WCS_WS0))
		timeleft += readl(gwdt->control_base + SBSA_GWDT_WOR);

	timeleft += lo_hi_readq(gwdt->control_base + SBSA_GWDT_WCV) -
		    arch_timer_read_counter();

	do_div(timeleft, gwdt->clk);

	return timeleft;
}
```

```
重置看门狗
static int sbsa_gwdt_keepalive(struct watchdog_device *wdd)
{
	struct sbsa_gwdt *gwdt = watchdog_get_drvdata(wdd);

	/*
	 * Writing WRR for an explicit watchdog refresh.
	 * You can write anyting (like 0).
	 */
	writel(0, gwdt->refresh_base + SBSA_GWDT_WRR);

	return 0;
}
```

```
启动看门狗
static int sbsa_gwdt_start(struct watchdog_device *wdd)
{
	struct sbsa_gwdt *gwdt = watchdog_get_drvdata(wdd);

	/* writing WCS will cause an explicit watchdog refresh */
	writel(SBSA_GWDT_WCS_EN, gwdt->control_base + SBSA_GWDT_WCS);

	return 0;
}

关闭看门狗
static int sbsa_gwdt_stop(struct watchdog_device *wdd)
{
	struct sbsa_gwdt *gwdt = watchdog_get_drvdata(wdd);

	/* Simply write 0 to WCS to clean WCS_EN bit */
	writel(0, gwdt->control_base + SBSA_GWDT_WCS);

	return 0;
}

看门狗中断处理函数，发生超时时触发 panic
static irqreturn_t sbsa_gwdt_interrupt(int irq, void *dev_id)
{
	panic(WATCHDOG_NAME " timeout");

	return IRQ_HANDLED;
}
```

# 看门狗设备注册
```
static const struct watchdog_info sbsa_gwdt_info = {
	.identity	= WATCHDOG_NAME,
	.options	= WDIOF_SETTIMEOUT |
			  WDIOF_KEEPALIVEPING |
			  WDIOF_MAGICCLOSE |
			  WDIOF_CARDRESET,
};
// 定义看门狗设备的名字，还有支持的功能选项（其实有点怪，这里只用到了最后一个标志位，表示系统重启的来源是否是来自看门狗）

static const struct watchdog_ops sbsa_gwdt_ops = {
	.owner		= THIS_MODULE,
	.start		= sbsa_gwdt_start,
	.stop		= sbsa_gwdt_stop,
	.ping		= sbsa_gwdt_keepalive,
	.set_timeout	= sbsa_gwdt_set_timeout,
	.get_timeleft	= sbsa_gwdt_get_timeleft,
};
// 定义了一组函数，并告知了这个函数的拥有者是谁，以及其他相关函数，在rust中就是为某个结构体实现相关函数了
```

# 主体函数
```c
static int sbsa_gwdt_probe(struct platform_device *pdev)
{
	void __iomem *rf_base, *cf_base;
	struct device *dev = &pdev->dev;
	struct watchdog_device *wdd;
	struct sbsa_gwdt *gwdt;
	int ret, irq;
	u32 status;
    // rf_base 和 cf_base：分别是看门狗的“刷新寄存器”和“控制寄存器”的虚拟地址。__iomem 是内核标识符，表示这些是内存映射 I/O 地址。
    // dev：指向当前平台设备的指针。
    // wdd：指向 watchdog_device 结构体的指针，表示看门狗设备。
    // gwdt：指向 sbsa_gwdt 结构体的指针，它是此设备的私有数据结构。
    // ret：用于存储函数返回值（例如错误码）。
    // irq：用于存储中断请求（IRQ）号。
    // status：用于存储从看门狗控制寄存器读取的状态。


	gwdt = devm_kzalloc(dev, sizeof(*gwdt), GFP_KERNEL);
	if (!gwdt)
		return -ENOMEM;
	platform_set_drvdata(pdev, gwdt);
    // devm_kzalloc：分配内存并清零，为 gwdt 结构体分配空间。如果分配失败，则返回 -ENOMEM。
    // platform_set_drvdata(pdev, gwdt)：将 gwdt 作为平台设备的私有数据，保存到设备结构中。以后可以通过设备访问这个私有数据。

	cf_base = devm_platform_ioremap_resource(pdev, 0);
	if (IS_ERR(cf_base))
		return PTR_ERR(cf_base);

	rf_base = devm_platform_ioremap_resource(pdev, 1);
	if (IS_ERR(rf_base))
		return PTR_ERR(rf_base);
    // devm_platform_ioremap_resource：将平台设备的 I/O 地址映射到虚拟内存地址空间，以便在内核中访问。如果映射失败，返回错误。

	/*
	 * Get the frequency of system counter from the cp15 interface of ARM
	 * Generic timer. We don't need to check it, because if it returns "0",
	 * system would panic in very early stage.
	 */
	gwdt->clk = arch_timer_get_cntfrq(); // arch_timer_get_cntfrq()：从 ARM 架构的计时器获取系统计时器的频率（单位：Hz）。该值用于设置看门狗的超时。
	gwdt->refresh_base = rf_base;
	gwdt->control_base = cf_base; // 将映射的 I/O 地址保存到 gwdt 结构体中，refresh_base 和 control_base 分别指向看门狗的刷新寄存器和控制寄存器。

	wdd = &gwdt->wdd;
	wdd->parent = dev;
	wdd->info = &sbsa_gwdt_info;
	wdd->ops = &sbsa_gwdt_ops;
	wdd->min_timeout = 1;
	wdd->max_hw_heartbeat_ms = U32_MAX / gwdt->clk * 1000;
	wdd->timeout = DEFAULT_TIMEOUT;
	watchdog_set_drvdata(wdd, gwdt);
	watchdog_set_nowayout(wdd, nowayout);
    // 初始化 watchdog_device：将 wdd（watchdog_device）结构体的各个字段初始化：
    // 
    // parent：设备父节点指向当前设备。
    // 
    // info：设置为 sbsa_gwdt_info，包含看门狗设备的基本信息。
    // 
    // ops：设置为 sbsa_gwdt_ops，这包含了看门狗操作函数（如启动、停止等）。
    // 
    // min_timeout 和 max_hw_heartbeat_ms：设置看门狗的最小超时时间和最大硬件超时时间。
    // 
    // timeout：设置默认超时为 10 秒。
    // 
    // watchdog_set_drvdata：将 gwdt 作为私有数据与 watchdog_device 关联。
    // 
    // watchdog_set_nowayout：设置 nowayout，指示看门狗是否可以停止。

	status = readl(cf_base + SBSA_GWDT_WCS);
	if (status & SBSA_GWDT_WCS_WS1) {
		dev_warn(dev, "System reset by WDT.\n");
		wdd->bootstatus |= WDIOF_CARDRESET;
	}
    // readl：从控制寄存器中读取状态。
    // 
    // SBSA_GWDT_WCS_WS1：检查 WS1 位，表示系统是否已由看门狗触发硬件重置。
    // 
    // 如果触发了重置：
    // 
    // dev_warn：发出警告，通知系统已经被看门狗重置。
    // 
    // wdd->bootstatus |= WDIOF_CARDRESET：设置 WDIOF_CARDRESET 标志，表示系统是由于看门狗重置的。


	if (status & SBSA_GWDT_WCS_EN)
		set_bit(WDOG_HW_RUNNING, &wdd->status);
    // 如果看门狗启用（SBSA_GWDT_WCS_EN 位被设置），则将 WDOG_HW_RUNNING 标志设置为 1，表示硬件看门狗正在运行。

	if (action) {
		irq = platform_get_irq(pdev, 0);
		if (irq < 0) {
			action = 0;
			dev_warn(dev, "unable to get ws0 interrupt.\n");
		} else {
			/*
			 * In case there is a pending ws0 interrupt, just ping
			 * the watchdog before registering the interrupt routine
			 */
			writel(0, rf_base + SBSA_GWDT_WRR);
			if (devm_request_irq(dev, irq, sbsa_gwdt_interrupt, 0,
					     pdev->name, gwdt)) {
				action = 0;
				dev_warn(dev, "unable to request IRQ %d.\n",
					 irq);
			}
		}
		if (!action)
			dev_warn(dev, "falling back to single stage mode.\n");
	}
    // action：检查是否启用了双阶段模式。
    // 
    // platform_get_irq：获取看门狗中断请求号（IRQ）。如果无法获取中断，警告并回退到单阶段模式。
    // 
    // devm_request_irq：请求中断并注册中断处理函数 sbsa_gwdt_interrupt。


	/*
	 * In the single stage mode, The first signal (WS0) is ignored,
	 * the timeout is (WOR * 2), so the maximum timeout should be doubled.
	 */
	if (!action)
		wdd->max_hw_heartbeat_ms *= 2;
    // 如果回退到单阶段模式，则将看门狗的最大超时时间加倍。

	watchdog_init_timeout(wdd, timeout, dev);
	/*
	 * Update timeout to WOR.
	 * Because of the explicit watchdog refresh mechanism,
	 * it's also a ping, if watchdog is enabled.
	 */
	sbsa_gwdt_set_timeout(wdd, wdd->timeout);

	watchdog_stop_on_reboot(wdd);
	ret = devm_watchdog_register_device(dev, wdd);
	if (ret)
		return ret;
    // watchdog_init_timeout：根据 timeout 参数初始化超时时间
    // sbsa_gwdt_set_timeout：设置硬件超时。
    // watchdog_stop_on_reboot：设置在重启时停止看门狗。
    // devm_watchdog_register_device：注册看门狗设备。

	dev_info(dev, "Initialized with %ds timeout @ %u Hz, action=%d.%s\n",
		 wdd->timeout, gwdt->clk, action,
		 status & SBSA_GWDT_WCS_EN ? " [enabled]" : "");
    // dev_info：输出设备初始化日志，显示超时时间、计时器频率、动作和启用状态。

	return 0;
}
```

# 一些补充
```c
/* Disable watchdog if it is active during suspend */
static int __maybe_unused sbsa_gwdt_suspend(struct device *dev)
{
	struct sbsa_gwdt *gwdt = dev_get_drvdata(dev);

	if (watchdog_active(&gwdt->wdd))
		sbsa_gwdt_stop(&gwdt->wdd);
    // watchdog_active(&gwdt->wdd)：这是一个内核函数，用于检查看门狗设备是否处于活动状态。wdd 是 sbsa_gwdt 中的 watchdog_device 结构体，表示看门狗设备。
    // 
    // sbsa_gwdt_stop：这是定义在驱动中的一个函数，用于停止看门狗。它将关闭看门狗计时器，防止在系统挂起时触发看门狗。
    // 
    // 将 wdd 作为参数传递，表示停止与该 watchdog_device 相关联的看门狗
    // 
    // 如果看门狗设备处于活动状态（即看门狗计时器正在运行），函数将返回 true

	return 0;
}
// __maybe_unused：这是一个宏，告诉编译器“这个函数可能不会被使用”，避免编译器因为函数未被调用而发出警告。
// 
// sbsa_gwdt_suspend：这是一个自定义的挂起函数，处理看门狗在挂起时的状态。它的作用是当设备挂起时，禁用看门狗。
// 
// struct device *dev：这是挂起操作的设备指针，表示调用挂起操作的设备。通过此指针可以获取设备的状态及其私有数据。
// 
// dev_get_drvdata(dev)：这是一个内核函数，用于获取与设备相关的私有数据。设备驱动通常会将设备的私有数据（如硬件控制结构体）与设备关联，使用该函数可以获取到这些数据。
// 
// gwdt：这是指向 sbsa_gwdt 结构体的指针，sbsa_gwdt 结构体是用于表示 SBSA 看门狗设备的结构体，其中包含了看门狗相关的硬件控制信息。
```

```c
/* Enable watchdog if necessary (系统恢复)*/
static int __maybe_unused sbsa_gwdt_resume(struct device *dev)
{
	struct sbsa_gwdt *gwdt = dev_get_drvdata(dev);
    // dev_get_drvdata(dev)：这是一个内核函数，用于获取与设备关联的私有数据。在设备初始化时，私有数据通常通过 platform_set_drvdata 函数与设备关联，dev_get_drvdata 允许我们获取这些数据。
    // 
    // gwdt：这是指向 sbsa_gwdt 结构体的指针，sbsa_gwdt 结构体包含了与 SBSA 看门狗设备相关的硬件信息和控制结构。

	if (watchdog_active(&gwdt->wdd))
		sbsa_gwdt_start(&gwdt->wdd);

	return 0;
}
```
```c
这段代码是 Linux 驱动程序的核心部分，用于定义和注册 SBSA Generic Watchdog 驱动。它涉及设备的电源管理（PM）、设备树匹配、平台设备匹配和驱动注册。下面我们逐行解析这些代码。

1. 电源管理操作
static const struct dev_pm_ops sbsa_gwdt_pm_ops = {
	SET_SYSTEM_SLEEP_PM_OPS(sbsa_gwdt_suspend, sbsa_gwdt_resume)
};
dev_pm_ops：这是 Linux 内核用于设备电源管理的操作结构体。它包含了一系列的回调函数，用于处理设备的挂起（suspend）、恢复（resume）、关闭（shutdown）等电源管理操作。

SET_SYSTEM_SLEEP_PM_OPS：这个宏将系统挂起（sbsa_gwdt_suspend）和恢复（sbsa_gwdt_resume）操作关联到 dev_pm_ops 结构体中。具体地：

sbsa_gwdt_suspend：当系统进入挂起状态时调用，负责禁用看门狗。

sbsa_gwdt_resume：当系统恢复时调用，负责重新启用看门狗。

2. 设备树匹配
static const struct of_device_id sbsa_gwdt_of_match[] = {
	{ .compatible = "arm,sbsa-gwdt", },
	{},
};
MODULE_DEVICE_TABLE(of, sbsa_gwdt_of_match);
of_device_id：这是一个结构体，用于在设备树（Device Tree）中匹配设备。设备树是一个描述硬件的结构，通常用于 ARM 架构的设备。

sbsa_gwdt_of_match：这个数组列出了支持的设备树设备名称（compatible）。在这里，设备的 compatible 字段是 "arm,sbsa-gwdt"，表示支持 SBSA Generic Watchdog 设备。

MODULE_DEVICE_TABLE(of, sbsa_gwdt_of_match)：这个宏将 sbsa_gwdt_of_match 数组注册到内核的设备表中，这样内核就可以通过设备树匹配到这个驱动。

3. 平台设备匹配
static const struct platform_device_id sbsa_gwdt_pdev_match[] = {
	{ .name = DRV_NAME, },
	{},
};
MODULE_DEVICE_TABLE(platform, sbsa_gwdt_pdev_match);
platform_device_id：这是一个结构体，用于在平台设备上匹配驱动。平台设备一般是通过平台总线（Platform Bus）连接的设备，通常用于没有特定设备树支持的硬件。

sbsa_gwdt_pdev_match：这个数组定义了平台设备名称（name）。设备名与平台驱动名称相匹配，DRV_NAME 是一个宏，代表了驱动的名称。

MODULE_DEVICE_TABLE(platform, sbsa_gwdt_pdev_match)：这个宏将平台设备匹配表注册到内核，内核会根据设备名称匹配平台驱动。

4. 平台驱动定义
static struct platform_driver sbsa_gwdt_driver = {
	.driver = {
		.name = DRV_NAME,
		.pm = &sbsa_gwdt_pm_ops,
		.of_match_table = sbsa_gwdt_of_match,
	},
	.probe = sbsa_gwdt_probe,
	.id_table = sbsa_gwdt_pdev_match,
};
platform_driver：这是一个结构体，用于描述一个平台驱动。平台驱动是专门为特定硬件平台编写的驱动程序，通常用于特定的设备，如 SBSA Generic Watchdog。

sbsa_gwdt_driver：这是一个具体的驱动实例，定义了驱动的名称、PM 操作、设备树匹配表、设备匹配表等。

name：驱动的名称，通常与设备名称相同，这里是 DRV_NAME。

pm：指定了设备的电源管理操作，关联到 sbsa_gwdt_pm_ops，包含设备挂起和恢复操作。

of_match_table：设备树匹配表，关联到 sbsa_gwdt_of_match，用于通过设备树匹配设备。

probe：驱动的探测函数，当设备与驱动匹配时，probe 函数将被调用，初始化设备。

id_table：平台设备匹配表，关联到 sbsa_gwdt_pdev_match，用于匹配平台设备。

5. 注册平台驱动

module_platform_driver(sbsa_gwdt_driver);
module_platform_driver：这是一个宏，用于注册平台驱动。当内核加载这个模块时，它会自动注册平台驱动，将其与相应的设备匹配。

它会调用驱动的 probe 函数，以便初始化和配置设备。

6. 模块描述和元数据
MODULE_DESCRIPTION("SBSA Generic Watchdog Driver");
MODULE_AUTHOR("Fu Wei <fu.wei@linaro.org>");
MODULE_AUTHOR("Suravee Suthikulpanit <Suravee.Suthikulpanit@amd.com>");
MODULE_AUTHOR("Al Stone <al.stone@linaro.org>");
MODULE_AUTHOR("Timur Tabi <timur@codeaurora.org>");
MODULE_LICENSE("GPL v2");
MODULE_ALIAS("platform:" DRV_NAME);
MODULE_DESCRIPTION：描述模块的功能，这里是 "SBSA Generic Watchdog Driver"，表示这是一个 SBSA 通用看门狗驱动程序。

MODULE_AUTHOR：列出模块的作者信息，每个 MODULE_AUTHOR 行都列出了一个作者。

MODULE_LICENSE：指定模块的许可证。这里是 GPL v2，表示该模块使用 GNU General Public License v2 版本。

MODULE_ALIAS：定义模块的别名。通过别名，内核可以在加载模块时识别它。
```