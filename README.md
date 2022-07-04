# chip8c

chip8c assembles text files  of CHIP-8 assembly code to binaries runnable in an emulator.

    chip8c 0.1.0
    Assembles CHIP-8 files to binary
    
    USAGE:
        chip8c [OPTIONS] <INPUT>
    
    ARGS:
        <INPUT>    File to compile
    
    OPTIONS:
        -h, --help               Print help information
        -o, --output <OUTPUT>    Output path. Defaults to [input path].bin
        -V, --version            Print version information

For further information, consult the [project wiki](https://github.com/Keating950/chip8c/wiki).

## Arguments

### `-o`
One or more paths to use for output files. If no paths (or fewer paths than there are inputs) are provided,
the default pattern is `[input file stem].bin`.
