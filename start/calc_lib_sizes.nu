 let v6m_size_r = ((cargo size --lib --target=thumbv6m-none-eabi --release) | detect columns | get dec | into int | math sum)

 let v6m_size = ((cargo size --lib --target=thumbv6m-none-eabi) | detect columns | get dec | into int | math sum)

let  v7em_r = ((cargo size --lib --target=thumbv7em-none-eabi --release) | detect columns | get dec | into int | math sum)

let v7em = ((cargo size --lib --target=thumbv7em-none-eabi) | detect columns | get dec | into int | math sum)

echo $'Built size of release thumbv6m: ($v6m_size_r)'
echo $'Built size of debug thumbv6m:   ($v6m_size)'
echo 
echo  $'Built size of release thumbv7em: ($v7em_r)'
echo  $'Built size of debug thumbv7em:   ($v7em)'  

