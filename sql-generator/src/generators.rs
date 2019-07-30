pub struct Config {
    pub nb_queries: usize,
    pub nb_rows: usize,
    pub table_name: String,
    pub file_name: String,
    pub data_pattern: String,
    pub random: usize,
    pub verbose: usize,
}

pub struct Data {
    pub key: String,
    pub value: String,
}

pub struct Generator {
    config: Config,
    data: Data,
}

pub trait Generators {
    fn new(config: Config, data: Data) -> Self;
    fn prepare(&self) -> bool;
    fn insert(&self) -> Vec<String>;
    fn update(&self) -> Vec<String>;
}

impl Generators for Generator {
    fn new(config: Config, data: Data) -> Generator {
        Generator {
            config: config,
            data: data,
        }
    }

    fn prepare(&self) -> bool {
        return true;
    }

    fn insert(&self) -> Vec<String> {
        let mut queries : Vec<String> = Vec::new();
        let mut query : String = "".to_string();
        let mut percent : usize = 0;
        let coef : usize = ((self.config.nb_queries * self.config.nb_rows) / self.config.nb_rows) / 10;
        //let mut coef = (self.config.nb_queries * self.config.nb_rows) * 0.1;
        //coef = coef * div;

        println!("Let's generate {} sql rows {}...", self.config.nb_queries * self.config.nb_rows, coef);

        if self.config.nb_queries > 0 && self.config.nb_rows > 0 {
            for it_q in 0..self.config.nb_queries {
                if ((it_q + 1) % coef) == 0 {
                    percent = percent + 10;
                    println!("{}% generated...\n", percent);
                }
                query = format!("INSERT INTO {} (col_1, col_2, col_3) VALUES \n", self.config.table_name);
                for it_v in 0..self.config.nb_rows {
                    if it_v > 0 {
                        query = format!("{},\n", query);
                    }
                    query = format!("{} ('value_1', 'value_2', 'value_3')", query);
                }
                query = format!("{};\n", query);
                queries.push(query);
            }
        }

        queries
    }

    fn update(&self) -> Vec<String>  {
        let mut queries : Vec<String> = Vec::new();

        queries
    }
}
