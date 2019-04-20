use crate::token2::{TokenList,Token};

fn syntax_analysis(tokenlist: &mut TokenList) -> Result<(),&'static str>{

    tokenlist.next();

    let mut stack = Vec::new();
    stack.push("null");
    stack.push("exp");
    let mut counter = 1000;
    while let Some(token) = tokenlist.current() {
//        println!("while...");
        counter -= 1;
        if counter == 0 {
            return Err("infinite loop");
        }
        println!("{}",token);
        let token = token.get_token();
        if token.len() == 0 {
            col_null(&mut stack,tokenlist).unwrap_or_else(
                |x| {
                    println!("Error:{}",x);
                    assert!(false);
                }
            );
        } else if token == "plus" {
            col_plus(&mut stack,tokenlist).unwrap_or_else(
                |x| {
                    println!("Error:{}",x);
                    assert!(false);
                }
            );
        } else if token == "minus" {
            col_minus(&mut stack,tokenlist).unwrap_or_else(
                |x| {
                    println!("Error:{}",x);
                    assert!(false);
                }
            );
        } else if token == "times" {
            col_times(&mut stack,tokenlist).unwrap_or_else(
                |x| {
                    println!("Error:{}",x);
                    assert!(false);
                }
            );
        } else if token == "slash" {
            col_slash(&mut stack,tokenlist).unwrap_or_else(
                |x| {
                    println!("Error:{}",x);
                    assert!(false);
                }
            );
        } else if token == "ident" {
            col_ident(&mut stack,tokenlist).unwrap_or_else(
                |x| {
                    println!("Error:{}",x);
                    assert!(false);
                }
            );
        } else if token == "number" {
            col_number(&mut stack,tokenlist).unwrap_or_else(
                |x| {
                    println!("Error:{}",x);
                    assert!(false);
                }
            );
        } else if token == "lparen" {
            col_lparen(&mut stack,tokenlist).unwrap_or_else(
                |x| {
                    println!("Error:{}",x);
                    assert!(false);
                }
            );
        } else if token == "rparen" {
            col_rparen(&mut stack,tokenlist).unwrap_or_else(
                |x| {
                    println!("Error:{}",x);
                    assert!(false);
                }
            );
        } else {
            return Err("invalid token");
        }
    }

    Ok(())

}

fn col_plus(stack: &mut Vec<&str>,tokenlist:&mut TokenList) -> Result<(),&'static str>{
    println!("col_plus");
    if let Some(&top) = stack.last() {
        println!("{}",top);
        if top == "exp" {
            stack.pop();
            stack.push("exp_b");
            stack.push("term");
            stack.push("add");
        } else if top == "exp_b" {
            stack.pop();
            stack.push("exp_b");
            stack.push("term");
            stack.push("add");
        } else if top == "term" {
            return Err("Expect a term: ident, number, or lparen.");
        } else if top == "term_b" {
            stack.pop();
        } else if top == "factor" {
            return Err("Expect a factor: ident, number, or lparen.");
        } else if top == "add" {
            stack.pop();
            stack.push("plus");
        } else if top == "mult" {
            return Err("Expect a mult.");
        } else if top == "plus" {
            stack.pop();
            tokenlist.next();
        } else if top == "minus" {
            return Err("Expect a minus.");
        } else if top == "times" {
            return Err("Expect a times.");
        } else if top == "slash" {
            return Err("Expect a slash.");
        } else if top == "ident" {
            return Err("Expect a ident.");
        } else if top == "number" {
            return Err("Expect a number.");
        } else if top == "lparen" {
            return Err("Expect a lparen.");
        } else if top == "rparen" {
            return Err("Expect a rparen.");
        } else if top == "null" {
            return Err("Expect nothing else");
        }
    }
    Ok(())
}

