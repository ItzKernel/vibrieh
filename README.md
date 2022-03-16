<h1 align="center">Vibrieh</h1>
<p align="center">brrrrrrrrrrrrrrr</p>

- This is a little program written in `Rust` for making your *PlayStation 4* controller rumble.
- You can use it for debugging or any other sick purposes you can think of.
- Tested on `Arch btw`, probably works on other platforms too but I'm not sure.

## The reason why this exists
- There was already a program for this, but it was paid and I couldn't find any similar programs on Github. So I thought it would be nice to make an opensource alternative.

## Prerequisites
- Min. Rust 2021
- being able to type commands into the terminal

## Config
- **pattern** - `Sequence of "X" and "O" characters, "full" or "random"`
  - "OOXOOXOOX" - This would make your controller rumble 1/3 of the time
  - "full" - Controller rumble without delays
  - "random" - The pattern is completely random
- **delay_millis** - `a number, it's the time between each rumble in milliseconds. This affects the "X&O" and the "random" pattern`
- **small_motor_level** & **big_motor_level** - `rumble strength for each motor, must be a value between 0 and 255`


## Setup
```
git clone https://github.com/ItzKernel/vibrieh.git
cd vibrieh
cargo build
```
