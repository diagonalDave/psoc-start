cargo build --target=thumbv6m-none-eabi --release
let v6m_size = ((cargo size --lib --target=thumbv6m-none-eabi --release) | detect columns | get dec | into int | math sum)
echo $'Built size thumbv6m: ($v6m_size)'
cd target\thumbv6m-none-eabi\release
rust-objcopy --weaken-symbol=__aeabi_dcmplt --weaken-symbol=__aeabi_dcmpge --weaken-symbol=__aeabi_dcmpeq libstart.a
cd ..\..\..\
cargo build --target=thumbv7em-none-eabi --release
let v7em = ((cargo size --lib --target=thumbv7em-none-eabi --release) | detect columns | get dec | into int | math sum)
echo  $'Built size thumbv7em: ($v7em)'
cd target\thumbv7em-none-eabi\release
rust-objcopy --weaken-symbol=__aeabi_dcmplt --weaken-symbol=__aeabi_dcmpge --weaken-symbol=__aeabi_dcmpeq libstart.a
cd ..\..\..\
