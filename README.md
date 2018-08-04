# Rust AT24CXXX Driver

This is a platform agnostic Rust driver for the AT24CXXX serial EEPROM,
based on the [`embedded-hal`](https://github.com/japaric/embedded-hal) traits.

## The Device

Datasheet: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8568-SEEPROM-AT24C256C-Datasheet.pdf

## Status

- [x] Random address single byte read
- [ ] Current address read
- [ ] Sequential read
- [x] Byte write
- [ ] Page write

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
