# chip8c

chip8c assembles text files  of CHIP-8 assembly code to binaries runnable in an emulator.

    chip8c 0.1.0
    Keating Reid <keating.reid@protonmail.com>
    Assembles CHIP-8 assembly files to binary
    
    USAGE:
        chip8c [FLAGS] <INPUT>...
    
    FLAGS:
        -o               Output paths. Each argument to -o applies to the INPUT argument with the same index.
        -h, --help       Prints help information
        -V, --version    Prints version information
    
    ARGS:
        <INPUT>...    Input files to compile

For further information, consult the [project wiki](https://github.com/Keating950/chip8c/wiki).

## Arguments

### `-o`
One or more paths to use for output files. If no paths (or fewer paths than there are inputs) are provided,
the default pattern is `[input file stem].ch8`.

## Supported features

At the moment, no convenience features such as labels, macros, etc. are
provided. This may change in future versions.