fn col_minus(stack: &mut Vec<&str>,tokenlist:&mut TokenList) -> Result<(),&'static str>{
    println!("col_minus");
    if let Some(&top) = stack.last() {
        println!("{}",top);
        if top == "exp" {
            stack.pop();
            stack.push("exp_b");
            stack.push("term");
            stack.push("add");
        } else if top == "exp_b" {
            stack.pop();
            stack.push("exp_b");
            stack.push("term");
            stack.push("add");
        } else if top == "term" {
            return Err("Expect a term: ident, number, or lparen.");
        } else if top == "term_b" {
            stack.pop();
        } else if top == "factor" {
            return Err("Expect a factor: ident, number, or lparen.");
        } else if top == "add" {
            stack.pop();
            stack.push("minus");
        } else if top == "mult" {
            return Err("Expect a mult.");
        } else if top == "plus" {
            return Err("Expect a plus.");
        } else if top == "minus" {
            stack.pop();
            tokenlist.next();
        } else if top == "times" {
            return Err("Expect a times.");
        } else if top == "slash" {
            return Err("Expect a slash.");
        } else if top == "ident" {
            return Err("Expect a ident.");
        } else if top == "number" {
            return Err("Expect a number.");
        } else if top == "lparen" {
            return Err("Expect a lparen.");
        } else if top == "rparen" {
            return Err("Expect a rparen.");
        } else if top == "null" {
            return Err("Expect nothing else");
        }
    }
    Ok(())
}

fn col_times(stack: &mut Vec<&str>,tokenlist:&mut TokenList) -> Result<(),&'static str>{
    println!("col_times");
    if let Some(&top) = stack.last() {
        println!("{}",top);
        if top == "exp" {
            return Err("Expect a expression");
        } else if top == "exp_b" {
            stack.pop();
        } else if top == "term" {
            return Err("Expect a term: ident, number, or lparen.");
        } else if top == "term_b" {
            stack.pop();
            stack.push("term_b");
            stack.push("factor");
            stack.push("mult");
        } else if top == "factor" {
            return Err("Expect a factor: ident, number, or lparen.");
        } else if top == "add" {
            return Err("Expect a add.");
        } else if top == "mult" {
            stack.pop();
            stack.push("times");
        } else if top == "plus" {
            return Err("Expect a plus.");
        } else if top == "minus" {
            return Err("Expect a minus.");
        } else if top == "times" {
            stack.pop();
            tokenlist.next();
        } else if top == "slash" {
            return Err("Expect a slash.");
        } else if top == "ident" {
            return Err("Expect a ident.");
        } else if top == "number" {
            return Err("Expect a number.");
        } else if top == "lparen" {
            return Err("Expect a lparen.");
        } else if top == "rparen" {
            return Err("Expect a rparen.");
        } else if top == "null" {
            return Err("Expect nothing else");
        }
    }
    Ok(())
}

fn col_slash(stack: &mut Vec<&str>,tokenlist:&mut TokenList) -> Result<(),&'static str>{
    println!("col_slash");
    if let Some(&top) = stack.last() {
        println!("{}",top);
        if top == "exp" {
            return Err("Expect a expression");
        } else if top == "exp_b" {
            stack.pop();
        } else if top == "term" {
            return Err("Expect a term: ident, number, or lparen.");
        } else if top == "term_b" {
            stack.pop();
            stack.push("term_b");
            stack.push("factor");
            stack.push("mult");
        } else if top == "factor" {
            return Err("Expect a factor: ident, number, or lparen.");
        } else if top == "add" {
            return Err("Expect a add.");
        } else if top == "mult" {
            stack.pop();
            stack.push("slash");
        } else if top == "plus" {
            return Err("Expect a plus.");
        } else if top == "minus" {
            return Err("Expect a minus.");
        } else if top == "times" {
            return Err("Expect a times.");
        } else if top == "slash" {
            stack.pop();
            tokenlist.next();
        } else if top == "ident" {
            return Err("Expect a ident.");
        } else if top == "number" {
            return Err("Expect a number.");
        } else if top == "lparen" {
            return Err("Expect a lparen.");
        } else if top == "rparen" {
            return Err("Expect a rparen.");
        } else if top == "null" {
            return Err("Expect nothing else");
        }
    }
    Ok(())
}

