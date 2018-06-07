//! # Words
//! `Words` grabs the file at `/usr/share/dict/words` and places each word in
//! a vector.  Given a word it can be determined to be in the list or not, and
//! copied into a local variable if desired.

// TODO: Find a more efficient way to search the file rather than 
    // bringing it all into memory.  What kind of abilities exist
    // for reading the file into the same small buffer and only bringing 
    // in ranges that matter?  Maybe, just for fun, only read the first 
    // character of a line and bring in all the words that start with
    // that letter.

use std::io::{BufReader,BufRead};
use std::fs::File;


pub struct Words {
    pub words: Vec<String>
}


impl Words {
    /// Instantiates the Words object
    ///
    /// # Examples
    ///
    /// ``` 
    /// let wordy = Words::new();
    /// 
    /// assert_eq!(wordy.words.len(), 0);
    /// ```
    #[allow(unused)]
    pub fn new() -> Words { // perhaps merge new and populate?
        Words {
            words: vec![]
        }
    }

    /// Populates the words field of the Words object
    ///
    /// # Examples
    ///
    /// ```
    /// let wordy = Words::new();
    /// wordy.populate();
    /// 
    /// assert_new!(wordy.words.len(), 0);
    /// ```
    #[allow(unused)]
    pub fn populate(&mut self) {    // TODO: return result type?
        let file = File::open("/usr/share/dict/words").expect("/usr/share/dict/words DNE");
        for line in BufReader::new(file).lines() {
            self.words.push(line.unwrap());
        }
        self.words.sort();
    }

    /// Grabs a copy of an entry in the words list
    ///
    /// # Examples
    ///
    /// ```
    /// let wordy = Words::new();
    /// wordy.populate();
    /// let word = wordy.get("parenthesis".to_string());
    /// assert_eq!(word, "parenthesis".to_string());
    /// ```
    #[allow(unused)]
    pub fn get(&self, query: String) -> String {    
        // binary search returns a Result containing the index where
        // the item exists, or an error and the index where it could be inserted
        match self.words.binary_search(&query) {
            Ok(x) => return self.words[x].clone(),  
            Err(_) => return "This list doesn't contain the word.".to_string()
        }
    }

    /// Determines if a word exists in the list
    ///
    /// # Examples
    ///
    /// ```
    /// let wordy = Words::new();
    /// wordy.populate();
    /// let w0 = wordy.exists("kaleidoscope".to_string());
    /// let w1 = wordy.exists("kaleidescope".to_string());
    /// assert_eq!(w0, true);
    /// assert_eq!(w1, false);
    /// ```
    #[allow(unused)]
    pub fn exists(&self, query: String) -> bool {
        match self.words.binary_search(&"the".to_string()) {    
            Ok(_) => return true,
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
fn test_populate() {
    let mut wordy = Words::new();
    wordy.populate();
    assert!(wordy.words.len() > 200000)
}

#[test]
fn test_get() {
    let mut wordy = Words::new();
    wordy.populate();
    let x = wordy.get("chloroauric".to_string());
    assert_eq!(x, "chloroauric".to_string());
}

#[test]
fn test_exists() {
    let mut wordy = Words::new();
    wordy.populate();
    let x: bool = wordy.exists("the".to_string());
    assert!(x == true);
}
