use std::env;
use std::io::prelude::*; //Import BufRead
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use std::path::Path;


fn main(){
    let path = Path::new("/usr/share/dict/words");
    match File::open(&path) {
       Err(e) => println!("Error opening file: {}", e),
       Ok(f) => {
                let mut counter = 0;
                let mut anagrams: HashMap<String, Vec<String>> = HashMap::new();
                //let mut anagrams = HashMap::new(); 
                let reader = BufReader::new(f);
                for maybe_line in reader.lines() {
                   //reader.lines() will yield iterator on io::Result<String>
                   //string returned will not have a newline byte (the 0xA byte) or CRLF (0xD, 0xA bytes) at the end
                    let word = maybe_line.unwrap();
                    let mut chars: Vec<char> = word.chars().collect(); //Type: Vec<char>
                    chars.sort();
                    let sorted_word: String = chars.into_iter().collect(); 
                    if anagrams.contains_key(&sorted_word) {
                        anagrams.get_mut(&sorted_word).unwrap().push(word);
                    } else {
                        anagrams.insert(sorted_word, vec![word]);
                    }
                    counter = counter + 1;
                }
                println!("Number of words present in dictionary: {}", counter);
                println!("Number of unique entries in anagram map: {}", anagrams.len());
                let args: Vec<String> = env::args().collect();
                if args.len() == 2 {
                    let source = (&args[1]).clone();
                    let mut chars: Vec<char> = source.to_string().chars().collect();
                    chars.sort();
                    let sorted_word: String = chars.into_iter().collect(); 
                    match anagrams.get(&sorted_word) {
                        Some(anagrams) => println!("{:?}", anagrams),
                        None => println!("No anagrams found.")
                    }
                } else {
                    println!("Call the program with 1 (one) argument ONLY, the word to find anagrams for.");
                }
            }
      }
}




