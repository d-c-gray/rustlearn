use crate::database::get_word;
use std::io;
pub fn triangle(number: i32, invert: bool)->Result<(), &'static str> {
    for x in 0..number+1 {
        let mut ln  = String::from("");
        let mut x2 = x.clone();
        if invert {
            x2 = number - x;
        }
        for _i in 0..x2 {
            ln = ln + "*";
        }
        println!("{ln}");
    }
    Ok(())

}

pub fn make_hangman_output(word: &str, guessed: &str) -> String {
    let mut output = String::new();
    let mut inside = false; 
    for c_word in word.chars() {
        inside = false;
        for c_guess in guessed.chars() {
            if c_word == c_guess{
                inside  = true
            }
        }
        if inside {
            output.push(c_word);
        } else {
            output.push('_')
        }
    }
    output
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

pub fn get_hangman_guess()-> char {
    println!("Next guess?");
    let guess = loop {
        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(_) => (),
            Err(_)=> println!("Couldn't read the input")
        }
        if guess.ends_with('\n') {
            guess.pop();
            if guess.ends_with('\r') {
                guess.pop();
            }
        }
        if guess.chars().count() == 1 {
            break guess.chars().nth(0).unwrap();
            
        } else {
            println!("1 character at a time please!");
        }
    };
    guess
}

pub fn hangman(word_length: i32)->Result<(), &'static str> {
    let word_result = get_word(word_length);
    let word = if let Result::Ok(word) = word_result {
        word
    } else {
        return Err("Failed to read word.");
    };

    println!("The word is {word}");

    let mut guessed = String::new();
    let mut misses = 0;
    while misses < 10{
        let output = make_hangman_output(&word, &guessed);
        println!("\n Game Status: {output} \n Guessed : {guessed} \n");
        let guess = get_hangman_guess();
        guessed.push(guess);

        misses = misses + 1;
    }

    Ok(())
}