use tock_registers::interfaces::{Readable, Writeable, ReadWriteable};
use tock_registers::register_structs;
use tock_registers::register_bitfields;
use tock_registers::registers::{ReadOnly, ReadWrite};
use core::sync::atomic::{AtomicU32, Ordering};

// ================== 寄存器定义 (根据文档修正) ==================
register_structs! {
    pub PwmRegisters {
        // === 死区控制寄存器空间 (0x000~0x3FF) ===
        (0x0000 => dbctrl: ReadWrite<u32, DBCTRL::Register>),
        (0x0004 => dbdly: ReadWrite<u32, DBDLY::Register>),
        (0x0008 => _reserved_db: [u8; 0x3F8]),
        
        // === PWM0通道寄存器空间 (0x400~0x7FF) ===
        (0x0400 => ch0_tim_cnt: ReadWrite<u32, TIM_CNT::Register>),
        (0x0404 => ch0_tim_ctrl: ReadWrite<u32, TIM_CTRL::Register>),
        (0x0408 => ch0_state: ReadWrite<u32, STATE::Register>),
        (0x040C => ch0_pwm_period: ReadWrite<u32, PWM_PERIOD::Register>),
        (0x0410 => ch0_pwm_ctrl: ReadWrite<u32, PWM_CTRL::Register>),
        (0x0414 => ch0_pwm_ccr: ReadWrite<u32, PWM_CCR::Register>),
        (0x0418 => _reserved_ch0: [u8; 0x3E8]),
        
        // === PWM1通道寄存器空间 (0x800~0xBFF) ===
        (0x0800 => ch1_tim_cnt: ReadWrite<u32, TIM_CNT::Register>),
        (0x0804 => ch1_tim_ctrl: ReadWrite<u32, TIM_CTRL::Register>),
        (0x0808 => ch1_state: ReadWrite<u32, STATE::Register>),
        (0x080C => ch1_pwm_period: ReadWrite<u32, PWM_PERIOD::Register>),
        (0x0810 => ch1_pwm_ctrl: ReadWrite<u32, PWM_CTRL::Register>),
        (0x0814 => ch1_pwm_ccr: ReadWrite<u32, PWM_CCR::Register>),
        (0x0818 => @END),
    }
}

