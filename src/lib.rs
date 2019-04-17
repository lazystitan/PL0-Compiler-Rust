//use std::os::ios::raw::stat;
use std::{env, fs};
use std::fmt::Error;
use std::collections::HashMap;

fn init_hashtable() -> HashMap<&'static str,&'static str> {
    let mut table = HashMap::new();
    table.insert("begin","beginsym");
    table.insert("call","callsym");
    table.insert("const","constsym");
    table.insert("do","dosym");
    table.insert("end","endsym");
    table.insert("if","ifsym");
    table.insert("odd","oddsym");
    table.insert("procedure","proceduresym");
    table.insert("read","readsym");
    table.insert("then","thensym");
    table.insert("var","varsym");
    table.insert("while","whilesym");
    table.insert("write","writesym");
    table.insert("+","plus");
    table.insert("-","minus");
    table.insert("*","times");
    table.insert("/","slash");
    table.insert("=","eql");
    table.insert("#","neq");
    table.insert("<","lss");
    table.insert("<=","leq");
    table.insert(">","gtr");
    table.insert(">=","geq");
    table.insert(":=","becomes");
    table.insert("(","lparen");
    table.insert(")","rparen");
    table.insert(",","comma");
    table.insert(";","semicolon");
    table.insert(".","period");

    table
}

pub fn recognize_words(words:Vec<String>) -> Vec<(String,String)>{
    let table = init_hashtable();

//    println!("{:?}",words);

    words.into_iter()
        .map(|x| {
            match table.get(x.as_str()) {
                Some(value) => (value.to_string(),x),
                None => {
                    let value = match x.parse::<i32>(){
                        Ok(_) => String::from("number"),
                        Err(_) => String::from("ident")
                    };
                    (value, x)
                }
            }
        })
        .collect()
}

pub fn get_file_to_string(mut args: env::Args) -> Result<String,&'static str> {
    args.next();
    let filename = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a filename")
    };

    let content = fs::read_to_string(filename)
        .expect("Failed to read file.");

    Ok(content)

}

fn split_others(other:String) -> Vec<String> {
    //to solve the condition where two delimiters or operates attach to each other when they are not "<=",">=",":="or"==".
    let mut result= Vec::new();
    if other != "<=" && other != ">=" && other != ":=" && other != "==" {
        result = other.chars().map(|x| x.to_string()).collect();
    } else {
        result.push(other);
    }

    result
}

pub fn split_words(contents:String) -> Vec<String> {
    //using status transform to implement split words
    let delimiter = vec!['(',')',',',';','.','+','-','*','/','=','#','<','=','>',':'];

    enum Status {
        AlphabetAndNumber,
        Space,
        Others
    }
    let chars:Vec<char> = contents.chars().collect();

    let mut word = String::new();
    let mut words = Vec::new();
    let mut status = Status::Space;

    for a_char in chars {
        match status {
            Status::Space => {
                if delimiter.contains(&a_char) {
                    word.push(a_char);
                    status = Status::Others;
                } else if !a_char.is_ascii_whitespace() {
                    word.push(a_char);
                    status = Status::AlphabetAndNumber;
                } else {
                    continue;
                }
            },
            Status::AlphabetAndNumber => {
                if delimiter.contains(&a_char) {
                    status = Status::Others;
                    words.push(word);
                    word = String::new();
                    word.push(a_char);
                } else if !a_char.is_ascii_whitespace() {
                    word.push(a_char);
                } else {
                    status = Status::Space;
                    words.push(word);
                    word = String::new();
                }
            },
            Status::Others => {
                if delimiter.contains(&a_char) {
                    word.push(a_char);
                } else if !a_char.is_ascii_whitespace() {
                    status = Status::AlphabetAndNumber;

                    for item in split_others(word) {
                        words.push(item);
                    }

//                    words.push(word);
                    word = String::new();
                    word.push(a_char);
                } else {
                    status = Status::Space;

                    for item in split_others(word) {
                        words.push(item);
                    }
//                    words.push(word);
                    word = String::new();
                }
            }
        }
    }
    words.push(word);

    let words = words.into_iter()
        .filter(|x| x.len() != 0)
        .map(|x| x.to_lowercase())
        .collect();

    println!("{:?}",words);

    words
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_words_test() {
        let contents = String::from("\
Var num= 3;\n
Const num2 = 4;\n\
Begin\n
    a1 := b1 + num2;\n\
End.\n" );
        let output_right = vec!["var","num","=","3",";","const","num2","=","4",";","begin","a1",":=","b1","+","num2",";","end","."];
        let output_right:Vec<String> = output_right.iter().map(|x| String::from(*x)).collect();

        let output = split_words(contents);

        assert_eq!(output_right, output);

    }

    #[test]
    fn split_others_test() {
        let others = vec![":=","))",");","=="];
        let mut result = Vec::new();
        for other in others {
            for item in split_others(other.to_string()) {
                result.push(item);
            }
        }
        assert_eq!(result,
            [
                String::from(")"),
                String::from(")"),
                String::from(")"),
                String::from(";"),
            ]);
    }

}