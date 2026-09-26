# Embedded Rust Learning Context

This glossary defines the hardware and operating-system terms used throughout the PYNQ-Z2 Rust course.

## Runtime and Build

**Host**:
The AMD64 Debian 13 computer where Rust source code is edited and compiled.
_Avoid_: target machine, board (when referring to the development computer)

**Target**:
The ARM64 Debian 13 PYNQ-Z2 system where the compiled Rust application runs.
_Avoid_: host (for the board)

**Cross-compilation**:
Building a program on the host so that its executable runs on a different target architecture and operating system.
_Avoid_: remote compilation (unless compilation actually happens on the board)

**Linux user-space application**:
A normal Linux process that uses operating-system interfaces and device nodes while the Linux kernel continues to manage the hardware.
_Avoid_: kernel driver, firmware (when referring to this course's Rust executable)

## Hardware Interfaces

**Linux device node**:
A filesystem path, such as `/dev/spidev...` or `/dev/tty...`, through which a user-space process accesses a device exposed by a Linux driver.
_Avoid_: device-tree node (the device tree is a separate kernel configuration description)

**SPI test peer**:
A slave device that responds to transfers initiated by the PYNQ-Z2's Linux SPI controller.
_Avoid_: SPI master (the side that initiates the transfer)

## Network

**PYNQ client**:
The Rust application on the PYNQ-Z2 that initiates a TCP connection to the PC.
_Avoid_: TCP server (in the initial course topology)

**PC server**:
A TCP listener on the development computer that accepts the PYNQ client's connection and receives its data.
