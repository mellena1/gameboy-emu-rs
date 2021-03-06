# gameboy-emu-rs
[![Build Status](https://ci.andrewmellen.org/api/badges/mellena1/gameboy-emu-rs/status.svg)](https://ci.andrewmellen.org/mellena1/gameboy-emu-rs)
[![codecov](https://codecov.io/gh/mellena1/gameboy-emu-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/mellena1/gameboy-emu-rs)



A Gameboy emulator written in Rust. The goal of this project isn't to write the most feature rich emulator or even the most accurate emulator. This is exclusively a learning exercise for myself to learn more about emulation and the Rust language.

## Goals/Milestones
(These will be updated as I learn more about the internals of the Gameboy)

- [ ] Implement all CPU instructions
- [ ] Implement GPU
- [ ] Pass [Blargg's hardware test ROMs](https://github.com/retrio/gb-test-roms)
- [ ] Implement sound
- [ ] Implement a simple desktop GUI
- [ ] Successfully boot the boot ROM
- [ ] Implement passing actions/controls to the emulator
- [ ] Play a game
- [ ] Compile to WASM
- [ ] Add GBC support?

## References/Thanks
- [Ryan Levick's guide](https://blog.ryanlevick.com/DMG-01/public/book/introduction.html) is a great jumping off point
- [pandocs](http://bgb.bircd.org/pandocs.htm#cpuregistersandflags)
- [Game Boy CPU Manual](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf)
