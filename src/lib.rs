//! Clappers -  Command Line Argument Parsing Particularly Easy, Relatively Stressfree!
//!
//! `Clappers` aims to be the most user-friendly command line argument parser this side of the
//! Milky Way. You configure a `Clappers` parser with the command line arguments you care about
//! via chaining, with the last link in the chain being a call to `parse()`. Command line argument
//! values are then retrieved via getters on the `Clappers` parser.
//!
//! ## Example 1 - A Minimal `ls`
//!
//! ```
//! use clappers::Clappers;
//!
//! fn main() {
//!     let clappers = Clappers::build()
//!         .add_flags(vec![
//!             "h|help",
//!             "l",
//!             "R|recursive",
//!         ])
//!         .parse();
//!
//!     if clappers.get_flag("help") {
//!         println!("
//!             usage: ls [arguments] [FILE1]...
//!
//!             Arguments:
//!                 -h|--help        Print this help
//!                 -l               Use a long listing format
//!                 -R|--recursive   List subdirectories recursively
//!         ");
//!     }
//!
//!     if clappers.get_flag("l") {
//!         // Show more details than usual
//!     }
//!
//!     if clappers.get_flag("R") {
//!         // Recurse into subdirectories
//!     }
//!
//!     if clappers.get_flag("recursive") {
//!         // This will also recurse
//!     }
//!
//!     let filenames: Vec<String> = clappers.get_leftovers();
//!
//!     // ...
//! }
//! ```
//!
//! ## Example 2 - A Minimal Compiler
//!
//! ```
//! use clappers::Clappers;
//!
//! fn main() {
//!     let clappers = Clappers::build()
//!         .add_flags(vec![
//!             "h|help",
//!             "v|verbose",
//!         ])
//!         .add_singles(vec![
//!             "o|output",
//!         ])
//!         .add_multiples(vec![
//!             "i|input",
//!             "I",
//!             "L",
//!         ])
//!         .parse();
//!
//!     if clappers.get_flag("help") {
//!         println!("
//!             usage: compile [arguments]
//!
//!             Arguments:
//!                 -h|--help                        Print this help
//!                 -v|--verbose                     Enable verbose mode
//!                 -I <dir1> ... <dirN>             Include directories
//!                 -L <dir1> ... <dirN>             Library directories
//!                 -i|--input <file1> ... <fileN>   Input filenames
//!                 -o|--output filename             Output filename
//!         ");
//!     }
//!
//!     let output_filename = clappers.get_single("output");
//!     let input_filenames: Vec<String> = clappers.get_multiple("input");
//!
//!     // ...
//! }
//! ```
//!
//! # Argument Types
//!
//! There are four types of arguments:
//!
//! 1. Flags
//! 2. Single value
//! 3. Multiple value
//! 4. Leftovers
//!
//! ## 1. Flag Arguments
//!
//! Flag arguments are `true` if they were supplied on the command line, and `false` otherwise e.g:
//!
//!```ignore
//! -h
//! -help
//! -v
//! --verbose
//!```
//!
//! *Note:* flag arguments do not take values
//!
//! ## 2. Single Value Arguments
//!
//! Single value arguments contain a single `String` value if they were supplied on the command
//! line, and empty `String` otherwise e.g:
//!
//!```ignore
//! -o filename.txt
//! --output filename.txt
//! -u Zelensky
//! --username Zelensky
//!```
//!
//! ## 3. Multiple Value Arguments
//!
//! Multiple value arguments contain at least a single `String` value if they were supplied on the
//! command line, and empty `String` otherwise e.g:
//!
//!```ignore
//! -i file1.txt
//! --input file1.txt
//! --host host1
//!```
//!
//! They can also contain multiple values, by repetition on the command line e.g:
//!
//!```ignore
//! -i file1.txt -i file2.txt ... -i fileN.txt
//! --host host1 --host host2 ... --host hostN
//!```
//!
//! The following format also works, reading from the first value until either the next argument is
//! reached, or until the end of the entire command line arguments e.g:
//!
//!```ignore
//! -i file1.txt file2.txt ... fileN.txt -n next_argument
//! --host host1 host2 hostN
//!```
//!
//! ## 4. Leftover Arguments
//!
//! Leftover argument values are values supplied on the command line that are not associated with
//! any argument. These includes:
//!
//! - any values when no other argument types have been supplied e.g:
//!
//!```ignore
//! ls file1 file2... fileN
//!```
//!
//! - any values after the double-dash argument e.g:
//!
//!```ignore
//! ls -l -R  -- file1 file2... fileN`
//!```
//!
//! - any value supplied to flags, because flags do not accept values
//!
//! - any remaining values supplied to singles value arguments, because these only take a one value
//!
//! # Caveats
//!
//! Combining flags is currently unsupported i.e the following does not work:
//!
//!```ignore
//! tar -zcf filename.tar.gz *
//!```
//!
//! Equals-Value is currently unsupported i.e the following does not work:
//!
//!```ignore
//! tar -zc --file=filename.tar.gz
//!```
//!
//! Commands with their own separate `Clappers` parser is currently unsupported i.e the following
//! does not work:
//!
//!```ignore
//! apt-get -y install -f cargo
//! apt-get update -f
//!```
//!
//! Command line argument values are always `String` types. This was by design, and no convenience
//! functions are planned. To convert a `String` to something else, use `String`'s build-in
//! `parse()` function instead:
//!
//!```
//! use clappers::Clappers;
//!
//! fn main() {
//!     let clappers = Clappers::build()
//!         .add_singles(vec!["number"])
//!         .parse();
//!
//!     let number: i32 = clappers.get_single("number").parse().unwrap_or(0);
//!
//!     println!("Double {number} is {}", number * 2);
//! }
//!```
//!

