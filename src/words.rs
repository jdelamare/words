use std::io::{BufReader,BufRead};
use std::fs::File;

// TODO: Find a more efficient way to search the file rather than 
    // bringing it all into memory.  What kind of abilities exist
    // for reading the file into the same small buffer and only bringing 
    // in ranges that matter?  Maybe, just for fun, only read the first 
    // character of a line and bring in all the words that start with
    // that letter.


pub struct Words {
    pub words: Vec<String>
}


impl Words {
    #[allow(unused)]
    fn new() -> Words {
        Words {
            words: vec![]
        }
    }

    #[allow(unused)]
    fn populate(&mut self) {    // TODO: return result type?
        let file = File::open("/usr/share/dict/words").expect("/usr/share/dict/words DNE");
        for line in BufReader::new(file).lines() {
            self.words.push(line.unwrap());
        }
        self.words.sort();
    }


    #[allow(unused)]
    fn get(&self, query: String) -> String {    // ? they're always words
        // same binary search, just return the item in the vector
        match self.words.binary_search(&query) {
            Ok(x) => return self.words[x].clone(),  // consider cloning this value?
            Err(_) => return "This list doesn't contain the word.".to_string()
        }
    }


    #[allow(unused)]
    fn exists(&self, query: String) -> bool {
        // binary search returns a Result containing the index where
        // the item exists, or an error and the index where it could be inserted
        match self.words.binary_search(&"the".to_string()) {    
            Ok(x) => return true,
            Err(_) => return false
        }
    }
}

#[cfg(test)]
#[test]
fn test_new() {
    let wordy = Words::new();
    assert!(wordy.words.len() == 0);
}


#[test]
fn test_length() {
    let mut wordy = Words::new();
    wordy.populate();
    assert_ne!(wordy.words.len(), 0);
}


#[test]
fn test_get() {
    let mut wordy = Words::new();
    wordy.populate();
    let x = wordy.get("chloroauric".to_string());
    assert_eq!(x, "chloroauric".to_string());
}


#[test]
fn test_populate() {
    let mut wordy = Words::new();
    wordy.populate();
    assert!(wordy.words.len() > 200000)
}

#[test]
fn test_exists() {
    let mut wordy = Words::new();
    wordy.populate();
    // panic!(wordy.get("polyaxial".to_string()));
    let x: bool = wordy.exists("the".to_string());
    assert!(x == true);
}