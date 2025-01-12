https://developer.arm.com/documentation/ddi0183/g/introduction/about-the-uart

Advanced Microcontroller Bus Architecture (AMBA) slave module that connects to the Advanced Peripheral Bus (APB)
includes an Infrared Data Association (IrDA) Serial InfraRed (SIR) protocol ENcoder/DECoder (ENDEC).

The UART provides:
    Compliance to the AMBA Specification (Rev 2.0) onwards for easy integration into SoC implementation.
    Programmable use of UART or IrDA SIR input/output.
    Separate 32×8 transmit and 32×12 receive First-In, First-Out (FIFO) memory buffers to reduce CPU interrupts.
    Programmable FIFO disabling for 1-byte depth.
    Programmable baud rate generator. This enables division of the reference clock by (1×16) to (65535×16) and generates an internal ×16 clock. The divisor can be a fractional number enabling you to use any clock with a frequency >3.6864MHz as the reference clock.
    Standard asynchronous communication bits (start, stop and parity). These are added prior to transmission and removed on reception.
    Independent masking of transmit FIFO, receive FIFO, receive timeout, modem status, and error condition interrupts.
    Support for Direct Memory Access (DMA).
    False start bit detection.
    Line break generation and detection.
    Support of the modem control functions CTS, DCD, DSR, RTS, DTR, and RI.
    Programmable hardware flow control.
    Fully-programmable serial interface characteristics:
        data can be 5, 6, 7, or 8 bits
        even, odd, stick, or no-parity bit generation and detection
        1 or 2 stop bit generation
        baud rate generation, dc up to UARTCLK/16
    IrDA SIR ENDEC block providing:
        programmable use of IrDA SIR or UART input/output
        support of IrDA SIR ENDEC functions for data rates up to 115200 bps half-duplex
        support of normal 3/16 and low-power (1.41 - 2.23µs) bit durations
        programmable division of the UARTCLK reference clock to generate the appropriate bit duration for low-power IrDA mode.
    Identification registers that uniquely identify the UART. These can be used by an operating system to automatically configure itself.
