use std::collections::HashMap;
use std::collections::HashSet;
use std::env;

#[derive(Clone, Debug)]
pub struct Clappers {
    config: ClappersConfig,
    values: ClappersValues,
}

#[derive(Clone, Debug)]
pub struct ClappersConfig {
    flags: HashSet<String>,
    flag_aliases: HashMap<String, String>,
    single: HashSet<String>,
    single_aliases: HashMap<String, String>,
    multiple: HashSet<String>,
    multiple_aliases: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct ClappersValues {
    flags: HashSet<String>,
    single: HashMap<String, String>,
    multiple: HashMap<String, Vec<String>>,
}

impl Clappers {
    pub fn build() -> Self {
        Self {
            config: ClappersConfig {
                flags: HashSet::new(),
                flag_aliases: HashMap::new(),
                single: HashSet::new(),
                single_aliases: HashMap::new(),
                multiple: HashSet::new(),
                multiple_aliases: HashMap::new(),
            },
            values: ClappersValues {
                flags: HashSet::new(),
                single: HashMap::new(),
                multiple: HashMap::new(),
            },
        }
    }

    pub fn get_flag(&self, parameter: &str) -> bool {
        match self.config.flag_aliases.get(parameter) {
            None => false,
            Some(real_flag) => self.values.flags.contains(real_flag),
        }
    }

    pub fn get_single(&self, parameter: &str) -> String {
        match self.config.single_aliases.get(parameter) {
            None => "".to_string(),
            Some(real_single) => self
                .values
                .single
                .get(real_single)
                .unwrap_or(&"".to_string())
                .to_string(),
        }
    }

    pub fn get_multiple(&self, parameter: &str) -> Vec<String> {
        match self.config.multiple_aliases.get(parameter) {
            None => {
                vec![]
            }
            Some(real_multiple) => self
                .values
                .multiple
                .get(real_multiple)
                .unwrap_or(&vec![])
                .to_vec(),
        }
    }

    pub fn flags(mut self, argument_specs: Vec<&str>) -> Self {
        for argument_spec in argument_specs {
            let arguments: Vec<&str> = argument_spec.split('|').collect();

            if arguments.is_empty() {
                continue;
            }

            self.config.flags.insert(arguments[0].to_string());

            for argument in &arguments {
                self.config
                    .flag_aliases
                    .insert(argument.to_string(), arguments[0].to_string());
            }
        }

        self
    }

    pub fn multiples(mut self, argument_specs: Vec<&str>) -> Self {
        for argument_spec in argument_specs {
            let arguments: Vec<&str> = argument_spec.split('|').collect();

            if arguments.is_empty() {
                continue;
            }

            self.config.multiple.insert(arguments[0].to_string());

            for argument in &arguments {
                self.config
                    .multiple_aliases
                    .insert(argument.to_string(), arguments[0].to_string());
            }
        }

        self
    }

    pub fn singles(mut self, argument_specs: Vec<&str>) -> Self {
        for argument_spec in argument_specs {
            let arguments: Vec<&str> = argument_spec.split('|').collect();

            if arguments.is_empty() {
                continue;
            }

            self.config.single.insert(arguments[0].to_string());

            for argument in &arguments {
                self.config
                    .single_aliases
                    .insert(argument.to_string(), arguments[0].to_string());
            }
        }

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

                if let Some(real_flag) = self.config.flag_aliases.get(&next) {
                    self.values.flags.insert(real_flag.to_string());
                } else if let Some(real_single) = self.config.single_aliases.get(&next) {
                    if let Some(v) = args.peek() {
                        if v.starts_with('-') {
                            continue;
                        } else {
                            self.values
                                .single
                                .insert(real_single.to_string(), args.next().unwrap());
                        }
                    }
                } else if let Some(real_multiple) = self.config.multiple_aliases.get(&next) {
                    if self.values.multiple.get_mut(real_multiple).is_none() {
                        self.values.multiple.insert(real_multiple.clone(), vec![]);
                    }

                    while let Some(value) = args.peek() {
                        if value.starts_with('-') {
                            break;
                        } else {
                            self.values
                                .multiple
                                .get_mut(real_multiple)
                                .unwrap()
                                .push(args.next().unwrap());
                        }
                    }
                }
            } else {
                if self.values.multiple.get_mut("").is_none() {
                    self.values.multiple.insert("".to_string(), vec![]);
                }

                self.values.multiple.get_mut("").unwrap().push(next);
            }
        }

        self
    }
}
