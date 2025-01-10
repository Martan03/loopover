# loopover

Loopover TUI implementation in Rust 🦀

![image](https://github.com/user-attachments/assets/113bc955-d580-4145-8f99-96f2d279907c)

## Table of Contents
- [Installation](#installation)
- [Usage](#usage)
- [Detailed description](#detailed-description)
    - [What is Loopover?](#what-is-loopover)
    - [TUI & usage description](#tui--usage-description)
    - [Other sizes](#other-sizes)
    - [Statistics](#statistics)
    - [Other keybinds](#other-keybinds)
- [Links](#links)

## Installation

You have to compile it yourself, but that shouldn't be a problem. Only thing you need is cargo. You need to go to the loopover project folder and run:

```
cargo build -r
```

After it's done compiling, you can start it in `./target/release/loopover`.

## Usage

You can start `loopover` in default size *(3x3)* like this:
```
./loopover
```

If you want to play with custom game size, you can do it like this:
```
./loopover -s <width> <height>
```

All the usage and options can be seen in the help:
```
./loopover -h
```

## Detailed description

### What is Loopover?

It's sliding tile puzzle game, which was originaly created by carykh. It's
displayed on a grid of a chosen size. The board is then scrambled and players
goal is to solve the puzzle so the numbers are in ascending order. Solving is
done by sliding rows and columns.

### TUI & usage description

When you start the game, you can immediately see the game itself. To scramble
the board, you can press `Enter` key, after which you can start solving the
puzzle. By using `Arrow` keys you can change selected cell and when pressing
`Shift` together with any `Arrow` key, you slide from selected position to
direction corresponding to the arrow pressed. The timer is started after the
first sliding move and after finishing the solve, it's saved in the stats.

![image](https://github.com/user-attachments/assets/113bc955-d580-4145-8f99-96f2d279907c)

### Other sizes

When the board contains more then 9 cells, more compact cell type is used.
The game works the same, it is just for the purpose of fitting the board to
the screen.

![image](https://github.com/user-attachments/assets/54e4cafd-15e5-4140-ac15-f5679bcdce7f)

### Statistics

You can see last few solves on the right of the board, but if you want to
look at more detailed statistics, you can press `Tab` to open Statistics
screen. It shows list of all solves on the board with current size as well
as time, date, moves and also game replay of the currently selected stat.

To view the replay, you can go to the next move by pressing `Right` key,
or going back by pressing `Left` key.

To go back to the game screen, you can press `Tab` again.

![image](https://github.com/user-attachments/assets/cbc9742b-b719-4e36-b4aa-408479b00d10)

### Other keybinds
- `CTRL + c`: rage quit
- `q`/`Esc`: exit the game

## Links

- **Author:** [Martan03](https://github.com/Martan03)
- **GitHub repository:** [loopover](https://github.com/Martan03/loopover)
- **Author website:** [martan03.github.io](https://martan03.github.io)
