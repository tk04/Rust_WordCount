use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;

fn main() {
    //
    let x = match env::args().nth(1) {
        Some(val) => val,
        None => {
            println!("Enter a file command line arguement");
            return;
        }
    };
    let contents = match fs::read_to_string(x) {
        Ok(val) => val.to_lowercase(),
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => {
                eprintln!("File not found, enter valid file name");
                std::process::exit(1);
            }
            io::ErrorKind::PermissionDenied => {
                eprintln!("Permission denied, allow access to file path");
                std::process::exit(2);
            }
            _ => {
                eprintln!("An error happened, try again");
                std::process::exit(3);
            }
        },
    };

    let num_words: HashMap<&str, i32> = count_words(&contents);
    //println!("{:?}", num_words);
    let mut word_count: Vec<i32> = Vec::new();
    for x in &num_words {
        let (_, val) = x;
        word_count.push(*val);
    }
    let max_val = match word_count.iter().max() {
        Some(val) => *val,
        None => {
            println!("Could not find max value, try again!");
            std::process::exit(4);
        }
    };

    let mut max_words: Vec<&str> = Vec::new();
    for x in num_words {
        let (word, val) = x;
        if val == max_val {
            max_words.push(word);
        }
    }

    println!("Top word(s) occured {} time(s)", max_val);
    for word in max_words {
        println!("{}", word);
    }
}

fn count_words(contents: &str) -> HashMap<&str, i32> {
    //let vec:Vec<i32> = Vec::new();
    let mut count: HashMap<&str, i32> = HashMap::new();
    for x in contents.lines() {
        let val = x.split(" ");
        for word in val {
            if word != "" {
                let val = count.entry(word.trim()).or_insert(0);
                *val += 1;
            }
        }
    }
    count
}