fn col_ident(stack: &mut Vec<&str>,tokenlist:&mut TokenList) -> Result<(),&'static str>{
    println!("col_ident");
    if let Some(&top) = stack.last() {
        println!("{}",top);
        if top == "exp" {
            stack.pop();
            stack.push("exp_b");
            stack.push("term");
        } else if top == "exp_b" {
            return  Err("Expect a exp_b");
        } else if top == "term" {
            stack.pop();
            stack.push("term_b");
            stack.push("factor");
        } else if top == "term_b" {
            return  Err("Expect a term_b");
        } else if top == "factor" {
            stack.pop();
            stack.push("ident");
        } else if top == "add" {
            return Err("Expect a add.");
        } else if top == "mult" {
            return Err("Expect a mult.");
        } else if top == "plus" {
            return Err("Expect a plus.");
        } else if top == "minus" {
            return Err("Expect a minus.");
        } else if top == "times" {
            return Err("Expect a times.");
        } else if top == "slash" {
            return Err("Expect a slash.");
        } else if top == "ident" {
            stack.pop();
            tokenlist.next();
        } else if top == "number" {
            return Err("Expect a number.");
        } else if top == "lparen" {
            return Err("Expect a lparen.");
        } else if top == "rparen" {
            return Err("Expect a rparen.");
        } else if top == "null" {
            return Err("Expect nothing else");
        }
    }
    Ok(())
}
fn col_number(stack: &mut Vec<&str>,tokenlist:&mut TokenList) -> Result<(),&'static str>{
    println!("col_number");
    if let Some(&top) = stack.last() {
        println!("{}",top);
        if top == "exp" {
            stack.pop();
            stack.push("exp_b");
            stack.push("term");
        } else if top == "exp_b" {
            return  Err("Expect a exp_b");
        } else if top == "term" {
            stack.pop();
            stack.push("term_b");
            stack.push("factor");
        } else if top == "term_b" {
            return  Err("Expect a term_b");
        } else if top == "factor" {
            stack.pop();
            stack.push("number");
        } else if top == "add" {
            return Err("Expect a add.");
        } else if top == "mult" {
            return Err("Expect a mult.");
        } else if top == "plus" {
            return Err("Expect a plus.");
        } else if top == "minus" {
            return Err("Expect a minus.");
        } else if top == "times" {
            return Err("Expect a times.");
        } else if top == "slash" {
            return Err("Expect a slash.");
        } else if top == "ident" {
            return Err("Expect a ident.");
        } else if top == "number" {
            stack.pop();
            tokenlist.next();
        } else if top == "lparen" {
            return Err("Expect a lparen.");
        } else if top == "rparen" {
            return Err("Expect a rparen.");
        } else if top == "null" {
            return Err("Expect nothing else");
        }
    }
    Ok(())
}

fn col_lparen(stack: &mut Vec<&str>,tokenlist:&mut TokenList) -> Result<(),&'static str>{
    println!("col_lparen");
    if let Some(&top) = stack.last() {
        println!("{}",top);
        if top == "exp" {
            stack.pop();
            stack.push("exp_b");
            stack.push("term");
        } else if top == "exp_b" {
            return  Err("Expect a exp_b");
        } else if top == "term" {
            stack.pop();
            stack.push("term_b");
            stack.push("factor");
        } else if top == "term_b" {
            return  Err("Expect a term_b");
        } else if top == "factor" {
            stack.pop();
            stack.push("rparen");
            stack.push("exp");
            stack.push("lparen");
        } else if top == "add" {
            return Err("Expect a add.");
        } else if top == "mult" {
            return Err("Expect a mult.");
        } else if top == "plus" {
            return Err("Expect a plus.");
        } else if top == "minus" {
            return Err("Expect a minus.");
        } else if top == "times" {
            return Err("Expect a times.");
        } else if top == "slash" {
            return Err("Expect a slash.");
        } else if top == "ident" {
            return Err("Expect a ident.");
        } else if top == "number" {
            return Err("Expect a number.");
        } else if top == "lparen" {
            stack.pop();
            tokenlist.next();
        } else if top == "rparen" {
            return Err("Expect a rparen.");
        } else if top == "null" {
            return Err("Expect nothing else");
        }
    }
    Ok(())
}

