# `psoc_start`

The psoc_start project is intended as a test bed for developing the ([psoc6-hal](https://github.com/diagonalDave/psoc6-hal))

## Intro
This repo comprises a rust library and a minimum psoc-creator project configured for the CY8CPROTO-063-BLE.

The rust library, called start, implements two functions called start_cm0() and start_cm4(). These functions implement peripheral access and generally are equivalent to main functions.
The psoc-creator project provides the startup code for the PSOC6. The psoc-creator project has been exported as a make project so should be buildable on OSX amd Linux.

The projects uses [nushell](https://www.nushell.sh) scripts to simplify some of the tasks in building. Nushell is written in Rust and can be installed with cargo, see the ([nushell](https://www.nushell.sh)) link, or crates.io for details.
## Usage
To use this repo:
1. Ensure you have a working [Rust environment](https://docs.rust-embedded.org/book/intro/install.html) with the thumbv6m-none-eabi and thumbv7em-none-eabi targets installed. The code is built to target the thumbv7em-none-eabi rather than the hardware fpu variant thumbv7em-none-eabihf because psoc-creator only supports software fpu for some reason.
2. Clone the repo.
`git clone https://github.com/diagonalDave/psoc-start.git`
3. Navigate to the start directory then run both of the nushell scripts to kick off the library build:
` nu build_release.nu
  nu build_debug.nu` 
4. Open the psoc-creator project then **clean and build** the project with either debug or release profile. 
   -  Download to the ble board and watch the leds blink.
5. Alternatively build the project using the makefile in the minimum_rust_start.cydsn folder, then download it to the device using [openocd] (https://github.com/Infineon/openocd/releases).

## A brief introduction to all the moving parts
In its current state this project requires the following things to work:
1. The [psoc6-hal](https://github.com/diagonalDave/psoc6-hal) building the Rust code should provide this.
   - The hal comprises:
     1. The [psoc6-pac](https://github.com/diagonalDave/psoc6-pac) - register level access to the psoc.
     2. Drivers that build on the psoc6-pac functionality, intended to make it easier and safer to use Rust on the psoc6.
     3. The actual hal, a high level API abstracting away 'making things work details'.
2. The Rust code:
   - The code written in Rust to solve your problem. See the start folder for the Rust project.
   - The code comprises a "C" header (start.h) file that defines two functions used in the c code of the psoc-creator project.
3. The psoc-creator project:
   - The c code that provides all the psoc6 functionality and calls the Rust code.
   - The Rust project builds as a lib to directories that have been pre-configured in the psoc-creator profiles.
   - ATM the psoc-creator project requires a **clean and build** each time changes are made to the Rust code.
4. Nu shell:
   - [Nu shell](https://www.nushell.sh) is a Rust shell that, like Rust, is cross platform and awesome.
      - use `cargo install nu` or see [nu instructions](https://www.nushell.sh/book/installation.html#package-managers) for details.

## Purpose
The main purpose of this repo is to provide a test environment for porting microcontroller code to Rust. Given this purpose it is unlikely to ever be anything other than a hacked together project.
On the other hand it does show a multicore microcontroller running Rust code on both cores, with the ability to develop end-to-end functionality and see it run.
The intention is to port start-up code to Rust until psoc-creator is no longer required.

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
