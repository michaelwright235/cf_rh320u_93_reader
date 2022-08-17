# cf\_rh320u\_93\_reader

Rust library for interaction with Chafon CF-RH320U-93 USB card readers. It's an open source implementation of the official library, which can be found on their website. 

## Features

- Read and write ISO 15693 cards
- Control buzzer and LED
- Set and get internal serial number
- And other commands

The main goal is to support all ISO 15693 related commands in the future. 

## Usage

All commands are performed by calling a single function inside this crate.

```rust
use cf_rh320u_93_reader::*;
let inventory = iso15693_inventory().unwrap();
for card in inventory {
    println!("{:?}", card);
}
```

If something goes wrong, ReaderError enum is returned. It contains either a UsbError (which means that there was a connection error), or either a CommandError (a status code returned directly from a reader).
