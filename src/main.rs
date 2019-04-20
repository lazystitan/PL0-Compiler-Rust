use std::collections::HashMap;
use std::{process, env};

mod read_file;
mod syntax;
mod words;
mod token2;
mod token;
mod syntax2;

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

    let mut words = words::Words::new(content);

    let mut tokenlist = token2::TokenList::new(words).unwrap_or_else(
        |x| {
            eprintln!("Error:{}",x);
            process::exit(1);
        }
    );

    loop {
        let token = tokenlist.next();
        match token {
            Some(token)=> println!("{}",token),
            None => break
        };
    }

}
