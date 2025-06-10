首先要明白一件事：dora的工作流程

![](pic/dora-workflow.png)

这个项目的工作核心就是数据流的构建，在yaml文件中定义；除此之外，yaml还定义nodes，包括他们的输入和输出，以及节点之间的连接，其实和arceos一定程度上是类似的。

启动后，dora-cli会发送一个start数据流给dora-coordinator,然后coordinator会根据yaml文件中的配置来构建数据流，再发送给dora-daemon，再由daemon启动node进行执行
![](pic/entire-process.png)