register_bitfields! {
    u32,
    
    // === DBCTRL (0x00) ===
    pub DBCTRL [
        OUT_MODE          OFFSET(4)  NUMBITS(2) [
            Bypass = 0b00,
            FallEdgeOnly = 0b01,
            RiseEdgeOnly = 0b10,
            FullDeadband = 0b11
        ],
        
        POLSEL            OFFSET(2)  NUMBITS(2) [
            AH = 0b00,      // PWM0_OUT和PWM1_OUT都不翻转
            ALC = 0b01,     // PWM0_OUT翻转
            AHC = 0b10,     // PWM1_OUT翻转
            AL = 0b11       // PWM0_OUT和PWM1_OUT都翻转
        ],
        
        IN_MODE           OFFSET(1)  NUMBITS(1) [
            PWM0 = 0,
            PWM1 = 1
        ],
        
        DB_SW_RST         OFFSET(0)  NUMBITS(1) [
            Normal = 0,
            ResetActive = 1
        ]
    ],
    
    // === DBDLY (0x04) ===
    pub DBDLY [
        DBFED             OFFSET(10) NUMBITS(10) [], // 下降沿延迟周期
        DBRED             OFFSET(0)  NUMBITS(10) []  // 上升沿延迟周期
    ],
    
    // === TIM_CNT (0x400/0x800) ===
    pub TIM_CNT [
        CNT               OFFSET(0)  NUMBITS(16) []  // 当前计数值
    ],
    
    // === TIM_CTRL (0x404/0x804) === (DIV偏移修正)
    pub TIM_CTRL [
        DIV               OFFSET(16) NUMBITS(12) [], // [27:16] 分频参数
        
        GIE              OFFSET(5)  NUMBITS(1) [],   // 全局中断输出使能
        OVFIF_ENABLE     OFFSET(4)  NUMBITS(1) [],   // 溢出中断使能
        
        MODE             OFFSET(2)  NUMBITS(1) [
            Modulo = 0,      // 模计数
            UpAndDown = 1    // 三角计数
        ],
        
        ENABLE           OFFSET(1)  NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        
        SW_RST           OFFSET(0)  NUMBITS(1) [
            Normal = 0,
            ResetActive = 1
        ]
    ],
    
    // === STATE (0x408/0x808) === (完全修正，添加所有中断标志)
    pub STATE [
        // 所有标志使用RW1c（写1清除）
        FIFO_FULL     OFFSET(3)  NUMBITS(1) [], // FIFO满标志
        FIFO_EMPTY    OFFSET(2)  NUMBITS(1) [], // FIFO空中断标志
        OVFIF         OFFSET(1)  NUMBITS(1) [], // 计数器溢出中断标志
        CHIF          OFFSET(0)  NUMBITS(1) []  // 比较匹配中断标志
    ],
    
    // === PWM_PERIOD (0x40C/0x80C) ===
    pub PWM_PERIOD [
        CCR               OFFSET(0)  NUMBITS(16) []  // 周期控制值
    ],
    
    // === PWM_CTRL (0x410/0x810) ===
    pub PWM_CTRL [
        FIFO_EMPTY_ENABLE OFFSET(9)  NUMBITS(1) [],   // FIFO空中断使能
        DUTY_SEL          OFFSET(8)  NUMBITS(1) [
            Register = 0,
            FIFO = 1
        ],
        
        ICOV              OFFSET(7)  NUMBITS(1) [],  // 初始输出值
        
        CMP               OFFSET(4)  NUMBITS(3) [
            SetOnMatch = 0b000,
            ClearOnMatch = 0b001,
            ToggleOnMatch = 0b010,
            SetOnUpClearOnDown = 0b011,
            ClearOnUpSetOnDown = 0b100,
            ClearOnCCRSetOnPeriod = 0b101,
            SetOnCCRClearOnPeriod = 0b110,
            Initialize = 0b111
        ],
        
        IE                OFFSET(3)  NUMBITS(1) [],   // 比较中断使能
        
        MODE              OFFSET(2)  NUMBITS(1) [
            FreeRunning = 0,
            Compare = 1
        ]
    ],
    
    // === PWM_CCR (0x414/0x814) ===
    pub PWM_CCR [
        CCR               OFFSET(0)  NUMBITS(16) []  // 占空比控制值
    ]
}

// ================== 驱动实现 ==================
const SYSTEM_CLK: u32 = 50_000_000; // 50MHz系统时钟
const PWM_CONTROLLERS: usize = 8;    // 8个PWM控制器
const CHANNELS_PER_CONTROLLER: usize = 2; // 每个控制器2个通道

// 全局使能寄存器地址（文档5.24.1.4）
const GLOBAL_ENABLE_REG_ADDR: usize = 0x2807E020;

pub struct PwmConfig {
    pub frequency: u32,      // PWM频率(Hz)
    pub duty_cycle: f32,     // 占空比(0.0-1.0)
    pub counting_mode: TIM_CTRL::MODE::Value,
    pub deadtime_ns: Option<u32>, // 死区时间(纳秒)
    pub use_fifo: bool,           // 是否使用FIFO模式
    pub output_behavior: PWM_CTRL::CMP::Value, // 输出行为
    pub initial_value: PWM_CTRL::ICOV::Value, // 初始输出值
}

pub struct PwmChannel {
    config: Option<PwmConfig>,
    enabled: bool,
}

pub struct PwmController {
    base: usize,
    channels: [PwmChannel; CHANNELS_PER_CONTROLLER],
}

pub struct PwmSystem {
    controllers: [PwmController; PWM_CONTROLLERS],
}

