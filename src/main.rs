use std::fs;
use std::env;

fn print_el(path_buff: &std::fs::DirEntry, indent: usize, is_last: bool) {
    println!("{}{}─ {}",
        "│  ".repeat(indent),
        if is_last {"└"}  else {"├"},
        match path_buff.path().file_name() {
            Some(file_name) => match file_name.to_str() {
                Some(file_name) => file_name,
                None => "[WRONG]",
            },
            None => "[N/A]",
        }
    )
}

fn ls_dir(path: &str, indent: usize) {
    match fs::read_dir(path) {
        Ok(dir_entry) => {
            let paths: Vec<std::fs::DirEntry> =  dir_entry.filter(|x| x.is_ok()).map(|x| x.unwrap()).collect();
            if paths.len() > 1 {
                for path in paths[0..paths.len()-1].iter() {
                    print_el(path, indent, false);
                    if path.path().is_dir() {
                        match path.path().to_str() {
                            Some(name) => ls_dir(name, indent + 1),
                            _ => ls_dir("[wrong name]", indent + 1),
                        }
                    }
                }
            }
            match paths.last() {
                Some(last_path) => print_el(last_path, indent, true),
                None => print!(""),
            }
        },
        Err(_) => print!("Couldn't read dir: {}", path),
    };
}

fn main() {
    /*
    Some day I'll make colorization (:
    */
    /*
    for color in env::vars().filter(|x| x.0 == "LS_COLORS").last().unwrap().1.split(":") {
        println!("\x1b[{}m color \x1b[0m", color.split("=").last().unwrap());
    }
    */

    match env::args().nth(1) {
        Some(path) => {
            println!("{}", path);
            ls_dir(&path, 0)
        },
        None => {
            println!(".");
            ls_dir("./", 0)
        },
    }
}