# Learning Record: PYNQ-Z2 系统架构与官方镜像路线

## 结论

- PYNQ-Z2 使用 Zynq Z7020，PS 为双核 Cortex-A9，属于 32 位 ARMv7 平台。
- 官方镜像的发行版、用户空间架构和 C 库以板上输出为准；若报告 ARMv7 hard-float，常用 Rust Linux 目标是 `armv7-unknown-linux-gnueabihf`。
- TF 卡启动不是只有一个根文件系统：还需要板级 `BOOT.BIN`/FSBL、U-Boot、Linux 内核和设备树。
- 当前主线直接使用官方 PYNQ-Z2 镜像开发，先把 Rust、SPI、UART 和 TCP 用户态模块跑通。
- Debian 13 保留为后续独立迁移实验，等应用模块稳定后再评估。

## 证据

- PYNQ 官方板卡页将 PYNQ-Z2 列为 Zynq Z7020，并将 Zynq-7000 的预构建根文件系统标为 `arm`。
- PYNQ 官方 SD 卡构建文档说明构建流程会生成 `BOOT.bin`、U-Boot、设备树、内核和根文件系统。
- PYNQ 官方镜像文档说明镜像包含板级启动文件、U-Boot、设备树、内核和 PYNQ 根文件系统。

## 对课程的影响

第 0 课先完成官方镜像、TF 卡、串口、网络和 SPI/UART 节点验收；第 1 课根据板上实际输出决定 Rust target。真实 SPI/UART 迁移前，保留官方镜像文件和完整串口日志。

## 待确认

在板上记录 `cat /etc/os-release`、`uname -m`、`dpkg --print-architecture`、`cat /proc/device-tree/model`、内核版本以及 `/dev/ttyPS*`、`/dev/spidev*` 是否存在。若架构或 C 库与预期不同，暂停默认 Rust target 并核对系统来源。

## 第 1 课验证结果

- 主机原生产物：ELF 64 位 x86-64，动态链接到 `/lib64/ld-linux-x86-64.so.2`。
- 交叉产物：ELF 32 位 ARM EABI5，动态链接到 `/lib/ld-linux-armhf.so.3`。
- 板上运行：`target_probe` 正常输出 Hello World。
- 概念确认：`host` 是编译发生的平台，`target` 是程序准备运行的平台；两者架构不同就需要交叉编译。

## Cargo 默认目标

项目可以在 `.cargo/config.toml` 中固定默认目标，避免每次重复写 `--target`：

```toml
[build]
target = "armv7-unknown-linux-gnueabihf"

[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```

这样 `cargo build --release` 默认生成板上程序；主机调试时用 `cargo run --target x86_64-unknown-linux-gnu` 临时覆盖目标。
