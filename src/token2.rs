use std::{fmt, iter};
use std::collections::HashMap;
use crate::words::Words;

#[derive(Debug)]
pub struct Token {
    token:String,
    value:String
}

impl Token {
    pub fn new(token:&str,value:&str) -> Token {
        Token {
            token:token.to_string(),
            value:value.to_string()
        }
    }
    pub fn get_token(&self) -> &String {
        &self.token
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f,"({:^10},{:^8})",self.token,self.value)
    }
}

pub struct TokenList {
    list:Vec<Token>,
    state:i32
}

impl TokenList {
    pub fn new(words:Words) -> Result<TokenList,&'static str> {
        let mut list = recognize_words(&words);

//        println!("{:?}",list);

        for token in &list {
            if token.get_token() == "invalid" {
                return Err("Error accured when transfer to token list");
            }
        }

        list.push(Token{token:"".to_string(),value:"".to_string()});

        Ok(TokenList{
            list,
            state:-1
        })
    }


    pub fn push(&mut self, token: Token) {
        self.list.push(token);
    }

    pub fn len(&self) -> usize{
        self.len()
    }

    pub fn current(&self) -> Option<&Token>{
        match self.list.get(self.state as usize) {
            Some(token) => Some(token),
            None => None,
        }
    }

    pub fn next(&mut self) -> Option<&Token> {
        self.state += 1;
        match self.list.get(self.state as usize) {
            Some(token) => Some(token),
            None=> None
        }
    }
}

static LETTERS:[char;52] = ['a','b','c','d','e','f','g','h','i','j','k','l','m',
    'n','o','p','q','r','s','t','u','v','w','x','y','z',
    'A','B','C','D','E','F','G','H','I','J','K','L','M',
    'N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

static NUMBER:[char;10] = ['0','1','2','3','4','5','6','7','8','9'];

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

pub fn recognize_words(words:&Words) -> Vec<Token>{
    let table = init_hashtable();
    words.get_all_words().into_iter()
        .map(|x|{
            let x = x.to_lowercase();
            match table.get(x.as_str()) {
                Some(value) => Token::new(value,x.as_str()),

                None => {
                    //not reserve word
                    match parse_number(&x){
                        Ok(number) => Token::new(number.as_str(),x.as_str()),
                        Err(_) => {
                            //not a number
                            match parse_ident(&x) {
                                Ok(ident) => Token::new(ident.as_str(),x.as_str()),
                                Err(_) => Token::new("invalid",x.as_str()),
                            }
                        }
                    }
                }
            }
        }).collect()
}

pub fn parse_ident(word: &String) -> Result<String,&'static str> {
    enum Status {
        Start,
        End
    }

    let mut status = Status::Start;

    for tchar in word.chars() {
        match status {
            Status::Start => {
                if LETTERS.contains(&tchar) {
                    status = Status::End;
                } else {
                    return Err("is not an ident!")
                }
            },
            Status::End => {
                if LETTERS.contains(&tchar) {
                    continue;
                } else if NUMBER.contains(&tchar){
                    continue;
                } else {
                    return Err("is not an ident!")
                }
            }
        }
    }

    Ok(String::from("ident"))
}

pub fn parse_number(word: &String) -> Result<String,&'static str>{
    for number in word.chars() {
        if NUMBER.contains(&number) {
            continue;
        } else {
            return Err("is not a number");
        }
    }

    Ok(String::from("number"))
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn token_test() {
        let token = Token::new("ident","num");
        assert_eq!("ident",token.get_token());
        assert_eq!("num",token.get_value());

        println!("{}",token);
//        assert!(false);
    }

    #[test]
    fn tokenlist_test() {
        let content = "9+8*number-43".to_string();

        let token_example = Token::new("number","9");

        let words = Words::new(content);

        let mut tokenlist = TokenList::new(words).unwrap();
        let token = tokenlist.next().unwrap();
        assert_eq!(token.get_token(),token_example.get_token());
        assert_eq!(token.get_value(),token_example.get_value());
    }
}