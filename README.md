```
 ⡎⠑⢇⢸⢎⡱⡎⠑⡷⢾⣏⡱⣏⡱⢉⡹⢺ ⣎⣵⢎⡱ ⡏⢱⣏⡱⡇⡇⢸⣏⡉⣏⡱
 ⠣⠔ ⠇⠣⠜⠣⠔⠇⠸⠧⠜⠇⠱⠤⠜⠼⠄⠫⠜⠣⠜ ⠧⠜⠇⠱⠇⠸⠃⠧⠤⠇⠱
 Embedded HAL I2C driver for the CY8CMBR3108 Touch Controller
```

# CY8CMBR3108 Touch Controller Driver

Embedded HAL I2C driver for the [CY8CMBR3108 CapSense® Controller](docs/infineon-cy8cmbr3002-cy8cmbr3102-cy8cmbr3106s-cy8cmbr3108-cy8cmbr3110-cy8cmbr3116-datasheet-en.pdf).

The CY8CMBR3108 is a low-power, high-performance capacitive touch controller designed for portable applications that require precise touch sensing with minimal power consumption.

Featuring:
- Ultra-low-power operation with high performance
- Capacitive sensing technology for reliable touch detection
- Digital I/O for easy integration
- Configurable sensitivity and thresholds

> [!CAUTION]
> This project is actively being developed with frequent breaking changes. APIs may shift, features are incomplete, and stability is not guaranteed. Use at your own risk and expect regular updates that might require code adjustments. Have fun!

> [!IMPORTANT]
> **Hi Squeaky Things** can happen at any time. This driver let the [_Little Weirdo_](https://github.com/hi-squeaky-things/little-weirdo) squeak, squuuueak, squeeeeeaak, squeaaaaaaaaak!

## Examples

In the `examples` folder you'll find:

1. `touch_test.rs` - Basic example that initializes the touch controller and checks for touch events

## Features

- Device initialization and readiness check
- Family ID, device ID, and revision reading
- Configuration register access
- Touch detection capabilities
- Low-power operation support

## Usage

Add to your `Cargo.toml`:
```toml
[dependencies]
driver_cy8cmbR3108 = { git = "https://github.com/hi-squeaky-things/driver-cy8cmbR3108" }
```

## Configuration

The driver is configured for:
- Default I2C address: 0x37
- Standard touch detection parameters
- Automatic device identification

## Credits

- Inspired by various embedded touch controller drivers
- [Small Braille ASCII Font](https://patorjk.com/software/taag/#p=display&f=Small+Braille&t=LITTLE+WEIRDO&x=rainbow1&v=1&h=1&w=80&we=false)

## License

MIT