use std::collections::HashMap;
use std::collections::HashSet;
use std::env;

impl Clappers {
    /// Build a `Clappers` parser
    ///
    /// # Parameters
    ///
    /// None.
    ///
    /// # Return value
    ///
    /// An empty `Clappers` parser, which is ready to be configured by chaining:
    ///
    /// - `add_flags()`
    /// - `add_singles()`
    /// - `add_multiples()`
    ///
    /// Once configured, `parse()` is chained last to parse the actual command line arguments
    ///
    /// # Example
    ///
    /// ```
    /// use clappers::Clappers;
    ///
    /// fn main() {
    ///     let clappers = Clappers::build()
    ///         .add_flags(vec!["h|help", "v|verbose"])
    ///         .add_singles(vec!["o|output", "u|username"])
    ///         .add_multiples(vec!["i|input", "host"])
    ///         .parse();
    ///
    ///     // ...
    /// }
    /// ```
    ///
    pub fn build() -> Self {
        Self {
            config: Config {
                flags: ConfigType::new(),
                singles: ConfigType::new(),
                multiples: ConfigType::new(),
            },
            values: Values {
                flags: HashSet::new(),
                singles: HashMap::new(),
                multiples: HashMap::new(),
            },
        }
    }

    /// Add flag argument parsing to the `Clappers` config
    ///
    /// Flag arguments are `true` if they were supplied on the command line, and `false` otherwise
    /// e.g:
    ///
    ///```ignore
    /// -h
    /// -help
    /// -v
    /// --verbose
    ///```
    ///
    /// *Note:* flag arguments do not take values
    ///
    /// # Parameters
    ///
    /// `arg_specs` specifies which flag arguments on the command line to care about.
    ///
    /// Each `arg_spec` contains "|" separated flag argument alias names e.g:
    ///
    ///```ignore
    /// clappers.add_flags(vec!["h|help", "v|verbose"]);
    ///```
    ///
    /// # Return value
    ///
    /// The `Clappers` parser so that it can be chained
    ///
    /// # Example
    ///
    /// ```
    /// use clappers::Clappers;
    ///
    /// fn main() {
    ///     let clappers = Clappers::build()
    ///         .add_flags(vec!["h|help", "v|verbose"])
    ///         .add_singles(vec!["o|output", "u|username"])
    ///         .add_multiples(vec!["i|input", "host"])
    ///         .parse();
    ///
    ///     // ...
    /// }
    /// ```
    ///
    pub fn add_flags(mut self, arg_specs: Vec<&str>) -> Self {
        self.config.flags.add_to_config(arg_specs);
        self
    }

    /// Add single value argument parsing to the `Clappers` config
    ///
    /// Single value arguments contain a single `String` value if they were supplied on the command
    /// line, and empty `String` otherwise e.g:
    ///
    ///```ignore
    /// -o filename.txt
    /// --output filename.txt
    /// -u Zelensky
    /// --username Zelensky
    ///```
    ///
    /// # Parameters
    ///
    /// `arg_specs` specifies which single value arguments on the command line to care about.
    ///
    /// Each `arg_spec` contains "|" separated single value argument alias names e.g:
    ///
    ///```ignore
    /// clappers.add_singles(vec!["o|output", "u|username"]);
    ///```
    ///
    /// # Return value
    ///
    /// The `Clappers` parser so that it can be chained
    ///
    /// # Example
    ///
    /// ```
    /// use clappers::Clappers;
    ///
    /// fn main() {
    ///     let clappers = Clappers::build()
    ///         .add_flags(vec!["h|help", "v|verbose"])
    ///         .add_singles(vec!["o|output", "u|username"])
    ///         .add_multiples(vec!["i|input", "host"])
    ///         .parse();
    ///
    ///     // ...
    /// }
    /// ```
    ///
    pub fn add_singles(mut self, arg_specs: Vec<&str>) -> Self {
        self.config.singles.add_to_config(arg_specs);
        self
    }

