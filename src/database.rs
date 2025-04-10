use std::fs::File;
use std::io::{prelude::*, BufReader};
use rand::Rng;

pub fn get_word(word_length: i32)->Result<String, &'static str>{
    let file_result = File::open("data/en_US.dic");
    let file = match  file_result {
        Result::Ok(file) => file,
        Result::Err(e) => return Err("Couldn't read file.")
    };

    let reader = BufReader::new(file);

    let mut words = vec![String::new(); 0];

    for line in reader.lines() {
        if let Ok(word) = line{
            //the dic file has /M or similar tags on things, remove those
            // This splits the word, turns it into a vector I can index
            let parts = word.split('/').collect::<Vec<&str>>();

            //remove special characters and turn lowercase
            let word = parts[0]
                .replace(&['(', ')', ',', '\"', '.', ';', ':', '\'','\\','/'][..], "")
                .to_lowercase();

            //if length is what I asked for, push to vector
            let length: i32 = word.chars().count().try_into().unwrap();
            if  length == word_length {
                words.push(word)
            }   
        
        }
    }

    // pick a random word and return it
    let choice = rand::thread_rng().gen_range(0..words.len());
    Ok(words[choice].clone())
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_word() {
        let result = get_word(3);

    }
}