impl PwmController {
    pub unsafe fn new(base_addr: usize) -> Self {
        Self {
            base: base_addr,
            channels: [
                PwmChannel { config: None, enabled: false },
                PwmChannel { config: None, enabled: false }
            ],
        }
    }

    // 获取寄存器视图
    fn registers(&self) -> &PwmRegisters {
        unsafe { &*(self.base as *const PwmRegisters) }
    }

    // 配置PWM通道
    pub fn configure_channel(
        &mut self,
        channel: usize,
        config: PwmConfig,
    ) -> Result<(), &'static str> {
        if channel >= CHANNELS_PER_CONTROLLER {
            return Err("Invalid channel number");
        }
        
        if config.duty_cycle > 1.0 || config.duty_cycle < 0.0 {
            return Err("Duty cycle must be between 0.0 and 1.0");
        }
        
        // 禁用通道后再配置
        self.disable_channel(channel);
        
        // 计算分频和周期值
        let div = (SYSTEM_CLK / config.frequency) as u16; // 初始分频值
        let period_cycles = (SYSTEM_CLK as f32 / (div as f32 * config.frequency as f32)) as u32;
        
        // 实际写入的周期值 = 计算值 - 1 (看文档5.24.3.6中对CCR寄存器的描述)
        let period_reg = period_cycles.checked_sub(1).ok_or("Period too small")?;
        if period_reg > 0xFFFF {
            return Err("Period value too large");
        }
        
        let duty_cycles = (period_reg as f32 * config.duty_cycle) as u16;
        
        // 写入寄存器
        let regs = self.registers();
        
        // 配置死区 (共享配置)
        if let Some(deadtime) = config.deadtime_ns {
            let delay_cycles = (deadtime as f32 * SYSTEM_CLK as f32 / 1e9) as u16;
            // 确保不超过10位 (0-1023)
            let delay_cycles = delay_cycles.min((1 << 10) - 1);
            
            regs.dbdly.write(
                DBDLY::DBRED.val(delay_cycles) +
                DBDLY::DBFED.val(delay_cycles)
            );
            
            regs.dbctrl.write(
                DBCTRL::OUT_MODE::FullDeadband + // 双边死区
                DBCTRL::IN_MODE::PWM0 +         // 使用PWM0作为输入
                DBCTRL::POLSEL::AH              // 不翻转
            );
        }
        
        // 获取通道寄存器
        let ch_reg = self.get_channel_reg(channel);
        
        // 1. 配置TIM_CTRL (但不使能)
        ch_reg.tim_ctrl.write(
            TIM_CTRL::DIV.val(div.into()) +       // 分频系数
            TIM_CTRL::MODE.val(config.counting_mode) +
            TIM_CTRL::ENABLE::Disabled
        );
        
        // 2. 设置PWM_PERIOD (周期值)
        ch_reg.pwm_period.write(PWM_PERIOD::CCR.val(period_reg as u16));
        
        // 3. 配置PWM_CTRL
        ch_reg.pwm_ctrl.write(
            PWM_CTRL::MODE::Compare +              // 比较模式
            PWM_CTRL::DUTY_SEL.val(config.use_fifo as u32) +
            PWM_CTRL::ICOV.val(config.initial_value) +
            PWM_CTRL::CMP.val(config.output_behavior) +
            PWM_CTRL::IE::SET                     // 使能比较中断
        );
        
        // 4. 设置占空比
        if config.use_fifo {
            // FIFO模式：预填充4个值（避免空）
            for _ in 0..4 {
                ch_reg.pwm_ccr.write(PWM_CCR::CCR.val(duty_cycles));
            }
            // 启用FIFO空中断
            ch_reg.pwm_ctrl.modify(PWM_CTRL::FIFO_EMPTY_ENABLE::SET);
        } else {
            // 寄存器模式
            ch_reg.pwm_ccr.write(PWM_CCR::CCR.val(duty_cycles));
        }
        
        // 保存配置
        self.channels[channel].config = Some(config);
        