    /// Add multiple value argument parsing to the `Clappers` config
    ///
    /// Multiple value arguments contain at least a singly populated `Vec<String>` value if they
    /// were supplied on the command line, and empty `Vec<String>` otherwise e.g:
    ///
    ///```ignore
    /// -i file1.txt
    /// --input file1.txt
    /// --host host1
    ///```
    ///
    /// They can also contain multiple values, by repetition on the command line e.g:
    ///
    ///```ignore
    /// -i file1.txt -i file2.txt ... -i fileN.txt
    /// --host host1 --host host2 ... --host hostN
    ///```
    ///
    /// The following format also works, reading from the first value until either the next
    /// argument is reached, or until the end of the entire command line arguments e.g:
    ///
    ///```ignore
    /// -i file1.txt file2.txt ... fileN.txt -n next_argument
    /// --host host1 host2 hostN
    ///```
    ///
    /// # Parameters
    ///
    /// `arg_specs` specifies which multiple value arguments on the command line to care about.
    ///
    /// Each `arg_spec` contains "|" separated multiple value argument alias names e.g:
    ///
    ///```ignore
    /// clappers.add_multiples(vec!["i|input", "host"]);
    ///```
    ///
    /// # Return value
    ///
    /// The `Clappers` parser so that it can be chained
    ///
    /// # Example
    ///
    /// ```
    /// use clappers::Clappers;
    ///
    /// fn main() {
    ///     let clappers = Clappers::build()
    ///         .add_flags(vec!["h|help", "v|verbose"])
    ///         .add_singles(vec!["o|output", "u|username"])
    ///         .add_multiples(vec!["i|input", "host"])
    ///         .parse();
    ///
    ///     // ...
    /// }
    /// ```
    ///
    pub fn add_multiples(mut self, arg_specs: Vec<&str>) -> Self {
        self.config.multiples.add_to_config(arg_specs);
        self
    }

    /// Parse the command line arguments with the current `Clappers` config
    ///
    /// # Parameters
    ///
    /// None
    ///
    /// # Return value
    ///
    /// The `Clappers` parser containing the parsed command line arguments values, accessed with:
    ///
    /// - `get_flags()`
    /// - `get_singles()`
    /// - `get_multiples()`
    /// - `get_leftovers()`
    ///
    /// # Example
    ///
    /// ```
    /// use clappers::Clappers;
    ///
    /// fn main() {
    ///     let clappers = Clappers::build()
    ///         .add_flags(vec!["h|help", "v|verbose"])
    ///         .add_singles(vec!["o|output", "u|username"])
    ///         .add_multiples(vec!["i|input", "host"])
    ///         .parse();
    ///
    ///     if clappers.get_flag("help") {
    ///         // Show help text
    ///     }
    ///
    ///     // ...
    /// }
    /// ```
    ///
    pub fn parse(mut self) -> Self {
        // setup "leftovers" before parsing
        self.config.multiples.name.insert("".to_string());
        self.config
            .multiples
            .aliases
            .insert("".to_string(), "".to_string());

        let mut args = env::args().peekable();

        // discard argv[0]
        args.next();

        while let Some(mut next) = args.next() {
            if next.starts_with('-') {
                next = next.split_off(1);

                if next.starts_with('-') {
                    next = next.split_off(1);
                }

                if let Some(name) = self.config.flags.aliases.get(&next) {
                    self.values.flags.insert(name.to_string());
                } else if let Some(name) = self.config.singles.aliases.get(&next) {
                    if let Some(v) = args.peek() {
                        if v.starts_with('-') {
                            continue;
                        } else {
                            self.values
                                .singles
                                .insert(name.to_string(), args.next().unwrap());
                        }
                    }
                } else if let Some(name) = self.config.multiples.aliases.get(&next) {
                    if self.values.multiples.get_mut(name).is_none() {
                        self.values.multiples.insert(name.clone(), vec![]);
                    }

                    while let Some(value) = args.peek() {
                        if value.starts_with('-') {
                            break;
                        } else {
                            self.values
                                .multiples
                                .get_mut(name)
                                .unwrap()
                                .push(args.next().unwrap());
                        }
                    }
                }
            } else {
                if self.values.multiples.get_mut("").is_none() {
                    self.values.multiples.insert("".to_string(), vec![]);
                }

                self.values.multiples.get_mut("").unwrap().push(next);
            }
        }

        self
    }

