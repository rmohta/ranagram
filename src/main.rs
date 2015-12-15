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
                let mut anagrams: HashMap<String, Vec<String>> = HashMap::new();
                let mut reader = BufReader::new(f);
                for maybe_line in reader.lines() {
                    let word = maybe_line.unwrap();
                    let mut chars: Vec<char> = word.chars().collect();
                    chars.sort();
                    let sorted_word: String = chars.into_iter().collect(); 
                    if anagrams.contains_key(&sorted_word) {
                        anagrams.get_mut(&sorted_word).push(word);
                    } else {
                        anagrams.insert(sorted_word, vec!(word));
                                                                                                                                                                                 }
                }
                let args: Vec<String> = env::args().collect();
                if args.len() == 2 {
                    let source = (&args[1]).clone();
                    let mut chars: Vec<char> = source.to_string().chars().collect();
                    chars.sort();
                    let sorted_word: String = chars.into_iter().collect(); 
                    match anagrams.find(&sorted_word) {
                        Some(anagrams) => println!("{}", anagrams),
                        None => println!("No anagrams found")
                    }
                } else {
                    println!("Call the app with exactly 1 argument, the word to find anagrams for");
                }
            }
      }
}

