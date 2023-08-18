use std::{fs::File, path::Path};

fn main() {
    let command = std::env::args().nth(1);
    match command {
        Some(cmd) => match cmd.as_str() {
            "init" => {
                for (key, value) in std::env::vars_os() {
                    if key == "HOME" {
                        if let Some(path) = value.to_str() {
                            //Create clipboard database
                            let clipper_db_path = format!("{path}/.clipperdb.txt");
                            if Path::new(&clipper_db_path).is_file() {
                                println!("An existing Clipper database lives on your system at {clipper_db_path}");
                                return;
                            }
                            if let Ok(_) = File::create(&clipper_db_path) {
                                println!("Clipper database initialized at {} ", &clipper_db_path)
                            }else {
                                println!("Error while creating clipboard database")
                            }
                        } else {
                            println!("Error while reading your home directory")
                        }
                    }
                }
            },
            "set" =>{

            },
            _ => {}
        },
        None => {}
    }
}
