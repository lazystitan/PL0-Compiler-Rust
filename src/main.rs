use std::collections::HashMap;
use std::{env, fs};
use std::hash::Hash;
use PL0_Compiler_Rust::split_words;


fn main() {

    let reserved_words = vec!["begin","call","const","do","end","if","odd",
        "procedure","read","then","var","while","write",
        "(",")",",",";",".",
        "+","-","*","/","=","#","<","<=",">",">=",":="];

    let reserved_words:Vec<String> = reserved_words.into_iter().map(|x| String::from(x)).collect();
//
//    let operator = vec!["+","-","*","/","=","#","<","<=",">",">=",":="];
//
//    let delimiter = vec!["(",")",",",";","."];

//    let test_words = String::from("Test just try");
    let mut args = env::args();

    if args.len() < 2 {
        panic!("Not enough arguments!");
    }

    args.next();
    let filename = args.next()
        .expect("Failed to init filename");

    let content = fs::read_to_string(filename)
        .expect("Failed to open file");
    let words = split_words(content);

    let words_except_number:Vec<String> = words.clone().into_iter()
        .filter(|x| {
            let not_number = match x.parse::<i32>(){
                Ok(_) => false,
                Err(_) => true
            };
            not_number
        }).collect();

//    println!("{:?}",words);

    let mut counter = HashMap::new();

    for word in words_except_number {
        if !reserved_words.contains(&word){
            let count = counter.entry(word).or_insert(0);
            *count+=1;
        }
    }

    let mut i = 0;
    for (key, value) in counter {
        println!("{}:({},{})",i,key,value);
        i+=1;
    }


//    println!("{:?}",a);



}
