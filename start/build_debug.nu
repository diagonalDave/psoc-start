cargo build --target=thumbv6m-none-eabi
cd target\thumbv6m-none-eabi\debug
rust-objcopy --weaken-symbol=__aeabi_dcmplt --weaken-symbol=__aeabi_dcmpge --weaken-symbol=__aeabi_dcmpeq librustypsoc.a
cd ..\..\..\

cargo build --target=thumbv7em-none-eabi

cd target\thumbv7em-none-eabi\debug
rust-objcopy --weaken-symbol=__aeabi_dcmplt --weaken-symbol=__aeabi_dcmpge --weaken-symbol=__aeabi_dcmpeq librustypsoc.a
cd ..\..\..\
# cd ..\psoc\rusty\rustypsoc.cydsn\
# make
# cd ..\..\..\rustypsoc\


