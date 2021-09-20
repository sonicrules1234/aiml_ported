//use serde::{Deserialize, Serialize};
use std::path::PathBuf;
//use std::path::Path;
//use serde_derive::{Serialize, Deserialize};
//use std::collections::HashMap;
//use sonicobject::{SonicObject, SonicPersistObject};
use crate::{sonicobject::{SonicObject, SonicPersistObject}, sonicobject};
use regex::Regex;
use sonic_serde_object::SonicSerdeObject;
//use serde_json::json;
//use json_value_merge::Merge;
//use anyhow::anyhow;
/*

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum SonicSerdeObject {
    Special(u8),
    Normal(String),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum SonicSerdeObjectError {
    NotSpecial(String),
    NotNormal(String),
}

impl SonicSerdeObject {
    pub fn get_special(&mut self) -> Result<u8, SonicSerdeObjectError> {
        match self {
            Self::Special(x) => return Ok(x.clone()),
            Self::Normal(_x) => return Err(SonicSerdeObjectError::NotSpecial("Not special".to_string()))
        }
    }
    pub fn get_normal(&mut self) -> Result<String, SonicSerdeObjectError> {
        match self {
            Self::Normal(x) => return Ok(x.clone()),
            Self::Special(_x) => return Err(SonicSerdeObjectError::NotNormal("Not normal".to_string()))
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum GeneralSonicSerdeObject {
    AimlHash(HashMap<SonicSerdeObject, GeneralSonicSerdeObject>),
    SonicSerdeObject(SonicSerdeObject),
    AimlList(Vec<GeneralSonicSerdeObject>),
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum GenSonicSerdeObjectError {
    NotAHash(String),
    KeyError(String),
    NotAKey(String),
    NotAList(String),
    IndexError(String),
}
//#[derive(Debug, Clone)]
//pub enum GenericError {
//    NoIdea(String),
//}

impl GeneralSonicSerdeObject {
    pub fn insert(&mut self, key: SonicSerdeObject, value: GeneralSonicSerdeObject) -> Result<Self, GenSonicSerdeObjectError> {
        match self {
            Self::AimlHash(x) => {
                let mut y = x.clone();
                y.insert(key, value);
                return Ok(GeneralSonicSerdeObject::AimlHash(y));
            },
            Self::SonicSerdeObject(_x) => {
                //return GeneralSonicSerdeObject::SonicSerdeObject(x.clone());
                return Err(GenSonicSerdeObjectError::NotAHash("Not a hash".to_string()));
            },
            Self::AimlList(_x) => {
                //return GeneralSonicSerdeObject::SonicSerdeObject(x.clone());
                return Err(GenSonicSerdeObjectError::NotAHash("Not a hash".to_string()));
            }
        }
    }
    pub fn contains_key(&mut self, key: SonicSerdeObject) -> Result<bool, GenSonicSerdeObjectError> {
        match self {
            Self::AimlHash(x) => {
                return Ok(x.contains_key(&key));
            },
            Self::SonicSerdeObject(_x) => {
                return Err(GenSonicSerdeObjectError::NotAHash("Not a hash".to_string()));
            },
            Self::AimlList(_x) => {
                return Err(GenSonicSerdeObjectError::NotAHash("Not a hash".to_string()));
            }
        }
    }
    pub fn get(&mut self, key: SonicSerdeObject) -> Result<Self, GenSonicSerdeObjectError> {
        match self {
            Self::AimlHash(x) => {
                match x.get(&key) {
                    Some(y) => {
                        return Ok(y.clone());
                    },
                    None => {
                        return Err(GenSonicSerdeObjectError::KeyError(format!("No such key: {:?}", &key)));
                    }
                }
            },
            Self::SonicSerdeObject(_x) => {
                return Err(GenSonicSerdeObjectError::NotAHash("Not a hash".to_string()));
            },
            Self::AimlList(_x) => {
                return Err(GenSonicSerdeObjectError::NotAHash("Not a hash".to_string()));
            }
        }
    }
    pub fn is_aiml_key(&mut self) -> bool {
        match self {
            Self::AimlHash(_x) => {
                return false;
            },
            Self::SonicSerdeObject(_x) => {
                return true;
            },
            Self::AimlList(_x) => {
                return false;
            }
        }
    }
    pub fn get_aiml_key_value(&mut self) -> Result<SonicSerdeObject, GenSonicSerdeObjectError>  {
        match self {
            Self::AimlHash(_x) => {
                return Err(GenSonicSerdeObjectError::NotAKey("Not a key".to_string()));
            },
            Self::SonicSerdeObject(x) => {
                return Ok(x.clone());
            },
            Self::AimlList(_x) => {
                return Err(GenSonicSerdeObjectError::NotAKey("Not a key".to_string()));
            }
        }
    }
    pub fn get_index(&mut self, index: usize) -> Result<GeneralSonicSerdeObject, GenSonicSerdeObjectError> {
        match self {
            Self::AimlList(x) => {
                if index < x.len() {
                    return Ok(x.clone()[index.clone()].clone());
                } else {
                    return Err(GenSonicSerdeObjectError::IndexError("Index not in range".to_string()))
                }
            },
            Self::SonicSerdeObject(_x) => {
                return Err(GenSonicSerdeObjectError::NotAList("Not a list".to_string()));
            },
            Self::AimlHash(_x) => {
                return Err(GenSonicSerdeObjectError::NotAList("Not a list".to_string()));
            }
        }
    }
}
impl AsRef<GeneralSonicSerdeObject> for GeneralSonicSerdeObject {
    fn as_ref(&self) -> &Self {
        &self
    }
}
impl AsMut<GeneralSonicSerdeObject> for GeneralSonicSerdeObject {
    fn as_mut(&mut self) -> &mut GeneralSonicSerdeObject {
        self
    }
}
//impl Index<&'_ SonicSerdeObject> for GeneralSonicSerdeObject {
//    type Output = ResultGeneralSonicSerdeObject;
//    fn index(&self, s: &SonicSerdeObject) -> &GeneralSonicSerdeObject {
//        match self {
//            Self::AimlHash(x) => {
//                return &x[s];
//            },
//            Self::SonicSerdeObject(x) => {
//                return ;
//            }
//    }
//}
*/
//#[derive(Deserialize, Serialize)]
#[derive(Clone)]
pub struct PatternMgr {
    
