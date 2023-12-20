use std::{
    env,
    fs::read_to_string,
    io::{stdin, Read},
    process::exit,
};

use count_charactors::Value;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("error: the number of argument must be 1");
        exit(1);
    }

    let input_json = if let Some(file_name) = args.first() {
        read_to_string(file_name)
            .ok()
            .unwrap_or_else(|| panic!("error: can't open a file {}", file_name))
    } else {
        let mut buffer = String::new();
        stdin()
            .read_to_string(&mut buffer)
            .expect("error: can't read a string from stdin");
        buffer
    };

    let json_value = count_charactors::parse(&input_json).expect("error: failed to parse json");
    let Value::Object(mut value) = json_value;

    for (i, j) in value.iter_mut() {
        j.retain(|c| c != '\n');
        println!("{}: {} {}", i, j.chars().count(), j);
    }
}
