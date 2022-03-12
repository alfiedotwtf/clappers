use std::collections::HashMap;
use std::collections::HashSet;
use std::env;

impl Clappers {
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

    pub fn flags(mut self, argument_specs: Vec<&str>) -> Self {
        self.config.flags.add_to_config(argument_specs);
        self
    }

    pub fn singles(mut self, argument_specs: Vec<&str>) -> Self {
        self.config.singles.add_to_config(argument_specs);
        self
    }

    pub fn multiples(mut self, argument_specs: Vec<&str>) -> Self {
        self.config.multiples.add_to_config(argument_specs);
        self
    }

    pub fn parse(mut self) -> Self {
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

    pub fn get_flag(&self, parameter: &str) -> bool {
        self.config
            .flags
            .aliases
            .get(parameter)
            .map_or(false, |f| self.values.flags.contains(f))
    }

    pub fn get_single(&self, parameter: &str) -> String {
        self.config
            .singles
            .aliases
            .get(parameter)
            .map_or("".to_string(), |s| {
                self.values
                    .singles
                    .get(s)
                    .unwrap_or(&"".to_string())
                    .to_string()
            })
    }

    pub fn get_multiple(&self, parameter: &str) -> Vec<String> {
        self.config
            .multiples
            .aliases
            .get(parameter)
            .map_or(vec![], |m| {
                self.values.multiples.get(m).unwrap_or(&vec![]).to_vec()
            })
    }

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

    fn add_to_config(&mut self, argument_specs: Vec<&str>) {
        for argument_spec in argument_specs {
            let arguments: Vec<&str> = argument_spec.split('|').collect();

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
