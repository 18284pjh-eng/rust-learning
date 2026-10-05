# 嵌入式 Rust Linux 用户态课程

面向有嵌入式 C 经验、希望系统学习 Rust 的工程师。课程以 PYNQ-Z2 为最终部署目标，重点讲清所有权、借用和错误处理，并逐步练习 Linux 用户态外设访问与 TCP 通信。

## 学习环境

- 开发机：Debian 13 AMD64
- 目标板：运行官方 PYNQ 镜像的 PYNQ-Z2；第 0 课会先验证架构和 Linux 接口
- 外设：复用系统已有设备树和 Linux 驱动，通过用户态接口访问 SPI、UART
- 网络：先使用 Linux TCP socket
- 节奏：自定进度，每课约 2–3 小时

## 从这里开始

1. 打开 [`index.html`](index.html) 查看课程路径。
2. 先学习 [第 0 课：从 TF 卡启动官方 PYNQ-Z2 镜像](lessons/0000-pynq-z2-debian13.html)。
3. 再学习 [第 1 课：从 AMD64 构建并运行 ARM Linux 程序](lessons/0001-host-to-arm64.html)。
4. 开始[第 2 课：Rust 表达式、类型与控制流](lessons/0002-rust-expressions-types-control-flow.html)。
5. 查阅[官方镜像系统搭建速查](reference/pynq-z2-debian13.html)、[交叉编译速查](reference/arm64-cross-compile.html)、[Rust 语法速查](reference/rust-expressions-control-flow.html)和[学习资源](RESOURCES.md)。

课程目标与范围见 [MISSION.md](MISSION.md)，术语约定见 [CONTEXT.md](CONTEXT.md)。

## 目录

- `lessons/`：逐课学习材料与任务
- `reference/`：开发、交叉编译和部署速查
- `learning-records/`：学习起点与进度记录
- `assets/`：课程页面共用样式
- `MISSION.md`：学习目标、成功标准和范围
- `RESOURCES.md`：官方文档和参考资料

目前内容从独立的小型练习开始；进入真实 SPI 或 UART 模块迁移时，再根据具体 C 源码和测试调整课程任务。
