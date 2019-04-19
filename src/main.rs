use std::collections::HashMap;
use std::{process, env};

mod token;
mod read_file;
mod syntax;

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

    let mut words = token::split_words(content);

    let words_with_name = token::recognize_words(words.get_all_words())
        .unwrap_or_else(|e| {
            eprintln!("Error when recognize words: {}",e);
            process::exit(1);
        });

    for word in words_with_name {
        println!("({: ^8},{: ^8})",word.0,word.1);
    }

    syntax::syntax_analysis(&mut words);
}