        Ok(())
    }
    
    // 启用通道输出
    pub fn enable_channel(&mut self, channel: usize) -> Result<(), &'static str> {
        if channel >= CHANNELS_PER_CONTROLLER {
            return Err("Invalid channel number");
        }
        
        let ch_reg = self.get_channel_reg(channel);
        ch_reg.tim_ctrl.modify(TIM_CTRL::ENABLE::SET);
        self.channels[channel].enabled = true;
        Ok(())
    }
    
    // 禁用通道
    pub fn disable_channel(&mut self, channel: usize) {
        let ch_reg = self.get_channel_reg(channel);
        ch_reg.tim_ctrl.modify(TIM_CTRL::ENABLE::CLEAR);
        self.channels[channel].enabled = false;
    }
    
    // 安全停止通道（完成当前周期）
    pub fn safe_stop_channel(&mut self, channel: usize) -> Result<(), &'static str> {
        let ch_reg = self.get_channel_reg(channel);
        
        // 1. 清零占空比
        ch_reg.pwm_ccr.write(PWM_CCR::CCR.val(0));
        
        // 2. 等待当前周期完成
        while ch_reg.tim_cnt.read(TIM_CNT::CNT) != 0 {
            // 实际实现中应添加超时和中断处理
            cortex_m::asm::nop();
        }
        
        // 3. 禁用通道
        self.disable_channel(channel);
        
        Ok(())
    }
    
    // 更新FIFO数据
    pub fn push_fifo_data(
        &mut self,
        channel: usize,
        duty_value: u16,
    ) -> Result<(), &'static str> {
        if channel >= CHANNELS_PER_CONTROLLER {
            return Err("Invalid channel number");
        }
        
        let ch_reg = self.get_channel_reg(channel);
        
        // 检查FIFO是否满（文档5.24.1.7）
        if ch_reg.state.matches_all(STATE::FIFO_FULL::SET) {
            return Err("FIFO full");
        }
        
        ch_reg.pwm_ccr.write(PWM_CCR::CCR.val(duty_value));
        Ok(())
    }
    
    // 处理中断（每个控制器单独调用）
    pub fn handle_interrupt(&mut self) {
        for channel in 0..CHANNELS_PER_CONTROLLER {
            if let Err(e) = self.handle_channel_interrupt(channel) {
                // 实际实现中应使用日志系统
                // log::error!("PWM ch{} error: {}", channel, e);
            }
        }
    }
    
    fn handle_channel_interrupt(&mut self, channel: usize) -> Result<(), &'static str> {
        let ch_reg = self.get_channel_reg(channel);
        let state = ch_reg.state.get();
        
        // 处理FIFO空中断 (RW1c清除方式)
        if state & STATE::FIFO_EMPTY.mask != 0 {
            // 仅在计数器为0时处理（按照软件设计文档要求）
            if ch_reg.tim_cnt.read(TIM_CNT::CNT) == 0 {
                if let Some(config) = &self.channels[channel].config {
                    let period = ch_reg.pwm_period.read(PWM_PERIOD::CCR) + 1;
                    let duty_cycles = (period as f32 * config.duty_cycle) as u16;
                    self.push_fifo_data(channel, duty_cycles)?;
                }
            }
            // RW1c清除标志：写1清除
            ch_reg.state.write(STATE::FIFO_EMPTY::SET);
        }
        
        // 处理计数器溢出中断
        if state & STATE::OVFIF.mask != 0 {
            // RW1c清除标志
            ch_reg.state.write(STATE::OVFIF::SET);
        }
        
        // 处理比较匹配中断
        if state & STATE::CHIF.mask != 0 {
            // RW1c清除标志
            ch_reg.state.write(STATE::CHIF::SET);
        }
        
        Ok(())
    }
    
    // 获取通道寄存器视图
    fn get_channel_reg(&self, channel: usize) -> ChannelRegisters {
        let regs = self.registers();
        match channel {
            0 => ChannelRegisters {
                tim_cnt: &regs.ch0_tim_cnt,
                tim_ctrl: &regs.ch0_tim_ctrl,
                state: &regs.ch0_state,
                pwm_period: &regs.ch0_pwm_period,
                pwm_ctrl: &regs.ch0_pwm_ctrl,
                pwm_ccr: &regs.ch0_pwm_ccr,
            },
            1 => ChannelRegisters {
                tim_cnt: &regs.ch1_tim_cnt,
                tim_ctrl: &regs.ch1_tim_ctrl,
                state: &regs.ch1_state,
                pwm_period: &regs.ch1_pwm_period,
                pwm_ctrl: &regs.ch1_pwm_ctrl,
                pwm_ccr: &regs.ch1_pwm_ccr,
            },
            _ => unreachable!(),
        }
    }
}

