# Learning Record: PYNQ-Z2 系统架构与 Debian 13 路线

## 结论

- PYNQ-Z2 使用 Zynq Z7020，PS 为双核 Cortex-A9，属于 32 位 ARMv7 平台。
- Debian 13 的默认用户空间架构应按 `armhf` 验证；常用 Rust Linux 目标是 `armv7-unknown-linux-gnueabihf`。
- TF 卡启动不是只有一个根文件系统：还需要板级 `BOOT.BIN`/FSBL、U-Boot、Linux 内核和设备树。
- 先用官方 PYNQ-Z2 镜像验证启动链，再建立 Debian 13 根文件系统，可以把板级启动问题和用户空间问题分开。

## 证据

- PYNQ 官方板卡页将 PYNQ-Z2 列为 Zynq Z7020，并将 Zynq-7000 的预构建根文件系统标为 `arm`。
- PYNQ 官方 SD 卡构建文档说明构建流程会生成 `BOOT.bin`、U-Boot、设备树、内核和根文件系统。
- Debian armhf 启动文档说明 U-Boot 启动 ARM Linux 时还需要平台匹配的 kernel、initrd 和 DTB。

## 对课程的影响

第 0 课先完成 TF 卡、串口和 Debian 13 验收；第 1 课根据板上实际输出决定 Rust target。真实 SPI/UART 迁移前，保留官方启动分区备份和完整串口日志。

## 待确认

在板上记录 `uname -m`、`dpkg --print-architecture`、`cat /proc/device-tree/model`、内核版本、根分区来源以及 `/dev/ttyPS*`、`/dev/spidev*` 是否存在。若报告 `aarch64`，暂停默认 ARMv7 路线并核对系统来源。
