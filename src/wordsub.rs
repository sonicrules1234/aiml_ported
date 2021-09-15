use std::collections::HashMap;
//use sonicobject::SonicObject;
use regex::{Regex, Match};
#[derive(Clone)]
pub struct WordSub {
    dict: HashMap<String, String>,
    reobj: Option<Regex>,
    redirty: bool,
}
impl WordSub {
    pub fn new(defaults: HashMap<String, String>) -> Self {
        Self {
            dict: defaults,
            reobj: None,
            redirty: true,
        }
    }
    fn update_regex(&mut self) -> () {
        self.reobj = Some(Regex::new(self.dict.clone().keys().map(|x| self.word_to_regex(x.to_string())).collect::<Vec<String>>().join("|").as_str()).unwrap());
    }
    fn word_to_regex(&mut self, word: String) -> String {
        let wordchars = word.chars().collect::<Vec<char>>();
        if word != "".to_string() && wordchars.first().unwrap().is_alphabetic() && wordchars.last().unwrap().is_alphabetic() {
            return format!("\\b{}\\b", regex::escape(word.as_str()));
        } else {
            return format!(r#"\b{}\b"#, regex::escape(word.as_str()));
        }
    }
    pub fn insert(&mut self, key: String, value: String) -> () {
        self.redirty = true;
        self.dict.insert(key.to_lowercase(), value.to_lowercase()).unwrap();
        self.dict.insert(crate::cap_words(key.to_string()), crate::cap_words(value.to_string())).unwrap();
        self.dict.insert(key.to_uppercase(), value.to_uppercase()).unwrap();
    }
    pub fn call(&mut self, thismatch: regex::Matches) -> String {
        self.dict[&thismatch.collect::<Vec<Match>>()[0].as_str().to_string()].as_str().to_string()
    }
    pub fn sub(&mut self, text: String) -> String {
        if self.redirty {
            self.update_regex()
        }
        let mut newtext = text.as_str().to_string();
        for x in self.reobj.as_ref().unwrap().captures_iter(text.as_str()) {
            let replacement = &self.dict[&x[0]];
            newtext = self.reobj.as_ref().unwrap().replace_all(newtext.as_str(), replacement).to_string()
        }
        newtext
    }
}