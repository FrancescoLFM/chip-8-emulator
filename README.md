# CHIP-8 Rust Emulator

![WIP](https://img.shields.io/badge/status-WIP-yellow)

A CHIP-8 emulator written in Rust. This is a work-in-progress project aimed at learning and demonstrating emulation concepts while exploring the Rust programming language.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)

## Overview

CHIP-8 is an interpreted programming language that was first implemented on the RCA COSMAC VIP microcomputer in the 1970s. It's popular among emulator developers due to its simplicity and is often used as a beginner's project for learning about emulation.

This project is an attempt to create a functional CHIP-8 emulator using Rust. The emulator is designed to read and execute CHIP-8 ROMs, simulating the behavior of the original CHIP-8 system.

## Features

- [ ] Chip 8 ROMs support
- [ ] Instructions fetcher
- [x] Instruction set decoding
- [ ] Display rendering
- [ ] Keyboard input handling
- [ ] Sound support
- [ ] Debugging tools

## Installation

### Prerequisites

- **Rust**: Make sure you have Rust installed. You can install it from [rust-lang.org](https://www.rust-lang.org/) or using your favorite package manager.

### Building from Source

1. Clone the repository:

    ```bash
    git clone https://github.com/yourusername/chip-8-emulator.git
    cd chip-8-emulator
    ```

2. Build the project:

    ```bash
    cargo build --release
    ```

3. Run the emulator (note: functionality might be limited as this is still a WIP):

    ```bash
    cargo run --release
    ```

## Usage

Still WIP