fn col_rparen(stack: &mut Vec<&str>,tokenlist:&mut TokenList) -> Result<(),&'static str>{
    println!("col_rparen");
    if let Some(&top) = stack.last() {
        println!("{}",top);
        if top == "exp" {
            return Err("Expect a exp");
        } else if top == "exp_b" {
            stack.pop();
        } else if top == "term" {
            return Err("Expect a term");
        } else if top == "term_b" {
            stack.pop();
        } else if top == "factor" {
            return Err("Expect a factor");
        } else if top == "add" {
            return Err("Expect a add.");
        } else if top == "mult" {
            return Err("Expect a mult.");
        } else if top == "plus" {
            return Err("Expect a plus.");
        } else if top == "minus" {
            return Err("Expect a minus.");
        } else if top == "times" {
            return Err("Expect a times.");
        } else if top == "slash" {
            return Err("Expect a slash.");
        } else if top == "ident" {
            return Err("Expect a ident.");
        } else if top == "number" {
            return Err("Expect a number.");
        } else if top == "lparen" {
            return Err("Expect a lparen.")
        } else if top == "rparen" {
            stack.pop();
            tokenlist.next();
        } else if top == "null" {
            return Err("Expect nothing else");
        }
    }
    Ok(())
}

fn col_null(stack: &mut Vec<&str>,tokenlist:&mut TokenList) -> Result<(),&'static str>{
    println!("col_null");
    if let Some(&top) = stack.last() {
        println!("{}",top);
        if top == "exp" {
            return Err("Expect a exp");
        } else if top == "exp_b" {
            stack.pop();
        } else if top == "term" {
            return Err("Expect a term");
        } else if top == "term_b" {
            stack.pop();
        } else if top == "factor" {
            return Err("Expect a factor");
        } else if top == "add" {
            return Err("Expect a add.");
        } else if top == "mult" {
            return Err("Expect a mult.");
        } else if top == "plus" {
            return Err("Expect a plus.");
        } else if top == "minus" {
            return Err("Expect a minus.");
        } else if top == "times" {
            return Err("Expect a times.");
        } else if top == "slash" {
            return Err("Expect a slash.");
        } else if top == "ident" {
            return Err("Expect a ident.");
        } else if top == "number" {
            return Err("Expect a number.");
        } else if top == "lparen" {
            return Err("Expect a lparen.")
        } else if top == "rparen" {
            return Err("Expect a rparen.");
        } else if top == "null" {
            stack.pop();
            tokenlist.next();
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::words::Words;
    use crate::token2::TokenList;
    use std::process;

    #[test]
    fn test() {
        let ttr = "test";
        assert_eq!("test",ttr);
    }

    #[test]
    fn syntax_anlysis_test() {
        let test1 = "12+32*(3-x/5)".to_string();
        let words = Words::new(test1);
        let mut tokenlist = TokenList::new(words).unwrap();
        let flag = syntax_analysis(&mut tokenlist).unwrap_or_else(
            |x| {
                eprintln!("Error:{}",x);
                assert!(false);
            }
        );

        assert_eq!(flag,());

        let test2 = "12/32*(3-x/5)".to_string();
        let words = Words::new(test2);
        let mut tokenlist = TokenList::new(words).unwrap();
        let flag = syntax_analysis(&mut tokenlist).unwrap_or_else(
            |x| {
                eprintln!("Error:{}",x);
                assert!(false);
            }
        );

        assert_eq!(flag,());
    }
}