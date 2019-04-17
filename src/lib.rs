//use std::os::ios::raw::stat;

fn split_others(other:String) -> Vec<String> {
    //to solve the condition where two delimiters or operates attach to each other when they are not "<=",">=",":="or"==".
    let mut result= Vec::new();
    if other != "<=" && other != ">=" && other != ":=" && other != "==" {
        result = other.chars().map(|x| x.to_string()).collect();
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
    fn experiment() {
        let a = String::from("test");
        let b = String::from("test");
        let c = String::from("not test");
        assert_eq!(a,b);
        assert_eq!(a,c);
        assert_eq!(b,c);

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