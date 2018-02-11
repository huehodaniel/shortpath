use std::env;
use std::path::{PathBuf,Component};

fn from_char(ch: char) -> String {
    let mut to_return = String::new();
    to_return.push(ch);
    to_return
}

fn abbreviate(path: PathBuf) -> PathBuf {
    let mut to_return = PathBuf::new();
    let components: Vec<Component> = path.components().collect();

    match components.split_last() {
        Some((last, elems)) => {
            for elem in elems {
                match elem {
                    &Component::Normal(os_str) => {
                        let letter = os_str.to_str()
                            .and_then(|s| s.chars().next())
                            .map(|ch| from_char(ch))
                            .unwrap_or("?".to_owned());
                        to_return.push(letter);
                    },
                    _ => to_return.push(elem.as_os_str())
                }
            }

            to_return.push(last.as_os_str());
        },
        None => ()
    }

    to_return
}

fn main() {
    let home = env::home_dir().unwrap();
    let current = env::current_dir().unwrap();

    let result = if current.starts_with(&home) {
        let stripped = current.strip_prefix(&home).unwrap();
        let mut path = PathBuf::from("~/");
        path.push(stripped);
        abbreviate(path)
    } else {
        abbreviate(current)
    };

    println!("{}", result.display());
}
