use serde_json::Value;
use std::collections::HashMap;
const DEFAULT_GENDER_STRING: &str = r#"{
    "he": "she",
    "him": "her",
    "his": "her",
    "himself": "herself",
    "she": "he",
    "her": "him",
    "hers": "his",
    "herself": "himself"
}"#;

const DEFAULT_PERSON_STRING: &str = r#"{
    "I": "he",
    "me": "him",
    "my": "his",
    "mine": "his",
    "myself": "himself",
    "he":"I",
    "him":"me",
    "his":"my",
    "himself":"myself",
    "she":"I",
    "her":"me",
    "hers":"mine",
    "herself":"myself"
}"#;

const DEFAULT_PERSON2_STRING: &str = r#"{
    "I": "you",
    "me": "you",
    "my": "your",
    "mine": "yours",
    "myself": "yourself",
    "you": "me",
    "your": "my",
    "yours": "mine",
    "yourself": "myself"
}"#;

const DEFAULT_NORMAL_STRING: &str = r#"{
    "wanna": "want to",
    "gonna": "going to",
    "I'm": "I am",
    "I'd": "I would",
    "I'll": "I will",
    "I've": "I have",
    "you'd": "you would",
    "you're": "you are",
    "you've": "you have",
    "you'll": "you will",
    "he's": "he is",
    "he'd": "he would",
    "he'll": "he will",
    "she's": "she is",
    "she'd": "she would",
    "she'll": "she will",
    "we're": "we are",
    "we'd": "we would",
    "we'll": "we will",
    "we've": "we have",
    "they're": "they are",
    "they'd": "they would",
    "they'll": "they will",
    "they've": "they have",
    "y'all": "you all",    
    "can't": "can not",
    "cannot": "can not",
    "couldn't": "could not",
    "wouldn't": "would not",
    "shouldn't": "should not",
    "isn't": "is not",
    "ain't": "is not",
    "don't": "do not",
    "aren't": "are not",
    "won't": "will not",
    "weren't": "were not",
    "wasn't": "was not",
    "didn't": "did not",
    "hasn't": "has not",
    "hadn't": "had not",
    "haven't": "have not",
    "where's": "where is",
    "where'd": "where did",
    "where'll": "where will",
    "who's": "who is",
    "who'd": "who did",
    "who'll": "who will",
    "what's": "what is",
    "what'd": "what did",
    "what'll": "what will",
    "when's": "when is",
    "when'd": "when did",
    "when'll": "when will",
    "why's": "why is",
    "why'd": "why did",
    "why'll": "why will",
    "it's": "it is",
    "it'd": "it would",
    "it'll": "it will"
}"#;

pub fn str_to_hashmap(instr: &str) -> HashMap<String, String> {
    let mut v: Value = serde_json::from_str(instr).unwrap();
    let out = v.as_object_mut().unwrap();
    let mut outhashmap: HashMap<String, String> = HashMap::new();
    for key in out.keys() {
        outhashmap.insert(key.to_string(), out[key].as_str().unwrap().to_string());
    }
    outhashmap
}
pub fn get_default_gender() -> HashMap<String, String> {
    str_to_hashmap(DEFAULT_GENDER_STRING)
}
pub fn get_default_person() -> HashMap<String, String> {
    str_to_hashmap(DEFAULT_PERSON_STRING)
}
pub fn get_default_person2() -> HashMap<String, String> {
    str_to_hashmap(DEFAULT_PERSON2_STRING)
}
pub fn get_default_normal() -> HashMap<String, String> {
    str_to_hashmap(DEFAULT_NORMAL_STRING)
}