    dict_underscore: u8,
    dict_star: u8,
    dict_template: u8,
    dict_that: u8,
    dict_topic: u8,
    dict_bot_name: u8,
    pub root: SonicObject,
    template_count: u128,
    bot_name: String,
    //punctuation: String,
    //#[serde(with = "serde_regex")]
    punc_strip_re: Regex,
    //#[serde(with = "serde_regex")]
    whitespace_re: Regex,
}
//impl GeneralSonicSerdeObject {
//    pub fn insert(&mut self, key: SonicSerdeObject, value: GeneralSonicSerdeObject) -> crate::patternmgr::GeneralSonicSerdeObject {
//        let mut x = AimlHash()
//    }
//}
#[allow(unused_mut, unused_assignments)]
impl PatternMgr {
    pub fn new() -> Self {
        let mut dict_underscore = 0;
        let mut dict_star = 1;
        let mut dict_template = 2;
        let mut dict_that = 3;
        let mut dict_topic = 4;
        let mut dict_bot_name = 5;
        let mut template_count = 0;
        let mut root = SonicObject::new(SonicSerdeObject::new_map());
        let mut bot_name = "Nameless".to_string();
        let mut a = "[".to_string();
        let punctuation = regex::escape(r#"`~!@#$%^&*()-_=+[{]}\|;:'",<.>/?"#);
        a.push_str(punctuation.as_str());
        a.push_str("]");
        let mut punc_strip_re = Regex::new(a.as_str()).unwrap();
        let mut whitespace_re = Regex::new(r#"\s+"#).unwrap();
        Self {
            dict_underscore: dict_underscore,
            dict_star: dict_star,
            dict_template: dict_template,
            dict_that: dict_that,
            dict_topic: dict_topic,
            dict_bot_name: dict_bot_name,
            root: root,
            template_count: template_count,
            bot_name: bot_name,
            punc_strip_re: punc_strip_re,
            whitespace_re: whitespace_re,
        }
    }
    pub fn num_templates(&mut self) -> u128 {
        self.template_count
    }
    pub fn set_bot_name(&mut self, name: impl Into<String>) -> () {
        //self.bot_name = name.split(" ").collect::<Vec<&str>>().join("");
        self.bot_name = name.into().split_whitespace().collect::<Vec<&str>>().join(" ");
    }
    pub fn dump(&mut self) -> () {
        println!("{:?}", self.root);
    }
    pub fn save(&mut self, filename: PathBuf) {
        println!("{}", self.root.keys().len());
        let mut out_file = SonicPersistObject::new(filename);
        out_file.insert("templatecount", self.template_count.to_string());
        out_file.insert("botname", self.bot_name.clone());
        out_file.insert("root", self.root.clone().value);
        out_file.flush();
        drop(out_file);
    }
    pub fn restore(&mut self, filename: PathBuf) {
        let mut in_file = SonicPersistObject::new(filename);
        self.template_count = in_file.get("templatecount").value.as_str().unwrap().parse::<u128>().unwrap();
        self.bot_name = in_file.get("botname").value.as_str().unwrap().to_string();
        self.root = in_file.get("root");
        println!("{}", self.root.keys().len());
    }
    fn recursive_add(&self, key_value_vec: Vec<(SonicSerdeObject, SonicSerdeObject)>, root: SonicObject, depth: usize) -> SonicObject {
        //println!("kvv {:?}", key_value_vec);
        //println!("in depth {}", depth);
        //if key_value_vec[0].0 == SonicSerdeObject::U8(self.dict_that) {
        //    println!("that");
        //}
        //println!("key {:?}", key_value_vec[0].0);
        let mut new_key_val = key_value_vec.clone();
        let mut new_root = root;
        //let key = key_value_vec.clone()
        //let placenum = key_value_vec.len() - 1;
        if key_value_vec.len() == 1 {
            //let mut blah = SonicObject::new(SonicSerdeObject::new_map());
            //let mut new_root = root.clone();
            new_root.insert(key_value_vec[0].0.clone(), key_value_vec[0].1.clone());
            //println!("blah1 {:?}", blah);
            //println!("1");
            //println!("out depth {}", depth);
            return new_root;
        }
        new_key_val.remove(0);
        //println!("new key val len {:?}", new_key_val.len());
        if !new_root.contains(key_value_vec[0].0.clone()) {
            new_root.insert(key_value_vec[0].0.clone(), SonicSerdeObject::new_map());
        }
        //println!("2");
        let val = self.recursive_add(new_key_val, new_root.get(key_value_vec[0].0.clone()).unwrap(), depth + 1);
        //if val.contains(SonicSerdeObject::U8(self.dict_that)) {
        //    println!("out that");
        //}
        //let mut blah = SonicObject::new(SonicSerdeObject::new_map());
        
        //if depth > 0 {

            new_root.insert(key_value_vec[0].0.clone(), val.value);
            //println!("out depth {}", depth);
            return new_root
        //}
        //println!("out depth {}", depth);
        //val
    }
    /*
    fn merge_together(&self, root: SonicObject, key_value_vec: Vec<(serde_json::Value, serde_json::Value)>) -> SonicObject {
        let mut rootval = root.value.clone();
        let mut keylist: Vec<String> = Vec::new();
        for (key, _value) in key_value_vec.clone() {
            keylist.push(serde_json::to_value(key).unwrap().to_string());
        }
        for x in 0..(keylist.len()) {
            rootval.merge_in(format!("/{}", keylist.clone()[..(x + 1)].join("/")).as_str(), key_value_vec[x].1.clone());
        }
        SonicObject::new(rootval)
    }
    */
    pub fn add(&mut self, data: (String, String, String), template: SonicObject) -> () {
        let (pattern, that, topic) = data.clone();
        //if that.as_str() == "" {
        //    println!("empty that");
        //}
        //if pattern.as_str() == "HELLO" {
        //    println!("pattern that topic {:?}", data);
        //}
        
        let mut node = self.root.clone();
        //let mut key: serde_json::Value;
        if pattern.as_str() == "A LOT OF *" {
            //println!("aiml root {:?}", node.clone());
        }
        let mut key_value_vec: Vec<(SonicSerdeObject, SonicSerdeObject)> = Vec::new();
        //println!("add pattern {}", pattern);
        //println!("add that {}", that);
        //println!("add topic {}", topic);
        let mut key = SonicSerdeObject::U8(255);
        for word in pattern.split_whitespace() {
            //println!("word {}", word);
            key = SonicSerdeObject::String(word.to_string());
            if word == "_" {
                key = SonicSerdeObject::U8(self.dict_underscore);
            } else if word == "*" {
                key = SonicSerdeObject::U8(self.dict_star);
            } else if word == "BOT_NAME" {
                key = SonicSerdeObject::U8(self.dict_bot_name);
            }
            //println!("key1 {}", key);
            //let mut out = GeneralSonicSerdeObject::AimlHash(HashMap::new().into());
            if !node.contains(key.clone()) {
                node.insert(key.clone(), SonicSerdeObject::new_map());
                
                //key_value_vec.push((key.clone(), node.getvalue(key.clone()).unwrap()));
            }// else {
            //    node = node.get(key.clone()).unwrap();
            //}
            //self.root =
            key_value_vec.push((key.clone(), node.getvalue(key.clone()).unwrap()));
            node = node.get(key.clone()).unwrap();
            //let n = self.recursive_add(key_value_vec.clone());
            //println!("n keys {:?}", n.clone().keys()[0].clone());
            //self.root.insert(n.clone().keys()[0].clone(), n.clone().getvalue(n.clone().keys()[0].clone()).unwrap());
        }
        //let n = self.recursive_add(key_value_vec.clone());
        //println!("n keys {:?}", n.clone().keys()[0].clone());
        //self.root.insert(n.clone().keys()[0].clone(), n.clone().getvalue(n.clone().keys()[0].clone()).unwrap());
        if that.len() > 0 {
            //if pattern.as_str() == "HELLO" {
            //
            //}
            //let mut key_value_vec: Vec<(serde_json::Value, serde_json::Value)> = Vec::new();
            //let mut key_value_vec: Vec<(serde_json::Value, serde_json::Value)> = Vec::new();
            if !node.contains(SonicSerdeObject::U8(self.dict_that)) {

                node.insert(SonicSerdeObject::U8(self.dict_that), SonicSerdeObject::new_map());
                //key_value_vec.push((SonicSerdeObject::U8(self.dict_that), node.getvalue(SonicSerdeObject::U8(self.dict_that)).unwrap()));
            }
            //println!("dictthat {:?}", node.getvalue(SonicSerdeObject::U8(self.dict_that)).unwrap());
            key_value_vec.push((SonicSerdeObject::U8(self.dict_that), node.getvalue(SonicSerdeObject::U8(self.dict_that)).unwrap()));
            node = node.get(SonicSerdeObject::U8(self.dict_that)).unwrap();
            for word in that.split_whitespace() {
                //let key = word;
                key = SonicSerdeObject::String(word.to_string());
                if word == "_" {
                    key = SonicSerdeObject::U8(self.dict_underscore);
                } else if word == "*" {
                    key = SonicSerdeObject::U8(self.dict_star);
                }
                if !node.contains(key.clone()) {
                    node.insert(key.clone(), SonicSerdeObject::new_map());
                    //key_value_vec.push((key.clone(), node.getvalue(key.clone()).unwrap())); 
                }// else {
                //    node = node.get(key.clone()).unwrap();
                //}
                //let mut inter1 = 
                //node = node.get(key.clone()).unwrap();
                //node = SonicObject::new(json!({key.clone(): node.getvalue(key.clone()).unwrap()}));
                key_value_vec.push((key.clone(), node.getvalue(key.clone()).unwrap()));
                node = node.get(key.clone()).unwrap();
            }
            //let n = self.recursive_add(key_value_vec.clone());
            //self.root.insert(n.clone().keys()[0].clone(), n.clone().getvalue(n.clone().keys()[0].clone()).unwrap());
        }
        
        if topic.len() > 0 {
            //let mut key_value_vec: Vec<(serde_json::Value, serde_json::Value)> = Vec::new();
            if !node.contains(SonicSerdeObject::U8(self.dict_topic)) {
                node.insert(SonicSerdeObject::U8(self.dict_topic), SonicSerdeObject::new_map());
                //key_value_vec.push((SonicSerdeObject::U8(self.dict_topic), node.getvalue(SonicSerdeObject::U8(self.dict_topic)).unwrap()));
            }
            
            //let n = self.recursive_add(vec![(SonicSerdeObject::U8(self.dict_topic), SonicSerdeObject::new_map())]);
            //self.root.insert(n.clone().keys()[0].clone(), n.clone().getvalue(n.clone().keys()[0].clone()).unwrap());
            key_value_vec.push((SonicSerdeObject::U8(self.dict_topic), node.getvalue(SonicSerdeObject::U8(self.dict_topic)).unwrap()));
            node = node.get(SonicSerdeObject::U8(self.dict_topic)).unwrap();
            for word in topic.split_whitespace() {
                //let key = word;
                key = SonicSerdeObject::String(word.to_string());
                if word == "_" {
                    key = SonicSerdeObject::U8(self.dict_underscore);
                } else if word == "*" {
                    key = SonicSerdeObject::U8(self.dict_star);
                }
                if !node.contains(key.clone()) {
                    node.insert(key.clone(), SonicSerdeObject::new_map());
                    //key_value_vec.push((key.clone(), node.getvalue(key.clone()).unwrap()));
                }// else {
                //    node = node.get(key.clone()).unwrap();
                //}
                //node = node.get(key.clone()).unwrap();
                key_value_vec.push((key.clone(), node.getvalue(key.clone()).unwrap()));
                node = node.get(key.clone()).unwrap();
                //node = SonicObject::new(json!({key.clone(): node.getvalue(key.clone()).unwrap()}));
            }
            //let n = self.recursive_add(key_value_vec.clone());
            //self.root.insert(n.clone().keys()[0].clone(), n.clone().getvalue(n.clone().keys()[0].clone()).unwrap());
        }
        
        if !node.contains(SonicSerdeObject::U8(self.dict_template)) {
            self.template_count = self.template_count + 1;
        }
        node.insert(SonicSerdeObject::U8(self.dict_template), template.value.clone());
        //self.root.insert(SonicSerdeObject::U8(self.dict_template), template.value.clone());
        key_value_vec.push((SonicSerdeObject::U8(self.dict_template), template.value.clone()));
        if !self.root.contains(key_value_vec[0].0.clone()) {
            self.root.insert(key_value_vec[0].0.clone(), SonicSerdeObject::new_map());
        }
        self.root.insert(key_value_vec[0].0.clone(), self.recursive_add(key_value_vec.clone()[1..].to_vec(), self.root.get(key_value_vec[0].0.clone()).unwrap(), 0).value);
        //self.root = self.recursive_add(key_value_vec.clone(), self.root.clone(), 0);
        //println!("n {:?}", n);
        //println!("nkeys {:?}", n.keys());
        //for key in n.clone().keys() {
        //    self.root.insert(key.clone(), n.getvalue(key.clone()).unwrap());
        //}
        //self.root.insert(n.clone().keys()[0].clone(), n.clone().getvalue(n.clone().keys()[0].clone()).unwrap());
        //self.root.insert(SonicSerdeObject::U8(self.dict_template), template.value);
        //self.root = node.clone();
        //println!("root {:?}", self.root.clone());
        //let output = self.merge_together(self.root.clone(), key_value_vec).clone();
        //self.root = output.clone();
        //drop(output);
        //if pattern.as_str() == "HELLO" {
            //println!("node template {:?}", node.get(SonicSerdeObject::U8(self.dict_template)).unwrap());
            //println!("root key len {}", self.root.keys().len());
        //}
        //if self.root.keys().len() == 1154 {
            //println!("{}", self.root.keys().len());
        //}

    }
    //pub fn match(&mut self, pattern: String, that: String, topic: String) -
    pub fn _match(&mut self, words: Vec<String>, that_words: Vec<String>, topic_words: Vec<String>, root: SonicObject) -> Result<(Option<SonicObject>, Option<SonicObject>), sonicobject::SonicObjectError> {
        //println!("wordlen {}, thatwords {:?}, topicwords {:?}", words.len(), that_words, topic_words);
        let mut pattern: Option<SonicObject>;
        let mut template: Option<SonicObject>;
        //let mut root = root.clone();
        //println!("words {:?}", words);
        if words.len() == 0 {
            pattern = Some(SonicObject::new(SonicSerdeObject::new_vec()));
            template = None;
            if that_words.len() > 0 {
                //println!("rootkeys {:?}", root.keys());
                match root.get(SonicSerdeObject::U8(self.dict_that)) {
                    Ok(_z) => {
                        match self._match(that_words, Vec::new(), topic_words, root.get(SonicSerdeObject::U8(self.dict_that)).unwrap()) {
                            Ok((p, t)) => {
                                //if p.is_none() {
                                //    return Err(anyhow!("Error"));
                                //};
                                pattern = p;
                                template = t;
                                if pattern.is_some() {
                                    
                                    let mut temppattern = SonicObject::new(SonicSerdeObject::new_vec());
                                    temppattern.push(SonicSerdeObject::U8(self.dict_that));
                                    for pat in pattern.unwrap().clone() {
                                        temppattern.push(pat.clone().value);
                                    }
                                    pattern = Some(temppattern.clone());
                                }// else {
                                //    pattern = p.unwrap();
                                //}
                                
                            },
                            Err(sonicobject::SonicObjectError::KeyError(_y)) => {
                                pattern = Some(SonicObject::new(SonicSerdeObject::new_vec()));
                                template = None;
                            },
                            Err(e) => {
                                panic!("{}", e);
                            }
                        };
                    },
                    Err(sonicobject::SonicObjectError::KeyError(_e)) => {
                        pattern = Some(SonicObject::new(SonicSerdeObject::new_vec()));
                        template = None;
                        //eprintln!("{}", e);
                    },
                    Err(_e) => {
                        ();
                    }
                } 
                
            } else if topic_words.len() > 0 {
                match root.get(SonicSerdeObject::U8(self.dict_topic)) {
                    Ok(_z) => {
                        match self._match(topic_words, Vec::new(), Vec::new(), root.get(SonicSerdeObject::U8(self.dict_topic)).unwrap()) {
                            Ok((p, t)) => {
                                //if p.is_none() {
                                //    return Err(anyhow!("Error"));
                                //};
                                pattern = p;
                                template = t;
                                if pattern.is_some() {
                                    
                                    let mut temppattern = SonicObject::new(SonicSerdeObject::new_vec());
                                    temppattern.push(SonicSerdeObject::U8(self.dict_topic));
                                    for pat in pattern.unwrap().clone() {
                                        temppattern.push(pat.clone().value);
                                    }
                                    pattern = Some(temppattern.clone());
                                }// else {
                                //    pattern = p.unwrap();
                                //}
                                
                            },
                            Err(sonicobject::SonicObjectError::KeyError(_y)) => {
                                pattern = Some(SonicObject::new(SonicSerdeObject::new_vec()));
                                template = None;
                            },
                            Err(e) => {
                                panic!("{}", e);
                            }
                        };
                    },
                    Err(sonicobject::SonicObjectError::KeyError(_e)) => {
                        pattern = Some(SonicObject::new(SonicSerdeObject::new_vec()));
                        template = None;
                        //eprintln!("{}", e);
                    },
                    Err(_e) => {
                        ();
                    }
                }
                
            }
            if template.is_none() {
                //println!("template is none");
                pattern = Some(SonicObject::new(SonicSerdeObject::new_vec()));
                template = match root.get(SonicSerdeObject::U8(self.dict_template)) {
                    Ok(x) => Some(x),
                    Err(sonicobject::SonicObjectError::KeyError(_x)) => {
                        //println!("didnt find");
                        None
                    },
                    Err(e) => panic!("{}", e),
                }
            }
            //println!("temp {:?}", template);
            return Ok((pattern, template));
        }
        let mut first: String = (&words[0]).to_string();
        let mut suffix: Vec<String> = words[1..].to_vec();
        //println!("first {:?} suffix {:?}", first.clone(), suffix.clone());
        // May implement following python:
        /*
        # Check underscore.
        # Note: this is causing problems in the standard AIML set, and is
        # currently disabled.
        if self._UNDERSCORE in root:
            # Must include the case where suf is [] in order to handle the case
            # where a * or _ is at the end of the pattern.
            for j in range(len(suffix)+1):
                suf = suffix[j:]
                pattern, template = self._match(suf, thatWords, topicWords, root[self._UNDERSCORE])
                if template is not None:
                    newPattern = [self._UNDERSCORE] + pattern
                    return (newPattern, template)
        */
        //println!("root keys {:?}", root.keys());
        
        if root.contains(SonicSerdeObject::U8(self.dict_underscore)) {
            //println!("contains _");
            for j in 0..(suffix.len() + 1) {
                let mut suf = suffix[j..].to_vec();
                //println!("root underscore {:?}", root.keys());
                let z = self._match(suf, that_words.clone(), topic_words.clone(), root.clone().get(SonicSerdeObject::U8(self.dict_underscore)).unwrap()).unwrap();
                pattern = z.0;
                template = z.1;
                if template.is_some() {
                    //println!("tempate is some");
                    let mut newpattern = SonicObject::new(SonicSerdeObject::new_vec());
                    newpattern.push(SonicSerdeObject::U8(self.dict_underscore));
                    for pat in pattern.unwrap() {
                        //println!("pat {:?}", pat.value);
                        newpattern.push(pat.value)
                    }
                    return Ok((Some(newpattern), template));
                }
            }
        }
        
        //println!("got past _");
        if root.contains(SonicSerdeObject::String(first.clone())) {
            //println!("contains first");
            let z = self._match(suffix.clone(), that_words.clone(), topic_words.clone(), root.get(SonicSerdeObject::String(first.clone())).unwrap()).unwrap();
            pattern = z.0;
            template = z.1;
            if template.is_some() {
                let mut newpattern = SonicObject::new(SonicSerdeObject::new_vec());
                newpattern.push(SonicSerdeObject::String(first.clone()));
                for pat in pattern.unwrap() {
                    newpattern.push(pat.value)
                }
                return Ok((Some(newpattern), template));
            }
        }
        //println!("past first");
        if root.contains(SonicSerdeObject::U8(self.dict_bot_name)) && first == self.bot_name {
            let z = self._match(suffix.clone(), that_words.clone(), topic_words.clone(), root.get(SonicSerdeObject::U8(self.dict_bot_name)).unwrap()).unwrap();
            pattern = z.0;//.unwrap();
            let template = z.1;
            if template.is_some() {
                let mut newpattern = SonicObject::new(SonicSerdeObject::new_vec());
                newpattern.push(SonicSerdeObject::String(first.clone()));
                for pat in pattern.unwrap() {
                    newpattern.push(pat.value)
                }
                return Ok((Some(newpattern), template));
            }
        }
        //println!("past bot_name");
        if root.contains(SonicSerdeObject::U8(self.dict_star)) {
            //println!("contains dict_star");
            //let z = self._match(suffix.clone(), that_words.clone(), topic_words.clone(), root.get(self.dict_star).unwrap()).unwrap();
            //pattern = z.0.unwrap();
            //template = z.1;
            //if template.is_some() {
            //    let mut newpattern = SonicObject::new(SonicSerdeObject::new_vec());
            //    newpattern.push(first.clone());
            //    for pat in pattern {
            //        newpattern.push(pat)
            //    }
            //    return Ok((Some(newpattern), template))
            //}
            //println!("contains star");
            for j in 0..(suffix.len() + 1) {
                let mut suf = suffix[j..].to_vec();
                let z = self._match(suf, that_words.clone(), topic_words.clone(), root.clone().get(SonicSerdeObject::U8(self.dict_star)).unwrap()).unwrap();
                pattern = z.0;
                template = z.1;
                if template.is_some() {
                    let mut newpattern = SonicObject::new(SonicSerdeObject::new_vec());
                    newpattern.push(SonicSerdeObject::U8(self.dict_star));
                    for pat in pattern.unwrap() {
                        newpattern.push(pat.value)
                    }
                    return Ok((Some(newpattern), template));
                }
            }
        }
        //println!("past dict_star");
        Ok((None, None))
    }
    pub fn r#match(&mut self, pattern: String, that: String, topic: String) -> Option<SonicObject> {
        let pattern = pattern.clone();
        let mut that = that.clone();
        let mut topic = topic.clone();

        if pattern.len() == 0 {
            return None;
        }
        //println!("pat input '{}'", pattern);
        let mut input: String = pattern.to_uppercase();
        input = self.punc_strip_re.replace_all(input.as_str(), " ").to_string();
        //println!("punct strip input '{}'", input);
        if that.trim().to_string() == String::new() {
            that = "ULTRABOGUSDUMMYTHAT".to_string();
        }
        let mut that_input: String = that.to_uppercase().to_string();
        that_input = self.punc_strip_re.replace_all(that_input.as_str(), " ").to_string();
        that_input = self.whitespace_re.replace_all(that_input.as_str(), " ").to_string();
        //println!("that_input {}", that_input);
        //println!("that input whitespace split {:?}", that_input.split_whitespace().map(|inword| inword.to_string()).collect::<Vec<String>>());
        if topic.trim().to_string() == String::new() {
            topic = "ULTRABOGUSDUMMYTOPIC".to_string();
        }
        let mut topic_input: String = topic.to_uppercase().to_string();
        topic_input = self.punc_strip_re.replace_all(topic_input.as_str(), " ").to_string();
        //let mut arg1 = 
        //println!("{:?}", input.split(" ").map(|inword| inword.to_string()).collect::<Vec<String>>());
        let (_patmatch, template) = self._match(input.split_whitespace().map(|inword| inword.to_string()).collect(), that_input.split_whitespace().map(|inword| inword.to_string()).collect(), topic_input.split_whitespace().map(|inword| inword.to_string()).collect(), self.root.clone()).unwrap();
        template
    }
    pub fn star(&mut self, star_type: String, pattern: String, that: String, topic: String, index: usize) -> String {
        let mut pattern = pattern.clone();
        //println!("index {}", index);
        let mut that = that.clone();
        let mut topic = topic.clone();
        let mut input: String = pattern.clone().to_uppercase().to_string();
        input = self.punc_strip_re.replace_all(input.as_str(), " ").to_string();
        input = self.whitespace_re.replace_all(input.as_str(), " ").to_string();
        if that.trim().to_string() == String::new() {
            that = "ULTRABOGUSDUMMYTOPIC".to_string();
        }
        let mut that_input: String = that.to_uppercase().to_string();
        that_input = self.punc_strip_re.replace_all(that_input.as_str(), " ").to_string();
        that_input = self.whitespace_re.replace_all(that_input.as_str(), " ").to_string();
        if topic.trim().to_string() == String::new() {
            topic = "ULTRABOGUSDUMMYTOPIC".to_string();
        }
        let mut topic_input: String = topic.to_uppercase().to_string();
        topic_input = self.punc_strip_re.replace_all(topic_input.as_str(), " ").to_string();
        topic_input = self.whitespace_re.replace_all(topic_input.as_str(), " ").to_string();
        let (mut pat_match, mut template) = self._match(input.split_whitespace().map(|inword| inword.to_string()).collect(), that_input.split_whitespace().map(|inword| inword.to_string()).collect(), topic_input.split_whitespace().map(|inword| inword.to_string()).collect(), self.root.clone()).unwrap();
        let mut words: Option<Vec<String>>;
        if template.is_none() {
            return String::new();
        }
        words = None;
        let vec_pat_match: Vec<SonicSerdeObject> = pat_match.unwrap().collectvec().into_iter().map(|x| x.value.into()).collect();
        if star_type.as_str() == "star" {
            pat_match = Some(SonicObject::new(vec_pat_match.clone()[..vec_pat_match.clone().iter().position(|x| *x == SonicSerdeObject::U8(self.dict_that)).unwrap()].to_vec()));
            words = Some(input.split_whitespace().map(|inword| inword.to_string()).collect());
        } else if star_type.as_str() == "thatstar" {
            pat_match = Some(SonicObject::new(vec_pat_match.clone()[(vec_pat_match.clone().iter().position(|x| *x == SonicSerdeObject::U8(self.dict_that)).unwrap() + 1)..(vec_pat_match.clone().iter().position(|x| *x == SonicSerdeObject::U8(self.dict_topic)).unwrap())].to_vec()));
            words = Some(that_input.split_whitespace().map(|inword| inword.to_string()).collect());
        } else if star_type.as_str() == "topicstar" {
            pat_match = Some(SonicObject::new(vec_pat_match.clone()[(vec_pat_match.clone().iter().position(|x| *x == SonicSerdeObject::U8(self.dict_topic)).unwrap() + 1)..].to_vec()));
            words = Some(topic_input.split_whitespace().map(|inword| inword.to_string()).collect());
        } else {
            panic!("Invalid star type: \"{}\"", star_type);
        }
        //println!("words {:?}", words);
        //println!("pat_match {:?}", pat_match);
        let mut found_right_star = false;
        let mut start_: usize = 0;
        let mut end: usize = 0;
        let mut j = 0;
        let mut num_stars = 0;
        let mut k = 0;
        //let mut i = 0;
        //while i < words.clone().unwrap().len() {
        for i in 0..(words.clone().unwrap().len()) {
            //if !istart {
            //    i = i + 1;
            //}
            //istart = false;
            if i < k {
                continue;
            }
            if j == pat_match.clone().unwrap().value.as_vec().unwrap().len() { //.unwrap().collect::<Vec<SonicObject>>().len() {
                break;
            }
            if !found_right_star {
                if vec![SonicSerdeObject::U8(self.dict_star), SonicSerdeObject::U8(self.dict_underscore)].contains(&pat_match.clone().unwrap().getindexvalue(j).unwrap()) {
                    num_stars = num_stars + 1;
                    if num_stars as usize == index {
                        found_right_star = true;
                    }
                    start_ = i.clone();
                    //k = i.clone();
                    //while k < words.clone().unwrap().len() {
                    for k in i..(words.clone().unwrap().len()) {
                        if j + 1 == pat_match.clone().unwrap().collect::<Vec<SonicObject>>().len() {
                            end = words.clone().unwrap().len();
                            break;
                        }
                        if pat_match.as_ref().unwrap().getindexvalue(j + 1).unwrap().as_str().unwrap().to_string() == words.as_ref().unwrap()[k] {
                            end = k - 1;
                            //i = k.clone();
                            break;
                        }
                        
                    }
                }
                if found_right_star {
                    break;
                }
            
            }
            j = j + 1;
            //i += i;
        }
        if found_right_star {
            if star_type.as_str() == "star" {
                //println!("start, end {} {}", start_, end);
                //println!("pattern {}", pattern);
                let a = pattern.split_whitespace().map(|inword| inword.to_string()).collect::<Vec<String>>();//[start_..(end + 1)].join(" ");
                let alen = a.len();
                if end + 1 >= alen {
                    end = alen - 2;
                }
                //println!("return, end '{}', {}", a[start_..(end + 1)].join(" "), end + 1);
                return a[start_..=(end + 1)].join(" ");
            } else if star_type.as_str() == "thatstar" {
                let a = that.split_whitespace().map(|inword| inword.to_string()).collect::<Vec<String>>();//[start_..(end + 1)].join(" ");
                //return a;
                let alen = a.len();
                if end + 1 >= alen {
                    end = alen - 2;
                }
                return a[start_..=(end + 1)].join(" ");
            } else if star_type.as_str() == "thatstar" {
                let a = topic.split_whitespace().map(|inword| inword.to_string()).collect::<Vec<String>>();//[start_..(end + 1)].join(" ");
                //println!("{}", a);
                //return a;
                let alen = a.len();
                if end + 1 >= alen {
                    end = alen - 2;
                }
                return a[start_..=(end + 1)].join(" ");
            } else {
                return String::new();
            }
        } else {
            return String::new();
        }
    }
}
