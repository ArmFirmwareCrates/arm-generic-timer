# Arm Generic Timer driver

Driver implementation for the memory mapped Generic Timer peripheral of the Arm A-profile
architecture. The implementation is based on the following sections of the
[Arm Architecture Reference Manual for A-profile architecture](https://developer.arm.com/documentation/ddi0487/la/).

* I2.2.3 Counter module control and status register summary
* I2.3 Memory-mapped timer components
* I5.6 Generic Timer memory-mapped registers overview
* I5.7 Generic Timer memory-mapped register descriptions

## Implemented features

* Register descriptions and drivers for the following frames:
  * `CNTControlBase`
  * `CNTCTLBase`
  * `CNTReadBase`
  * `CNTBaseN`
  * `CNTEL0BaseN`
* Blocking and interrupt based timer wait functions.

## License

The project is MIT and Apache-2.0 dual licensed, see `LICENSE-APACHE` and `LICENSE-MIT`.

## Maintainers

arm-generic-timer is a trustedfirmware.org maintained project. All contributions are ultimately merged by the
maintainers listed below.

* Bálint Dobszay <balint.dobszay@arm.com>
  [balint-dobszay-arm](https://github.com/balint-dobszay-arm)
* Imre Kis <imre.kis@arm.com>
  [imre-kis-arm](https://github.com/imre-kis-arm)
* Sandrine Afsa <sandrine.afsa@arm.com>
  [sandrine-bailleux-arm](https://github.com/sandrine-bailleux-arm)

## Contributing

Please follow the directions of the [Trusted Firmware Processes](https://trusted-firmware-docs.readthedocs.io/en/latest/generic_processes/index.html)

Contributions are handled through [review.trustedfirmware.org](https://review.trustedfirmware.org/q/project:arm-firmware-crates/arm-generic-timer).

## Arm trademark notice

Arm is a registered trademark of Arm Limited (or its subsidiaries or affiliates).

This project uses some of the Arm product, service or technology trademarks, as listed in the
[Trademark List][1], in accordance with the [Arm Trademark Use Guidelines][2].

Subsequent uses of these trademarks throughout this repository do not need to be prefixed with the
Arm word trademark.

[1]: https://www.arm.com/company/policies/trademarks/arm-trademark-list
[2]: https://www.arm.com/company/policies/trademarks/guidelines-trademarks

--------------

*Copyright The arm-generic-timer Contributors.*
