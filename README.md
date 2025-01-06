# Gratie ("grah-tee")
Grace's version of the [Piet programming language](https://dangermouse.net/esoteric/piet.html). Made by Grace and Jordan during the first half of the W1'24 batch at the Recurse Center.

(Piet Mondrian was a Dutch artist. According to Google Translate, "gratie" is Dutch for "grace"!)

## Usage
To write your own Gratie program:
1. Specify pixel colors in a comma-separated text file. These are the colors: ![gratie color wheel](gratie_color_wheel.png)
and this is the mapping from hue change steps to command:

| Hue change steps | Command       |
|------------------|---------------|
| 1                | add           |
| 2                | multiply      |
| 3                | pop           |
| 4                | in(char)      |
| 5                | out(number)   |
| 6                | duplicate     |
| 7                | out(char)     |
| 8                | in(number)    |
| 9                | push          |
| 10               | divide        |
| 11               | subtract      |

2. Run `cargo build && ./target/debug/gratie -d PATH/TO/YOUR_PROGRAM.txt` to run the repl. Currently, the available commands are `step` (execute one command) and `run` (run entire program and print result).

## Example programs
Push 3 onto the stack

![Push 3](tests/png/push3.png)

Print 7 to the console

![Print 7](tests/png/print7.png)

## Helpful resources for understanding how Piet works

[Visual video explanation of what's going on with the "direction pointer" and "codel chooser"](https://youtu.be/IcmCvT5whk0?si=cvZTspfWsUEzuZf1&t=402)

[User-friendly in-browser Piet IDE done by a former Recurser](https://gabriellesc.github.io/piet/) and [another one](https://piet-editor.github.io/)

[Examples for implementing control flow in Piet](https://web.archive.org/web/20190818054404/http://homepages.vub.ac.be/~diddesen/piet/index.html)