    /// Check if the flag was supplied on the command line for the specified argument
    ///
    /// # Parameters
    ///
    /// `argument` is any alias of the specified argument
    ///
    /// # Return value
    ///
    /// `true` if the flag was supplied on the command line, and `false` otherwise
    ///
    /// # Example
    ///
    /// ```
    /// use clappers::Clappers;
    ///
    /// fn main() {
    ///     let clappers = Clappers::build()
    ///         .add_flags(vec!["h|help"])
    ///         .parse();
    ///
    ///     if clappers.get_flag("help") {
    ///         // Show help text
    ///     }
    ///
    ///     if clappers.get_flag("h") {
    ///         // This will also show the help text
    ///     }
    ///
    ///     // ...
    /// }
    /// ```
    ///
    pub fn get_flag(&self, argument: &str) -> bool {
        self.config
            .flags
            .aliases
            .get(argument)
            .map_or(false, |f| self.values.flags.contains(f))
    }

    /// Get the single value supplied on the command line for the specified argument
    ///
    /// # Parameters
    ///
    /// `argument` is any alias of the specified argument
    ///
    /// # Return value
    ///
    /// The single `String` value if they were supplied on the command line, and empty `String`
    /// otherwise
    ///
    /// # Example
    ///
    /// ```
    /// use clappers::Clappers;
    ///
    /// fn main() {
    ///     let clappers = Clappers::build()
    ///         .add_singles(vec!["output"])
    ///         .parse();
    ///
    ///     println!("Output filename is {}", clappers.get_single("output"));
    ///
    ///     // ...
    /// }
    /// ```
    ///
    pub fn get_single(&self, argument: &str) -> String {
        self.config
            .singles
            .aliases
            .get(argument)
            .map_or("".to_string(), |s| {
                self.values
                    .singles
                    .get(s)
                    .unwrap_or(&"".to_string())
                    .to_string()
            })
    }

    /// Get multiple values supplied on the command line for the specified argument
    ///
    /// # Parameters
    ///
    /// `argument` is any alias of the specified argument
    ///
    /// # Return value
    ///
    /// Multiple `String` values if they were supplied on the command line, and empty `Vec<String>`
    /// otherwise
    ///
    /// # Example
    ///
    /// ```
    /// use clappers::Clappers;
    ///
    /// fn main() {
    ///     let clappers = Clappers::build()
    ///         .add_multiples(vec!["input"])
    ///         .parse();
    ///
    ///     println!("Input filenames are {:#?}", clappers.get_multiple("input"));
    ///
    ///     // ...
    /// }
    /// ```
    ///
    pub fn get_multiple(&self, argument: &str) -> Vec<String> {
        self.config
            .multiples
            .aliases
            .get(argument)
            .map_or(vec![], |m| {
                self.values.multiples.get(m).unwrap_or(&vec![]).to_vec()
            })
    }

    /// Get all values supplied on the command line that are not associated with any argument
    ///
    /// # Parameters
    ///
    /// None
    ///
    /// # Return value
    ///
    /// All `String` values supplied on the command line that are not associated with any argument,
    /// and empty `Vec<String>` otherwise
    ///
    /// # Example
    ///
    /// ```
    /// use clappers::Clappers;
    ///
    /// fn main() {
    ///     let clappers = Clappers::build()
    ///         .parse();
    ///
    ///     println!("`ls *` returned the following filenames: {:#?}", clappers.get_leftovers());
    ///
    ///     // ...
    /// }
    /// ```
    ///
    pub fn get_leftovers(&self) -> Vec<String> {
        self.get_multiple("")
    }
}

#[derive(Clone, Debug)]
pub struct Clappers {
    config: Config,
    values: Values,
}

#[derive(Clone, Debug)]
struct Config {
    flags: ConfigType,
    singles: ConfigType,
    multiples: ConfigType,
}

#[derive(Clone, Debug)]
struct Values {
    flags: HashSet<String>,
    singles: HashMap<String, String>,
    multiples: HashMap<String, Vec<String>>,
}

impl ConfigType {
    fn new() -> Self {
        Self {
            name: HashSet::new(),
            aliases: HashMap::new(),
        }
    }

    fn add_to_config(&mut self, arg_specs: Vec<&str>) {
        for arg_spec in arg_specs {
            let arguments: Vec<&str> = arg_spec.split('|').collect();

            if arguments.is_empty() {
                continue;
            }

            self.name.insert(arguments[0].to_string());

            for argument in &arguments {
                self.aliases
                    .insert(argument.to_string(), arguments[0].to_string());
            }
        }
    }
}

#[derive(Clone, Debug)]
struct ConfigType {
    name: HashSet<String>,
    aliases: HashMap<String, String>,
}
