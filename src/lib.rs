use std::collections::HashMap;
pub mod aimlparser;
pub mod defaultsubs;
pub mod kernel;
pub mod patternmgr;
pub mod sonicobject;
pub mod utils;
pub mod wordsub;
pub fn is_alpha(word: String) -> bool {
    for c in word.chars() {
        if !c.is_alphabetic() {
            return false;
        }
    }
    return true;
}
pub fn attr_list_to_hashmap(attr_list: &[roxmltree::Attribute]) -> HashMap<String, String> {
    let mut x: HashMap<String, String> = HashMap::new();
    for attr in attr_list {
        x.insert(attr.name().to_string(), attr.value().to_string());
    }
    x
}
pub fn cap_words(instring: String) -> String {
    let in_chars = instring.chars().collect::<Vec<char>>();
    let mut out_chars: Vec<char> = Vec::new();
    for char_place in 0..in_chars.len() {
        //let outchar: char;
        if char_place == 0 {
            let char_parts = in_chars[char_place].to_uppercase().collect::<Vec<char>>();
            for char_part in char_parts {
                out_chars.push(char_part);
            }
        } else if in_chars[char_place - 1] == ' ' {
            //" ".chars().last().unwrap() {
            let char_parts = in_chars[char_place].to_uppercase().collect::<Vec<char>>();
            for char_part in char_parts {
                out_chars.push(char_part);
            }
        } else {
            let char_parts = in_chars[char_place];
            out_chars.push(char_parts);
        }
    }
    out_chars.iter().collect()
}
