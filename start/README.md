# `psoc_start`

The psoc_start project is intended as a test bed for developing the ([psoc6-hal](https://github.com/diagonalDave/psoc6-hal))

## Intro
This repo comprises a rust library and a minimum psoc-creator project configured for the CY8CPROTO-063-BLE.

The rust library, called start, implements two functions called start_cm0() and start_cm4(). These functions implement peripheral access and generally are equivalent to main functions.
The psoc-creator project provides the startup code for the PSOC6. The psoc-creator project has been exported as a make project so should be buildable on OSX amd Linux.

The projects uses ([nushell](https://www.nushell.sh)) scripts to simplify some of the tasks in building. Nushell is written in Rust and can be installed with cargo, see the ([nushell](https://www.nushell.sh)) link, or crates.io for details.
## Usage
To use this repo:
1. Ensure you have a working ([Rust environment](https://docs.rust-embedded.org/book/intro/install.html)) with the thumbv6m-none-eabi and thumbv7em-none-eabi targets installed.
2. Clone the repo.
`git clone https://github.com/diagonalDave/psoc-start.git`
3. Navigate to the start directory then run both of the nushell scripts to kcik off the library build:
` nu build_release.nu
  nu build_debug.nu `
4. Open the psoc-creator project then build the project with either debug or release profile.
   -  Download to the ble board and watch the leds blink.
5. Alternatively build the project using the makefile in the minimum_rust_start.cydsn folder, then download it to the device using ([openocd] (https://github.com/Infineon/openocd/releases)).

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
