# Rust GB Emulator

This project is a Game Boy emulator written in Rust. The emulator is designed to run Game Boy ROMs, specifically those that are ROM-only and use MBC5 without rumble.

## Getting Started

To run the emulator, follow these steps:

1. Make sure you have Rust installed. If not, you can install it by following the instructions on [the official Rust website](https://www.rust-lang.org/).

2. Clone this repository to your local machine.

3. Navigate to the project directory.

4. Build and run the emulator with a specific ROM:

    ```bash
    cargo run 'path/to/rom.gb'
    ```

    Replace `'path/to/rom.gb'` with the actual path to the Game Boy ROM you want to run.

## ROM Compatibility

The emulator currently supports ROMs that are:

- **ROM-only** (Tetris for exemple)
- **MBC5** (Pokemon Red for exemple)
- Do not have rumble features

Please make sure your ROM meets these criteria for compatibility. Also a first version of save is present in this projet for MBC with a battery and a ram.

## Controls

The emulator uses a standard set of controls:

- **Arrow keys:** D-pad controls
- **Q:** A button
- **S:** B button
- **X:** Start button
- **W:** Select button

## Annex

For more information about the Game Boy architecture and opcodes, refer to the following documentation:

- [Game Boy Pandocs](https://gbdev.io/pandocs/)
- [Game Boy Opcodes](https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
- [DMG-01 Technical Reference](https://rylev.github.io/DMG-01/public/book/)
- [Game Boy CPU Manual (PDF)](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf)

---

Happy gaming! 🎮
