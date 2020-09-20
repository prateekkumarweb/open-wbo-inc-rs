# open-wbo-inc-rs

Rust bindings for Open-WBO-Inc

## How to

```sh
cd Open-WBO-Inc
sed -i 's/main(/main2(/g' Main.cc
LIB=open-wbo-inc make libr
cd ..
cargo build
```
