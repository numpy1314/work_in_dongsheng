# 学期报告
本报告为2025年5月26号进组学习到7月31号的学习报告，以训练营结束为界限划分六月和七月两部分进行说明

## 6月
第一个月以学习为主
### os训练营部分
- 完成了一些文档上和仓库下游合并的工作，主要是借这些工作进行学习
  - 文档上是allocator这个crate的介绍
  - 下游合并其一是文件系统的一些性能优化的工作
    - [fix] incorrect writev implementation：https://github.com/arceos-org/arceos/pull/267
    - [fix] sys_writev return value：https://github.com/arceos-org/arceos/pull/260
    - Upgrade axio to 0.1.1 and optimize default_read_to_end：https://github.com/arceos-org/arceos/pull/268
    - 下游合并其二是虚拟化的一些前置工作
    - [feat] functions essential to hypervisor implementation：https://github.com/arceos-org/axcpu/pull/10
### docker训练营部分
- 帮助腾讯云团队设计了相关基础阶段的课程内容 https://cnb.cool/opencamp/learning-docker/docker-exercises
### 飞腾派驱动训练营部分
- 写了第一周实验 https://github.com/numpy1314/arceos/commit/9a273aa71fc1eb9466f4673c8810185670892b57
但后续没有时间继续
- 手册编写部分完成了分配的任务（预备知识部分）https://github.com/chenlongos/Phytium-Pi-Driver/blob/main/src/chapter0/0_3_prerequisites.md

## 7月
第二个月os训练营结束，os相关转为虚拟化部分的学习
### 虚拟化（新主线的arm架构虚拟化的补充）
- https://github.com/arceos-org/axcpu/pull/13
（未审核）
- https://github.com/arceos-org/axplat_crates/pull/18
（未审核）
- https://github.com/arceos-org/axplat_crates/pull/20（3588的适配修改、未审核）
### docker训练营
- 担任项目三导师，https://cnb.cool/opencamp/learning-docker/project-3-os ，目前接近完成，还剩下一部分ci安全性问题，为防止学员修改成绩提交ci，目前采用采用把 ci 测评部分和 ci 环境都做成镜像的方法，统一放在一个仓库里，在子项目中直接使用 import 导入
### 飞腾派驱动训练营
- 编写了飞腾派上pwm驱动，完成了驱动指导手册的撰写 https://github.com/chenlongos/Phytium-Pi-Driver/blob/main/src/chapter1/1_2_pwm_driver.md
### 国家电网相关课题
- 整理资料 https://docs.qq.com/desktop/mydoc/folder/YdUpBYTsLVzp