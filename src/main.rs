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
                            if let Ok(_) = std::fs::File::create(clipper_db_path) {
                                println!("Clipper initialized")
                            }else {
                                println!("Error while creating clipboard database")
                            }
                        } else {
                            println!("Error while reading your home directory")
                        }
                    }
                }
            }
            _ => {}
        },
        None => {}
    }
}
