# am2320

A platform-agnostic driver to interface with the AM2320 I2c temperature & humidity sensor.

## Examples

You can find an example to use the sensor with a RaspberryPI under `examples/`.

Build for Raspberry Pi Zero using [cross](https://github.com/cross-rs/cross) with

```
$ cross build --target=arm-unknown-linux-musleabihf --example=print
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
