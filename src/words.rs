
pub struct Words {
    pointer:i32,
    words:Box<Vec<String>>,
}

impl Words {
    pub fn new(content:String) -> Words{
        let words = split_words(content);
        Words {
            pointer:-1,
            words:Box::new(words)
        }
    }

    pub fn get_next_word(&mut self) -> Option<&String> {
        self.pointer += 1;
        match self.words.get(self.pointer as usize) {
            Some(word) => {
                Some(word)
            },
            None => {
                None
            }
        }
    }

    pub fn set_pointer_to_previous(&mut self) {
        self.pointer -= 1;
    }

    pub fn get_pointer(&self) -> i32 {
        return self.pointer
    }

    pub fn get_all_words(&self) -> Vec<String> {
        (*self.words).clone()
    }

    pub fn is_finished(&self) -> bool {
//        println!("position is {} now",self.pointer);
        if self.pointer == (self.words.len()-1) as i32 {
            true
        } else {
            false
        }
    }
}

fn split_others(other:String) -> Vec<String> {
    //to solve the condition where two delimiters or operates attach
    // to each other when they are not "<=",">=",":="or"==".
    let mut result= Vec::new();
    if other != "<=" && other != ">=" && other != ":=" && other != "==" {
        result = other.chars().map(|x| x.to_string()).collect();
    } else {
        result.push(other);
    }

    result
}

fn split_words(contents:String) -> Vec<String> {
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

    words
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn split_words_test() {
        let content = "9+8*number-43".to_string();

        let words = split_words(content);

        assert_eq!(words,
            ["9","+","8","*","number","-","43"]
        )

    }

    #[test]
    fn words_test() {
        let content = "9+8*number-43".to_string();

        let words = Words::new(content);


        assert_eq!(words.get_all_words(),
                   ["9","+","8","*","number","-","43"]
        )

    }

}