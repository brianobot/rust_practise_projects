use std::fs::read_dir;
use std::env;


fn main() {
    let dir_entries = read_dir(".").expect("Invalid Directory");

    let file_names = dir_entries
        .map(|entry| {
            entry.unwrap().file_name()
        })
        .collect::<Vec<_>>();

    for file_name in file_names {
        print!("{:?}", file_name);
    }
    println!("");
}
