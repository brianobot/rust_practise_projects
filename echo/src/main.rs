use std::env;

/*
The Echo program is a simple command line tool that echos whatever is passed to in when calling it
*/

fn main() {
    let args = env::args().collect::<Vec<String>>();
    match args.len() {
        1 => println!("No Arguments passed to the program"),
        n => {
            for i in 1..n {
                println!("{}", args.get(i).unwrap());
            }
        }
    }
}
