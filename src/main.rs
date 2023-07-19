fn main() {
    let command = std::env::args().nth(1);
    match command {
        Some(cmd) => match cmd.as_str() {
            "init" => {
                let home_dir = "";
                for (key, value) in std::env::vars_os() {
                    if key == "HOME" {
                        if let Some(path) = value.to_str() {
                            println!("{path}")
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
