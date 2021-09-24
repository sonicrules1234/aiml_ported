use std::collections::HashMap;
use crate::sonicobject::SonicObject;
use sonic_serde_object::SonicSerdeObject;
pub struct AimlParser {
    state_outside_aiml: u8,
    state_inside_aiml: u8,
    state_inside_category: u8,
    state_inside_pattern: u8,
    state_after_pattern: u8,
    state_inside_that: u8,
    state_after_that: u8,
    state_inside_template: u8,
    state_after_template: u8,
    pub categories: HashMap<(String, String, String), SonicObject>,
    state: u8,
    version: String,
    _namespace: String,
    forward_compatible_mode: bool,
    current_pattern: String,
    current_that: String,
    current_topic: String,
    inside_topic: bool,
    current_unknown: String,
    skip_current_category: bool,
    num_parse_errors: u32,
    found_default_li_stack: Vec<bool>,
    whitespace_behavior_stack: Vec<String>,
    elem_stack: SonicObject,
    validate_info: HashMap<String, (Vec<String>, Vec<String>, bool)>,
}

#[derive(Debug)]
pub enum AimlParserError {
    Error(String),
}
#[allow(unused_mut, unused_assignments)]
impl AimlParser {
    pub fn new() -> Self {
        const STATE_OUTSIDE_AIML: u8 = 0;
        const STATE_INSIDE_AIML: u8 = 1;
        const STATE_INSIDE_CATEGORY: u8 = 2;
        const STATE_INSIDE_PATTERN: u8 = 3;
        const STATE_AFTER_PATTERN: u8 = 4;
        const STATE_INSIDE_THAT: u8 = 5;
        const STATE_AFTER_THAT: u8 = 6;
        const STATE_INSIDE_TEMPLATE: u8 = 7;
        const STATE_AFTER_TEMPLATE: u8 = 8;
        let categories: HashMap<(String, String, String), SonicObject> = HashMap::new();
        let state = STATE_OUTSIDE_AIML.clone();
        let version = String::new();
        let namespace = String::new();
        let forward_compatible_mode = false;
        let current_pattern = String::new();
        let current_that = String::new();
        let current_topic = String::new();
        let inside_topic = false;
        let mut current_unknown = String::new();
        let mut skip_current_category = false;
        let mut num_parse_errors = 0;
        let mut found_default_li_stack: Vec<bool> = Vec::new();
        let mut whitespace_behavior_stack = vec!["default".to_string()];
        let mut elem_stack: SonicObject = SonicObject::new(SonicSerdeObject::new_vec());
        let mut validate_info: HashMap<String, (Vec<String>, Vec<String>, bool)> = HashMap::new();
        validate_info.insert("bot".to_string(), ( vec!["name".to_string()], Vec::<String>::new(), false ));
        validate_info.insert("condition".to_string(), ( Vec::<String>::new(), vec!["name".to_string(), "value".to_string()], true ));
        validate_info.insert("date".to_string(), ( Vec::<String>::new(), Vec::<String>::new(), false ));
        validate_info.insert("formal".to_string(), ( Vec::<String>::new(), Vec::<String>::new(), true ));
        validate_info.insert("gender".to_string(), ( Vec::<String>::new(), Vec::<String>::new(), true ));
        validate_info.insert("get".to_string(), ( vec!["name".to_string()], Vec::<String>::new(), false ));
        validate_info.insert("gossip".to_string(), ( Vec::<String>::new(), Vec::<String>::new(), true ));
        validate_info.insert("id".to_string(), ( Vec::<String>::new(), Vec::<String>::new(), false ));
        validate_info.insert("input".to_string(), ( Vec::<String>::new(), vec!["index".to_string()], false ));
        validate_info.insert("javascript".to_string(), ( Vec::<String>::new(), Vec::<String>::new(), true ));
        validate_info.insert("learn".to_string(), ( Vec::<String>::new(), Vec::<String>::new(), true ));
        validate_info.insert("li".to_string(), ( Vec::<String>::new(), vec!["name".to_string(), "value".to_string()], true ));
        validate_info.insert("lowercase".to_string(), ( Vec::<String>::new(), Vec::<String>::new(), true ));
        validate_info.insert("person".to_string(), ( Vec::<String>::new(), Vec::<String>::new(), true ));
        validate_info.insert("person2".to_string(), ( Vec::<String>::new(), Vec::<String>::new(), true ));
        validate_info.insert("random".to_string(), ( Vec::<String>::new(), Vec::<String>::new(), true ));
        validate_info.insert("sentence".to_string(), ( Vec::<String>::new(), Vec::<String>::new(), true ));
        validate_info.insert("set".to_string(), ( vec!["name".to_string()], Vec::<String>::new(), true));
        validate_info.insert("size".to_string(), ( Vec::<String>::new(), Vec::<String>::new(), false ));
        validate_info.insert("sr".to_string(), ( Vec::<String>::new(), Vec::<String>::new(), false ));
        validate_info.insert("srai".to_string(), ( Vec::<String>::new(), Vec::<String>::new(), true ));
        validate_info.insert("star".to_string(), ( Vec::<String>::new(), vec!["index".to_string()], false ));
        validate_info.insert("system".to_string(), ( Vec::<String>::new(), Vec::<String>::new(), true ));
        validate_info.insert("template".to_string(), ( Vec::<String>::new(), Vec::<String>::new(), true ));
        validate_info.insert("that".to_string(), ( Vec::<String>::new(), vec!["index".to_string()], false ));
        validate_info.insert("thatstar".to_string(), ( Vec::<String>::new(), vec!["index".to_string()], false ));
        validate_info.insert("think".to_string(), ( Vec::<String>::new(), Vec::<String>::new(), true ));
        validate_info.insert("topicstar".to_string(), ( Vec::<String>::new(), vec!["index".to_string()], false ));
        validate_info.insert("uppercase".to_string(), ( Vec::<String>::new(), Vec::<String>::new(), true ));
        validate_info.insert("version".to_string(), ( Vec::<String>::new(), Vec::<String>::new(), false ));

        Self {
            state_outside_aiml: STATE_OUTSIDE_AIML,
            state_inside_aiml: STATE_INSIDE_AIML,
            state_inside_category: STATE_INSIDE_CATEGORY,
            state_inside_pattern: STATE_INSIDE_PATTERN,
            state_after_pattern: STATE_AFTER_PATTERN,
            state_inside_that: STATE_INSIDE_THAT,
            state_after_that: STATE_AFTER_THAT,
            state_inside_template: STATE_INSIDE_TEMPLATE,
            state_after_template: STATE_AFTER_TEMPLATE,
            categories: categories,
            state: state,
            version: version,
            _namespace: namespace,
            forward_compatible_mode: forward_compatible_mode,
            current_pattern: current_pattern,
            current_that: current_that,
            current_topic: current_topic,
            inside_topic: inside_topic,
            current_unknown: current_unknown,
            skip_current_category: skip_current_category,
            num_parse_errors: num_parse_errors,
            found_default_li_stack: found_default_li_stack,
            whitespace_behavior_stack: whitespace_behavior_stack,
            elem_stack: elem_stack,
            validate_info: validate_info,
        }
    }
    pub fn location(&mut self) -> String {
        format!("{}", self.state)
    }
    pub fn get_num_errors(&mut self) -> u32 {
        self.num_parse_errors
    }
    pub fn push_whitespace_behavior(&mut self, attr: HashMap<String, String>) -> Result<(), AimlParserError> {
        if self.whitespace_behavior_stack.len() > 0 {
            if attr.contains_key(&"xml:space".to_string()) {
                if attr[&"xml:space".to_string()] == "default".to_string() || attr[&"xml:space".to_string()] == "preserve".to_string() {
                    self.whitespace_behavior_stack.push(attr.clone()[&"xml:space".to_string()].clone());
                } else {
                    return Err(AimlParserError::Error(format!("Invalid value for xml:space attribute {}", self.location())));
                }
            } else {
                self.whitespace_behavior_stack.push(self.whitespace_behavior_stack.last().unwrap().to_string());
            }
        }
        Ok(())
    }
    pub fn start_element_ns(&mut self, name: (String, String), qname: String, attr: HashMap<String, String>) {
        println!("QNAME: {:?}", qname);
        println!("NAME: {:?}", name);
        let (_uri, elem) = name;
        //if elem.as_str() == "bot" {
        //    println!("name: ")
        //}
        self.start_element(elem, attr);
    }
    pub fn start_element(&mut self, name: String, attr: HashMap<String, String>) {
        //println!("name {}", name.clone());
        if self.current_unknown.as_str() != "" {
            return;
        }
        if self.skip_current_category {
            return;
        }
        match self._start_element(name, attr) {
            Ok(_x) => (),
            Err(AimlParserError::Error(x)) => {
                eprintln!("PARSE ERROR: {}", x);
                self.num_parse_errors = self.num_parse_errors + 1;
                if self.state >= self.state_inside_category {
                    self.skip_current_category = true;
                }
            },
        }
    }
    pub fn _start_element(&mut self, name: String, attr: HashMap<String, String>) -> Result<(), AimlParserError> {
        //println!("_start name {}", name.clone());
        if name.as_str() == "aiml" {
            if self.state != self.state_outside_aiml {
                return Err(AimlParserError::Error(format!("Unexpected <aiml> tag {}", self.location())));
            }
            self.state = self.state_inside_aiml;
            self.inside_topic = false;
            self.current_topic = String::new();
            if attr.contains_key(&"version".to_string()) {
                self.version = attr[&"version".to_string()].clone();
            } else {
                self.version = "1.0".to_string();
            }
            self.forward_compatible_mode = self.version.as_str() != "1.0.1";
            self.push_whitespace_behavior(attr)?;
        } else if self.state == self.state_outside_aiml {
            return Ok(());
        } else if name.as_str() == "topic" {
            if (self.state != self.state_inside_aiml) || self.inside_topic {
                return Err(AimlParserError::Error(format!("Unexpected <topic> tag {}", self.location())));
            }
            if attr.contains_key(&"name".to_string()) {
                self.current_topic = attr["name"].to_string();
            } else {
                return Err(AimlParserError::Error(format!("Required \"name\" attribute missing in topic element {}", self.location())));
            }
            self.inside_topic = true;
            
        } else if name.as_str() == "category" {
            if self.state != self.state_inside_aiml {
                return Err(AimlParserError::Error(format!("Unexpected <category> tag {}", self.location())));
            }
            self.state = self.state_inside_category;
            self.current_pattern = String::new();
            self.current_that = String::new();
            if !self.inside_topic {
                self.current_topic = "*".to_string();
            }
            self.elem_stack = SonicObject::new(SonicSerdeObject::new_vec());
            self.push_whitespace_behavior(attr)?;
        } else if name.as_str() == "pattern" {
            if self.state != self.state_inside_category {
                return Err(AimlParserError::Error(format!("Unexpected <pattern> tag {}", self.location())));
            }
            self.state = self.state_inside_pattern;
        } else if name.as_str() == "that" && self.state == self.state_after_pattern {
            self.state = self.state_inside_that;
        } else if name.as_str() == "template" {
            if !vec![self.state_after_pattern, self.state_after_that].contains(&self.state) {
                return Err(AimlParserError::Error(format!("Unexpected <template> tag {}", self.location())));
            }
            if self.state == self.state_after_pattern {
                self.current_that = "*".to_string();
            }
            self.state = self.state_inside_template;
            //let mut s:  = 
            let mut empty_val = SonicSerdeObject::new_vec();
            empty_val.push("template");
            empty_val.push(SonicSerdeObject::new_map());
            self.elem_stack.push(empty_val);
            //self.elem_stack.push(json!(["template".to_string(), {}]));
            self.push_whitespace_behavior(attr)?;
        } else if self.state == self.state_inside_pattern {
            if name.as_str() == "bot" && attr.contains_key("name") {
                if attr["name"] == "name".to_string() {
                    self.current_pattern = format!("{} BOT_NAME ", self.current_pattern);
                } else {
                    return Err(AimlParserError::Error(format!("Unexpected <{}> tag {}", name, self.location())));
                }
            } else {
                return Err(AimlParserError::Error(format!("Unexpected <{}> tag {}", name, self.location())));
            }
        } else if self.state == self.state_inside_that {
            if name.as_str() == "bot" && attr.contains_key("name") {
                if attr["name"] == "name".to_string() {
                    self.current_that = format!("{} BOT_NAME ", self.current_that);
                } else {
                    return Err(AimlParserError::Error(format!("Unexpected <{}> tag {}", name, self.location())));
                }
            } else {
                return Err(AimlParserError::Error(format!("Unexpected <{}> tag {}", name, self.location())));
            }
        } else if self.state == self.state_inside_template && self.validate_info.contains_key(&name) {
            self.validate_elem_start(name.clone(), attr.clone(), self.version.clone())?;
            self.elem_stack.push(SonicSerdeObject::Vec(vec![name.clone().into(), attr.clone().into()]));
            self.push_whitespace_behavior(attr)?;
            if name.as_str() == "condition" {
                self.found_default_li_stack.push(false);
            }
        } else {
            if self.forward_compatible_mode {
                self.current_unknown = name.clone();
                //return Ok(());
            } else {
                return Err(AimlParserError::Error(format!("Unexpected <{}> tag {}", name, self.location())));
            }
        }
        Ok(())
    }
    pub fn characters(&mut self, ch: String) {
        if self.state == self.state_outside_aiml {
            return;
        }
        if self.current_unknown != String::new() {
            return;
        }
        if self.skip_current_category {
            return;
        }
        match self._characters(ch) {
            Ok(_x) => (),
            Err(AimlParserError::Error(x)) => {
                eprintln!("PARSE ERROR: {}", x);
                self.num_parse_errors = self.num_parse_errors + 1;
                if self.state == self.state_inside_category {
                    self.skip_current_category = true;
                }
            }
        }
    }
    pub fn _characters(&mut self, ch: String) -> Result<(), AimlParserError> {
        let mut text = ch.clone();
        //println!("text {}", text);
        let mut parent: String;
        let mut parent_attr: HashMap<String, String> = HashMap::new();
        if self.state == self.state_inside_pattern {
            self.current_pattern.push_str(text.as_str());
        } else if self.state == self.state_inside_that {
            self.current_that.push_str(text.as_str());
        } else if self.state == self.state_inside_template {
            if self.elem_stack.clone().collect::<Vec<SonicObject>>().len() != 0 {
                parent = self.elem_stack.clone().last().unwrap().getindexvalue(0).unwrap().as_str().unwrap().to_string();
                let mut x = self.elem_stack.clone().last().unwrap().getindexvalue(1).unwrap();
                let mut y = x.as_map().unwrap();
                for key in y.keys() {
                    parent_attr.insert(key.as_str().unwrap().to_string(), y.get(key).unwrap().as_str().unwrap().to_string());
                }
                let (_required, _optional, can_be_parent) = self.validate_info.clone()[&parent].clone();
                if text.as_str() == "HELLO" {
                    println!("parent {:?}, parent_attr {:?}", parent, parent_attr);
                }
                let mut non_block_style_condition = parent.as_str() == "condition" && !(parent_attr.contains_key("name") && parent_attr.contains_key("value"));
                if !can_be_parent {
                    return Err(AimlParserError::Error(format!("Unexpected text inside <{}> element {}", parent, self.location())));
                } else if parent.as_str() == "random" || non_block_style_condition {
                    if text.trim().len() == 0 {
                        return Ok(());
                    } else {
                        return Err(AimlParserError::Error(format!("Unexpected text inside <{}> element {}", parent, self.location())));
                    }
                }
            } else {
                return Err(AimlParserError::Error(format!("Element stack is empty while validating text {}", self.location())));
            }
            let mut text_elem_on_stack: bool;
            if self.elem_stack.clone().collect::<Vec<SonicObject>>().len() != 0 {
                if self.elem_stack.clone().last().unwrap().last().unwrap().value.is_map() {
                    text_elem_on_stack = false;
                } else if self.elem_stack.clone().last().unwrap().last().unwrap().value.is_vec() {
                    if self.elem_stack.clone().last().unwrap().last().unwrap().collect::<Vec<SonicObject>>().len() >= 1 {
                        text_elem_on_stack = self.elem_stack.clone().last().unwrap().last().unwrap().getindexvalue(0).unwrap().as_str().unwrap() == "text";
                    } else {
                        text_elem_on_stack = false;
                    }
                } else {
                    text_elem_on_stack = false;
                } 
            } else {
                text_elem_on_stack = false;
            }
            if text_elem_on_stack {
                let mut newtext = self.elem_stack.clone().last().unwrap().last().unwrap().getindexvalue(2).unwrap().as_str().unwrap().to_string();
                newtext.push_str(text.as_str());
                let mut inter1 = self.elem_stack.clone().last().unwrap().last().unwrap();
                //inter1.removeindex(2);
                //inter1.push(newtext);
                inter1.replace_index_with(2, newtext);
                let mut inter2 = self.elem_stack.clone().last().unwrap();
                let placenum = inter2.clone().collect::<Vec<SonicObject>>().len() - 1;
                inter2.removeindex(placenum);
                inter2.push(inter1.value);
                let placenum = self.elem_stack.clone().collect::<Vec<SonicObject>>().len() - 1;
                self.elem_stack.removeindex(placenum);
                self.elem_stack.push(inter2.clone().value);
                //println!("text elem on stack {:?}", self.elem_stack.clone().last().unwrap());
            } else {
                //println!("before elem stack {:?}", self.elem_stack.clone());
                let mut inter1 = self.elem_stack.clone().last().unwrap();
                //println!("before inter1 {:?}", inter1.clone());
                let mut empty_val = SonicSerdeObject::new_vec();
                empty_val.push("text");
                let mut empty_val_2 = SonicSerdeObject::new_map();
                empty_val_2.insert("xml:space", self.whitespace_behavior_stack.last().unwrap().to_string());
                empty_val.push(empty_val_2);
                empty_val.push(text);
                inter1.push(empty_val);
                //inter1.push(json!(["text", {json!("xml:space").to_string(): self.whitespace_behavior_stack.last().unwrap()}, text]));
                //println!("after inter1 {:?}", inter1.clone());
                let placenum = self.elem_stack.clone().collect::<Vec<SonicObject>>().len() - 1;
                self.elem_stack.removeindex(placenum);
                self.elem_stack.push(inter1.value);
                //println!("after elem stack {:?}", self.elem_stack.clone());
            }
        } else {
            ();
        }
        Ok(())
    }
    pub fn end_element_ns(&mut self, name: (String, String), _qname: String) {
        let (_uri, elem) = name;
        self.end_element(elem);
    }
    pub fn end_element(&mut self, name: String) {
        //println!("got to end_element {}", name.clone());
        if self.state == self.state_outside_aiml {
            return;
        }
        //println!("got past outside");
        if self.current_unknown != String::new() {
            if name.clone() == self.current_unknown {
                self.current_unknown = String::new();
            }
            return;
        }
        //println!("got past unknown");
        if self.skip_current_category {
            if name.as_str() == "category" {
                self.skip_current_category = false;
                self.state = self.state_inside_aiml;
            }
            return;
        }
        //println!("got past skip");
        //println!("got to match");
        match self._end_element(name) {
            Ok(_x) => (),
            Err(AimlParserError::Error(x)) => {
                eprintln!("PARSE ERROR: {}", x);
                self.num_parse_errors += 1;
                if self.state >= self.state_inside_category {
                    self.skip_current_category = true;
                }
            }
        }
    }
    pub fn _end_element(&mut self, name: String) -> Result<(), AimlParserError> {
        if name.as_str() == "aiml" {
            if self.state != self.state_inside_aiml {
                return Err(AimlParserError::Error(format!("Unexpected </aiml> tag {}", self.location())));
            }
            self.state = self.state_outside_aiml;
            //let placenum = self.whitespace_behavior_stack.len() - 1;
            //self.whitespace_behavior_stack.remove(placenum);
            self.whitespace_behavior_stack.pop().unwrap();
        } else if name.as_str() == "topic" {
            if self.state != self.state_inside_aiml || !self.inside_topic {
                return Err(AimlParserError::Error(format!("Unexpected </topic> tag {}", self.location())));
            }
            self.inside_topic = false;
            self.current_topic = String::new();
        } else if name.as_str() == "category" {
            if self.state != self.state_after_template {
                return Err(AimlParserError::Error(format!("Unexpected </category> tag {}", self.location())));
            }
            self.state = self.state_inside_aiml;
            let key = (self.current_pattern.trim().to_string(), self.current_that.trim().to_string(), self.current_topic.trim().to_string());
            //println!("cat last {:?}", self.elem_stack.clone().last().unwrap());
            self.categories.insert(key, self.elem_stack.clone().last().unwrap());
            //println!("inserted");
            //let placenum = self.whitespace_behavior_stack.len() - 1;
            //self.whitespace_behavior_stack.remove(placenum);
            self.whitespace_behavior_stack.pop();
        } else if name.as_str() == "pattern" {
            if self.state != self.state_inside_pattern {
                return Err(AimlParserError::Error(format!("Unexpected </pattern> tag {}", self.location())));
            }
            self.state = self.state_after_pattern;
        } else if name.as_str() == "that" && self.state == self.state_inside_that {
            self.state = self.state_after_that;
        } else if name.as_str() == "template" {
            if self.state != self.state_inside_template {
                return Err(AimlParserError::Error(format!("Unexpected </template> tag {}", self.location())));
            }
            self.state = self.state_after_template;
            //let placenum = self.whitespace_behavior_stack.len() - 1;
            //self.whitespace_behavior_stack.remove(placenum);
            self.whitespace_behavior_stack.pop();
        } else if self.state == self.state_inside_pattern {
            if !vec!["bot"].contains(&name.as_str()) {
                return Err(AimlParserError::Error(format!("Unexpected </{}> tag {}", name, self.location())));
            }
        } else if self.state == self.state_inside_that {
            if !vec!["bot"].contains(&name.as_str()) {
                return Err(AimlParserError::Error(format!("Unexpected </{}> tag {}", name, self.location())));
            }
        } else if self.state == self.state_inside_template {
            //println!("self elem stack {:?}", self.elem_stack.clone().last().unwrap().value);
            let mut elem = self.elem_stack.clone().last().unwrap();
            let placenum = self.elem_stack.clone().collect::<Vec<SonicObject>>().len() - 1;
            self.elem_stack.removeindex(placenum);
            let mut inter1 = self.elem_stack.clone().last().unwrap();
            inter1.push(elem.clone().value);
            let placenum = self.elem_stack.clone().collect::<Vec<SonicObject>>().len() - 1;
            self.elem_stack.removeindex(placenum);
            self.elem_stack.push(inter1.value);
            //let placenum = self.whitespace_behavior_stack.len() - 1;
            //self.whitespace_behavior_stack.remove(placenum);
            self.whitespace_behavior_stack.pop();
            //println!("elemlen {}", elem.clone().collectvec().len());
            if elem.getindexvalue(0).unwrap().as_str().unwrap() == "condition" {
                //let placenum = self.found_default_li_stack.len() - 1;
                //self.found_default_li_stack.remove(placenum);
                self.found_default_li_stack.pop();
            }
        } else {
            return Err(AimlParserError::Error(format!("Unexpected </{}> tag {}", name, self.location())));
        }
        Ok(())
    }
    pub fn validate_elem_start(&mut self, name: String, attr: HashMap<String, String>, _version: String) -> Result<bool, AimlParserError> {
        let (required, optional, _can_be_parent) = self.validate_info[&name].clone();
        let mut parent = String::new(); //SonicObject::new(SonicSerdeObject::new_vec());
        let mut parent_attr: HashMap<String, String> = HashMap::new();
        for a in required.clone() {
            if !attr.contains_key(&a) && !self.forward_compatible_mode {
                return Err(AimlParserError::Error(format!("Required \"{}\" attribute missing in <{}> element {}", a, name, self.location())));
            }
        }
        for a in attr.keys() {
            if required.clone().contains(a) {
                continue;
            }
            if a.chars().collect::<Vec<char>>()[..4].iter().map(|x| x.to_string()).collect::<Vec<String>>().join("") == "xml:" {
                continue;
            }
            if !optional.contains(a) && !self.forward_compatible_mode {
                return Err(AimlParserError::Error(format!("Unexpected \"{}\" attribute in <{}> element {}", a, name, self.location())));
            }
        }
        if vec!["star", "thatstar", "topicstar"].contains(&name.as_str()) {
            for k in attr.keys() {
                let v = attr.clone()[k].clone();
                if k == "index" {
                    let mut temp: u32 = 0;
                    match v.parse::<u32>() {
                        Ok(d) => {
                            temp = d.clone();
                        },
                        Err(_) => {
                            return Err(AimlParserError::Error(format!("Bad type for \"{}\" attribute (expected u32, found \"{}\" {}", k, v, self.location())));
                        }
                    }
                    if temp < 1 {
                        return Err(AimlParserError::Error(format!("\"{}\" attribute must have non-negative value {}", k, self.location())));
                    }
                }
            }
        }
        //for g in self.elem_stack.value.as_vec() {
        //
        //}
        if self.elem_stack.clone().collect::<Vec<SonicObject>>().len() != 0 {
            if self.elem_stack.clone().last().unwrap().value.as_vec().unwrap().len() > 0 {
                parent = self.elem_stack.clone().last().unwrap().getindexvalue(0).unwrap().as_str().unwrap().to_string();
            }
            if self.elem_stack.clone().last().unwrap().value.as_vec().unwrap().len() > 1 {
                let mut x = self.elem_stack.clone().last().unwrap().getindexvalue(1).unwrap();
                let mut y = x.as_map().unwrap();
                for key in y.keys() {
                    parent_attr.insert(key.as_str().unwrap().to_string(), y.get(key).unwrap().as_str().unwrap().to_string());
                }
                //parent_attr = self.elem_stack.clone().last().unwrap().getindexvalue(1).as_map().unwrap().as_;
            }
    
        } else {
            return Err(AimlParserError::Error(format!("Element stack is empty while validating <{}> {}", name.clone(), self.location())))
        }
        //println!("parent {}", parent);
        let (_required, _optional, can_be_parent) = self.validate_info.clone()[&parent].clone();
        let mut non_block_style_condition = parent.as_str() == "condition" && !(parent_attr.contains_key("name") && parent_attr.contains_key("value"));
        if !can_be_parent {
            return Err(AimlParserError::Error(format!("<{}> elements cannot have any contents {}", parent, self.location())));
        } else if (parent.as_str() == "random" || non_block_style_condition) && name.as_str() != "li" {
            return Err(AimlParserError::Error(format!("<{}> elements can only contain <li> subelements {}", parent, self.location())));
        } else if name.as_str() == "li" {
            if !(parent.as_str() == "random" || non_block_style_condition) {
                return Err(AimlParserError::Error(format!("Unexpected <li> element contained by <{}> element {}", parent, self.location())));
            }
            if non_block_style_condition {
                if parent_attr.contains_key("name") {
                    if attr.keys().len() == 0 {
                        if self.found_default_li_stack.last().unwrap().clone() {
                            return Err(AimlParserError::Error(format!("Unexpected default <li> element inside <condition> {}", self.location())));
                        } else {
                            //let last_place = self.found_default_li_stack.clone().len();
                            //self.found_default_li_stack[found_default_li_stack_clone.len() - 1 ] = true;
                            self.found_default_li_stack.pop();
                            self.found_default_li_stack.push(true);
                        }
                    } else if attr.keys().len() == 1 && attr.contains_key("value") {
                        ();
                    } else {
                        return Err(AimlParserError::Error(format!("Invalid <li> inside single-predicate <condition> {}", self.location())));
                    }
                } else if parent_attr.keys().len() == 0 {
                    if attr.keys().len() == 0 {
                        if self.found_default_li_stack.last().unwrap().clone() {
                            return Err(AimlParserError::Error(format!("Unexpected default <li> element inside <condition> {}", self.location())));
                        } else {
                            //let found_default_li_stack_clone = self.found_default_li_stack.clone();
                            //self.found_default_li_stack[found_default_li_stack_clone.len() - 1 ] = true;
                            let _last_place = self.found_default_li_stack.clone().len();
                            //self.found_default_li_stack[found_default_li_stack_clone.len() - 1 ] = true;
                            self.found_default_li_stack.pop();
                            self.found_default_li_stack.push(true);
                        }
                    } else if attr.keys().len() == 2 && attr.contains_key("value") && attr.contains_key("name") {
                        ();
                    } else {
                        return Err(AimlParserError::Error(format!("Invalid <li> inside multi-predicate <condition> {}", self.location())));
                    }
                }
            }
        }
        Ok(true)
    }
}