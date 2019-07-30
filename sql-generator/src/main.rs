use std::io::prelude::*;
use std::fs::File;
use std::time::SystemTime;
use std::env;
use std::collections::HashMap;
use std::cmp;
use std::fs;
use std::path::PathBuf;

use arguments::*;
use generators::*;
mod arguments;
mod generators;

fn generate_insert(nb_queries: usize, nb_values: usize, table: String) -> Vec<String> {
    let mut queries : Vec<String> = Vec::new();
    let mut query : String = "".to_string();
    let mut percent : usize = 0;
    let coef : usize = ((nb_queries * nb_values) / nb_values) / 10;

    println!("Let's generate {} sql rows...", nb_queries * nb_values);

    if nb_queries > 0 && nb_values > 0 {
        for it_q in 0..nb_queries {
            if ((it_q + 1) % coef) == 0 {
                percent = percent + (coef / (10));
                println!("{}% generated...\n", percent);
            }
            query = format!("INSERT INTO {} (col_1, col_2, col_3) VALUES \n", table);
            for it_v in 0..nb_values {
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

fn get_path() -> String {
    let relative_path = PathBuf::from("sql-generator");
    let mut absolute_path = std::env::current_dir();

    match absolute_path {
        Ok(absolute_path) => absolute_path.display().to_string(),
        _ => relative_path.display().to_string()
    }
}

fn save_queries(filename: &str, queries: &Vec<String>) -> std::io::Result<()> {
    let mut file = File::create(filename.to_string())?;

    for query in queries {
        file.write(query.as_bytes())?;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut wanted : Vec<String> = Vec::new();
    wanted.push(String::from("-queries"));
    wanted.push(String::from("-values"));
    wanted.push(String::from("-table"));

    let options = Arguments::new(Arguments::parse(wanted));

    let nb_queries = 1000;
    let nb_values = 5000;
    let table = "usr_user".to_string();
    let output = format!("{}/sql/insert_{}_rows.sql", get_path(), nb_queries * nb_values);

    let config = Config {
        nb_queries: nb_queries,
        nb_rows: nb_values,
        table_name: table,
        file_name: "".to_string(),
        data_pattern: "(({).*?(}))".to_string(),
        random: 0,
        verbose: 1,
    };

    let data = Data {
        key : "title".to_string(),
        value: "Let's Rock".to_string(),
    };

    let generator = Generator::new(config, data);
    let timer = SystemTime::now();
    let queries = generator.insert();

    let difference = timer.duration_since(timer)
                          .expect("SystemTime::duration_since failed");

    println!("Generated in {:?}", timer);

    save_queries(&output, &queries)


    // for opt in options.storage {
    //     println!("{} => {}", opt.name, opt.value);
    // }

}
