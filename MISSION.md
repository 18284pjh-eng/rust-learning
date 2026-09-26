# Mission: Rust Linux User-Space for Embedded Systems

## Why
Learn Rust systematically and use it to replace embedded C user-space modules for SPI and UART on a PYNQ-Z2 running Debian 13. Establish the board system from a TF card first, then extend the application so the board can send peripheral data to a PC over Linux TCP sockets.

## Success looks like
- Build and deploy an ARMv7 hard-float Linux Rust program from the AMD64 Debian development machine.
- Implement and test small Rust user-space SPI and UART modules that use the board's existing Linux device interfaces.
- Explain how ownership and `Result` help manage buffers, device handles, and expected I/O failures.
- Send framed peripheral data from a PYNQ TCP client to a PC server.

## Constraints
- The learner is experienced with embedded C and has read some Rust, but has not studied it systematically.
- Focus on ownership and error handling; use SPI/UART protocol knowledge as context rather than reteaching the protocols from the beginning.
- Lessons are self-paced, one lesson at a time, and about 2-3 hours each.
- Develop on Debian 13 AMD64 and deploy to Debian 13 armhf on the PYNQ-Z2. Confirm the board architecture before choosing a Rust target.
- Use a spare TF card to validate the PYNQ-Z2 boot chain, then build the Debian 13 armhf user space around the board's compatible kernel and device tree.
- Begin with small self-contained modules. Ask for the actual C source and tests when the learner is ready to begin that migration.
- Prefer Rust user-space libraries for SPI/UART and explain their relationship to Linux interfaces.

## Out of scope
- Linux kernel driver and device-tree development in the main course.
- Bare-metal or `no_std` Rust.
- Implementing a custom TCP/IP stack.
- FPGA RTL development as a required part of the Rust course. Hardware test arrangements can be adapted later.
