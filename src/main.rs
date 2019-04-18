use std::collections::HashMap;
use std::{env, fs};
use std::hash::Hash;
use std::process;

mod token;
mod read_file;

fn main() {
    let reserved_words = vec!["begin","call","const","do","end","if","odd",
        "procedure","read","then","var","while","write",
        "(",")",",",";",".",
        "+","-","*","/","=","#","<","<=",">",">=",":="];

    let reserved_words:Vec<String> = reserved_words.into_iter().map(|x| String::from(x)).collect();

//    let operator = vec!["+","-","*","/","=","#","<","<=",">",">=",":="];
//    let delimiter = vec!["(",")",",",";","."];

    let args = env::args();

    let content = read_file::get_file_to_string(args)
        .unwrap_or_else(|err| {
            eprintln!("Problem when getting string from file: {}",err);
            process::exit(1);
        });

    let words = token::split_words(content);

    let words_except_number:Vec<String> = words.clone().into_iter()
        .filter(|x| {
            let not_number = match x.parse::<i32>(){
                Ok(_) => false,
                Err(_) => true
            };
            not_number
        }).collect();

    let mut counter = HashMap::new();

    for word in words_except_number {
        if !reserved_words.contains(&word){
            let count = counter.entry(word).or_insert(0);
            *count+=1;
        }
    }

    let words_with_name = token::recognize_words(words.clone());

    for word in words_with_name {
        println!("({},{})",word.0,word.1);
    }
}
