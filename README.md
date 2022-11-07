# Rust6502

In order to learn Rust, I've been working on a 6502 emulator.
Currently it works well enough to run WozMon, but that's about it.

## Features

* Emulates 6502 and 65c02 op codes
* Includes code for WozMon and Apple BASIC
* Includes a crate that provides basic terminal emulation
* When launched, it starts WozMon at FF00
* You can launch Apple BASIC (which doesn't work) by entering E000R
* ESC or other keys such as cursor keys will stop it running


## Limitations

* Only tested on Apple macOS
* Apple BASIC starts, but won't display anything other than error messages
* Doesn't support SBC in Decimal mode just yet
* I've tested the opcodes a lot, but there's probably an error or ten still there.


## Instructions

* Install Rust on your system
* Download the repo
* You should be able to run it with ```cargo run``` (it'll download a few other things the first time it's launched)
* Enter something like ```00.FF``` to see that WozMon is running
* Press ESC to stop

