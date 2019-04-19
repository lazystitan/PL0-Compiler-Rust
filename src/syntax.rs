//use std::slicer;
use std::vec;

use crate::token;

fn parse_expression_b(words: &mut token::Words) -> bool {
//    println!("parse_expression_b!");
    let mut flag = parse_add(words);
    if flag {
        flag = parse_term(words);
        if flag {
            true
        } else {
            false
        }
    } else {
        words.set_pointer_to_previous();
        true
    }

//    parse_expression(table_iter)
}

fn parse_expression(words: &mut token::Words) -> bool {
//    println!("parse_expression!");
    let has_add = parse_add(words);

    if !has_add {
        words.set_pointer_to_previous();
    }

    if !parse_term(words) {
        return false
    }
    parse_expression_b(words)
}

fn parse_term_b(words:&mut token::Words) -> bool {
//    println!("parse_term_b!");
    let mut flag = parse_times(words);
    if flag {
        flag = parse_factor(words);
        if flag {
            true
        } else {
            false
        }
    } else {
        words.set_pointer_to_previous();
        true
    }
}

fn parse_term(words:&mut token::Words) -> bool {
//    println!("parse_term!");
    let flag = parse_factor(words);
    if flag {
        parse_term_b(words)
    } else {
        false
    }
}

fn parse_factor(words:&mut token::Words) -> bool {
//    println!("parse_factor!");
    let symbol = words.get_next_word();
//    println!("{:?}",symbol);
    match symbol {
        Some(symbol) => {
            match token::parse_ident(symbol) {
                Ok(_) => true,
                Err(_) => match token::parse_number(symbol) {
                    Ok(_) => true,
                    Err(_) => {
                        if symbol != "(" {
                            return false
                        }
                        if  parse_expression(words) {
                            let symbol = words.get_next_word();
                            match symbol {
                                Some(symbol) => {
                                    if symbol != ")" {
                                        false
                                    } else {
                                        true
                                    }
                                },
                                None => false
                            }
                        } else {
                            false
                        }

                    }
                }
            }
        },
        None => false
    }
}

fn parse_times(words:&mut token::Words) -> bool{
//    println!("parse_times!");
    let symbol =  words.get_next_word();
    match symbol {
        Some(symbol) => {
            if symbol == "*" || symbol == "/"{
                true
            } else {
                false
            }
        },
        None => false
    }
}

fn parse_add(words:&mut token::Words) -> bool{
//    println!("parse_add!");
    let symbol =  words.get_next_word();
    match symbol {
        Some(symbol) => {
            if symbol == "+" || symbol == "-"{
                true
            } else {
                false
            }
        },
        None => false
    }
}

pub fn syntax_analysis(words:&mut token::Words) {
    if parse_expression(words) {
        println!("语法正确");
    } else {
        println!("语法错误:第{}字符'{}'非法",words.get_pointer()+1,words.get_current_word().unwrap());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token;

    #[test]
    fn parse_add_test() {
        let words = vec!["+".to_string(),"-".to_string(),"/".to_string()];
        let mut words = token::Words::new(words);
        let flag = parse_add(&mut words);
        assert_eq!(flag,true);

        let flag = parse_add(&mut words);
        assert_eq!(flag, true);

        let flag = parse_add(&mut words);
        assert_eq!(flag, false);

        let flag = parse_add(&mut words);
        assert_eq!(flag, false);


    }

    #[test]
    fn parse_times_test() {
        let mut words = vec!["*".to_string(),"-".to_string(),"/".to_string()];
        let mut words = token::Words::new(words);
//        assert_eq!(words.next().unwrap(),"*");


        let flag = parse_times(&mut words);
        assert_eq!(flag,true);

        let flag = parse_times(&mut words);
        assert_eq!(flag, false);

        let flag = parse_times(&mut words);
        assert_eq!(flag, true);

        let flag = parse_times(&mut words);
        assert_eq!(flag, false);


    }

    #[test]
    fn parse_expression_test() {
        let content = String::from("(12+5)//8+(a+5)*b");
        let mut words = token::split_words(content);
        let flag = parse_expression(&mut words);
        println!("{}-{}",words.get_pointer(),flag);

        assert_eq!(flag,false);
    }

    #[test]
    fn words_test() {
        let words = vec!["test1".to_string(),"test2".to_string(),"test3".to_string()];

        let mut words = token::Words::new(words);

        assert_eq!(words.get_next_word().unwrap(),"test1");
        assert_eq!(words.get_next_word().unwrap(),"test2");
        words.set_pointer_to_previous();
        assert_ne!(words.get_next_word().unwrap(),"test3");

    }

    #[test]
    fn syntax_analysis_test() {
        let content = String::from("(12+5)//8+(a+5)*b");
        let mut words = token::split_words(content);
        syntax_analysis(&mut words);

        assert_eq!(false,true);
    }
}