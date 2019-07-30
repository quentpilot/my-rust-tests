use std::env;

pub struct Argument {
    pub name: String,
    pub value: String,
}

pub struct Wanted  {
    name: String
}

impl Wanted {
    fn new(name: String) -> Wanted {
        Wanted {
            name: name.to_string()
        }
    }
}

impl Argument {
    fn new(name: String, value: String) -> Argument {
        Argument {
            name: name.to_string(),
            value: value.to_string()
        }
    }
}

pub struct Arguments {
    pub storage: Vec<Argument>,
}

impl Arguments {
    pub fn new(storage: Vec<Argument>) -> Arguments {
        Arguments {
            storage: storage
        }
    }

    pub fn parse(wanted: Vec<String>) -> Vec<Argument> {
        let args: Vec<String> = env::args().collect();
        let mut options : Vec<Argument> = Vec::new();

        for arg in args {
            options.push(Argument::new(arg, "val".to_string()));
        }
        options
    }
}
