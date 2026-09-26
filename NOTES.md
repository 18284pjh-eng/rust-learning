# Teaching Notes

- Communicate in Chinese and keep the learner in control of the pace.
- Each lesson should take about 2-3 hours; do not assign a weekly schedule.
- Begin with small self-contained Rust modules. Request the concrete C source, interfaces, and tests only when the learner is ready to start the real migration.
- Prioritize ownership and error handling, and use the learner's embedded C experience as a bridge.
- Use Rust user-space libraries for SPI/UART exercises, with a concise explanation of the Linux interfaces beneath them.
- Keep the main course focused on Rust. Use the Wildfire Zhengtu Pro board and FPGA peer as adaptable test equipment rather than adding required FPGA RTL lessons.
- Initial TCP topology: the PYNQ application connects to a server on the PC.
