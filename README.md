# Clappers

Command Line Argument Parsing Particularly Easy, Relatively Straightforward!

`Clappers` aims to be the most user-friendly command line argument parser this
side of the Milky Way. You configure a `Clappers` parser with the command line
arguments you care about via chaining, with the last link in the chain being a
call to `parse()`. Command line argument values are then retrieved via getters
on the `Clappers` parser.

## Example 1 - A Minimal Directory Listing

```
use clappers::Clappers;

fn main() {
    let clappers = Clappers::build()
        .add_flags(vec![
            "h|help",
            "l",
            "R|recursive",
        ])
        .parse();

    if clappers.get_flag("help") {
        println!("
            usage: ls [arguments] [FILE1]...

            Arguments:
                -h|--help        Print this help
                -l               Use a long listing format
                -R|--recursive   List subdirectories recursively
        ");
    }

    if clappers.get_flag("l") {
        // Show more details than usual
    }

    if clappers.get_flag("R") {
        // Recurse into subdirectories
    }

    if clappers.get_flag("recursive") {
        // This will also recurse
    }

    let filenames: Vec<String> = clappers.get_leftovers();

    // ...
}
```

## Example 2 - A Minimal Compiler

```
use clappers::Clappers;

fn main() {
    let clappers = Clappers::build()
        .add_flags(vec![
            "h|help",
            "v|verbose",
        ])
        .add_singles(vec![
            "o|output",
        ])
        .add_multiples(vec![
            "i|input",
            "I",
            "L",
        ])
        .parse();

    if clappers.get_flag("help") {
        println!("
            usage: compile [arguments]

            Arguments:
                -h|--help                        Print this help
                -v|--verbose                     Enable verbose mode
                -I <dir1> ... <dirN>             Include directories
                -L <dir1> ... <dirN>             Library directories
                -i|--input <file1> ... <fileN>   Input filenames
                -o|--output filename             Output filename
        ");
    }

    let output_filename = clappers.get_single("output");
    let input_filenames: Vec<String> = clappers.get_multiple("input");

    // ...
}
```

# Argument Types

There are four types of arguments:

1. Flags
2. Single value
3. Multiple value
4. Leftovers

## 1. Flag Arguments

Flag arguments are `true` if they were supplied on the command line, and
`false` otherwise e.g:

```
-h
-help
-v
--verbose
```

*Note:* flag arguments do not take values

## 2. Single Value Arguments

Single value arguments contain a single `String` value if they were supplied on
the command line, and empty `String` otherwise e.g:

```
-o filename.txt
--output filename.txt
-u Zelensky
--username Zelensky
```

## 3. Multiple Value Arguments

Multiple value arguments contain at least a single `String` value if they were
supplied on the command line, and empty `String` otherwise e.g:

```
-i file1.txt
--input file1.txt
--host host1
```

They can also contain multiple values, by repetition on the command line e.g:

```
-i file1.txt -i file2.txt ... -i fileN.txt
--host host1 --host host2 ... --host hostN
```

The following format also works, reading from the first value until either the
next argument is reached, or until the end of the entire command line arguments
e.g:

```
-i file1.txt file2.txt ... fileN.txt -n next_argument
--host host1 host2 hostN
```

## 4. Leftover Arguments

Leftover argument values are values supplied on the command line that are not
associated with any argument. These includes:

- any values when no other argument types have been supplied e.g:

```
ls file1 file2... fileN
```

- any values after the double-dash argument e.g:

```
ls -l -R  -- file1 file2...  fileN`
```

- any value supplied to flags, because flags do not accept values

- any remaining values supplied to singles value arguments, because these only
  take a one value

# Caveats

Combining flags is currently unsupported i.e the following does not work:

```
tar -zcf filename.tar.gz *
```

Equals-Value is currently unsupported i.e the following does not work:

```
tar -zc --file=filename.tar.gz
```

Commands with their own separate `Clappers` parser is currently unsupported i.e
the following does not work:

```
apt-get -y install -f cargo
apt-get update -f
```

Command line argument values are always `String` types. This was by design, and
no convenience functions are planned. To convert a `String` to something else,
use `String`'s build-in `parse()` function instead:

```
use clappers::Clappers;

fn main() {
    let clappers = Clappers::build()
        .add_singles(vec!["number"])
        .parse();

    let number: i32 = clappers.get_single("number").parse().unwrap();

    println!("Double {number} is {}", number * 2);
}
```

# Support

Please report any bugs or feature requests at:

* [https://github.com/alfiedotwtf/clappers/issues](https://github.com/alfiedotwtf/clappers/issues)

Feel free to fork the repository and submit pull requests :)

Слава Україні!

# Author

[Alfie John](https://www.alfie.wtf) &lt;[alfie@alfie.wtf](mailto:alfie@alfie.wtf)&gt;

# Warranty

IT COMES WITHOUT WARRANTY OF ANY KIND.

# Copyright and License

Copyright (C) 2022 by Alfie John

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License and GNU Free Documentation License
as published by the Free Software Foundation, either version 3 of the GPL or
1.3 of the GFDL, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
this program. If not, see [http://www.gnu.org/licenses/](http://www.gnu.org/licenses/).