// 通道寄存器视图
struct ChannelRegisters<'a> {
    tim_cnt: &'a ReadWrite<u32, TIM_CNT::Register>,
    tim_ctrl: &'a ReadWrite<u32, TIM_CTRL::Register>,
    state: &'a ReadWrite<u32, STATE::Register>,
    pwm_period: &'a ReadWrite<u32, PWM_PERIOD::Register>,
    pwm_ctrl: &'a ReadWrite<u32, PWM_CTRL::Register>,
    pwm_ccr: &'a ReadWrite<u32, PWM_CCR::Register>,
}

impl PwmSystem {
    // 初始化所有PWM控制器
    pub fn new() -> Self {
        // PWM控制器基地址（文档表5-67）
        const CONTROLLER_BASES: [usize; PWM_CONTROLLERS] = [
            0x2804_A000, // PWM0
            0x2804_B000, // PWM1
            0x2804_C000, // PWM2
            0x2804_D000, // PWM3
            0x2804_E000, // PWM4
            0x2804_F000, // PWM5
            0x2805_0000, // PWM6
            0x2805_1000, // PWM7
        ];
        
        let controllers = CONTROLLER_BASES.map(|base| unsafe {
            PwmController::new(base)
        });
        
        Self { controllers }
    }
    
    // 全局使能所有已配置的PWM控制器
    pub fn global_enable(&self) {
        let mut enable_mask: u32 = 0;
        
        for (i, ctrl) in self.controllers.iter().enumerate() {
            // 只要控制器任一通道配置了，就置位对应的全局使能位
            if ctrl.channels.iter().any(|ch| ch.config.is_some()) {
                enable_mask |= 1 << i;
            }
        }
        
        // 写入全局使能寄存器（文档0x2807E020）
        unsafe {
            let reg_ptr = GLOBAL_ENABLE_REG_ADDR as *mut u32;
            reg_ptr.write_volatile(enable_mask);
        }
    }
    
    // 获取控制器（按索引）
    pub fn controller(&mut self, index: usize) -> Option<&mut PwmController> {
        if index < PWM_CONTROLLERS {
            Some(&mut self.controllers[index])
        } else {
            None
        }
    }
}

// 示例用法
fn example_usage() {
    let mut pwm_system = PwmSystem::new();
    
    // 配置PWM0控制器的通道0
    if let Some(ctrl0) = pwm_system.controller(0) {
        let config = PwmConfig {
            frequency: 1000,  // 1KHz
            duty_cycle: 0.5,  // 50%占空比
            counting_mode: TIM_CTRL::MODE::Modulo,
            deadtime_ns: Some(100), // 100ns死区
            use_fifo: false,
            output_behavior: PWM_CTRL::CMP::ClearOnMatch,
            initial_value: PWM_CTRL::ICOV::CLEAR,
        };
        
        ctrl0.configure_channel(0, config).unwrap();
        ctrl0.enable_channel(0).unwrap();
    }
    
    // 启用所有已配置的PWM控制器
    pwm_system.global_enable();
}

// 中断处理函数示例
fn pwm_interrupt_handler(controller_idx: usize) {
    if let Some(ctrl) = pwm_system.controller(controller_idx) {
        ctrl.handle_interrupt();
    }
}