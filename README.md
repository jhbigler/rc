# RC

A simple calculator written in Rust. This was just an exercise for me to better grasp Rust, and was also a fun way to learn the Nom parsing library.

## Features

Supports addition, subtraction, multiplication, division, and parenthesis

## Sample usage

```bash
rc '(24 - 2) * 34 - 20 / 4'
743
```

## Planned features

- Exponents
- Accept expressions from stdin, much like the bc command on linux
- (Maybe) variables, which are set with set with something like `--set x=42`
