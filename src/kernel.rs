//use crate::patternmgr::GeneralAimlKey;
use std::path::PathBuf;
use std::collections::HashMap;
use crate::patternmgr::PatternMgr;
use crate::{sonicobject::SonicObject, sonicobject};
use crate::wordsub::WordSub;
use rand::seq::SliceRandom;
use sonic_serde_object::SonicSerdeObject;
use crate::defaultsubs;
//use xml::reader::ParserConfig;
//use std::fs::File;
use crate::aimlparser;
use chrono::{Utc};
//use std::io::BufReader;
//use serde::{Serialize, Deserialize};
//use xml::reader::{EventReader, XmlEvent};
#[derive(Clone)]
pub struct Kernel {
    global_session_id: String,
    max_history_size: u8,
    max_recursion_depth: u32,
    input_history: String,
    output_history: String,
    input_stack: String,
    verbose_mode: bool,
    version: String,
    brain: PatternMgr,
    sessions: SonicObject,
    bot_predicates: HashMap<String, String>,
    subbers: HashMap<String, WordSub>,
    //rand_thread: rand::prelude::ThreadRng,
    allow_commands: bool,
    reg: regex::Regex,
}
#[allow(unused_mut, unused_assignments, unused_doc_comments)]
impl Kernel {
    pub fn new() -> Self {
        let global_session_id = "_global".to_string();
        let max_history_size = 10;
        let max_recursion_depth = 100;
        let input_history = "_inputHistory".to_string();
        let output_history = "_outputHistory".to_string();
        let input_stack = "_inputStack".to_string();
        let mut verbose_mode = true;
        //let mut rand_thread = rand::thread_rng();
        const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
        let mut version = format!("aiml-port {}", VERSION.unwrap());
        let mut brain = PatternMgr::new();
        let mut empty_val = SonicSerdeObject::new_map();
        empty_val.insert(input_history.clone(), SonicSerdeObject::new_vec());
        empty_val.insert(output_history.clone(), SonicSerdeObject::new_vec());
        empty_val.insert(input_stack.clone(), SonicSerdeObject::new_vec());
        //let mut empty_val_2 = SonicSerdeObject::new_vec();
        let mut sessions = SonicObject::new(SonicSerdeObject::new_map_with(global_session_id.clone(), empty_val));
        //let mut sessions = SonicObject::new(json!({json!(global_session_id.clone()).to_string(): {json!(input_history.clone()).to_string(): [], json!(output_history.clone()).to_string(): [], json!(input_stack.clone()).to_string(): []}}));
        let mut bot_predicates: HashMap<String, String> = HashMap::new();
        bot_predicates.insert("name".to_string(), "Nameless".to_string());
        let mut reg = regex::Regex::new(r#"\s+"#).unwrap();
        let mut subbers: HashMap<String, WordSub> = HashMap::new();
        subbers.insert("gender".to_string(), WordSub::new(defaultsubs::get_default_gender()));
        subbers.insert("person".to_string(), WordSub::new(defaultsubs::get_default_person()));
        subbers.insert("person2".to_string(), WordSub::new(defaultsubs::get_default_person2()));
        subbers.insert("normal".to_string(), WordSub::new(defaultsubs::get_default_normal()));
        //println!("subbers normal {:?}", subbers["normal"].clone());
        Self {
            global_session_id: global_session_id.clone(),
            max_history_size: max_history_size,
            max_recursion_depth: max_recursion_depth,
            input_history: input_history,
            output_history: output_history,
            input_stack: input_stack,
            verbose_mode: verbose_mode,
            version: version,
            brain: brain,
            sessions: sessions,
            bot_predicates: bot_predicates,
            subbers: subbers,
            allow_commands: false,
            reg: reg,
            //rand_thread: rand_thread,
        }
    }
    //pub fn bootstrap(&mut self, brain_file: Option<PathBuf>, learn_files: Option<Vec<PathBuf>>, commands: Option<Vec<String>>, chdir: Option<PathBuf>) {
    //    if brain_file.is_some() {
    //
    //    }
    //}
    pub fn add_session(&mut self, session_id: &str) {
        if self.sessions.contains(session_id) {
            return;
        } else {
            let mut empty_val = SonicSerdeObject::new_map();
            empty_val.insert(self.input_history.clone(), SonicSerdeObject::new_vec());
            empty_val.insert(self.output_history.clone(), SonicSerdeObject::new_vec());
            empty_val.insert(self.input_stack.clone(), SonicSerdeObject::new_vec());
            //self.sessions.insert(session_id, json!({json!(self.input_history.clone()).to_string(): [], json!(self.output_history.clone()).to_string(): [], json!(self.input_stack.clone()).to_string(): []}));
            self.sessions.insert(session_id, empty_val);
        }
    }
    pub fn verbose(&mut self, is_verbose: bool) {
        self.verbose_mode = is_verbose;
    }
    pub fn version(&mut self) -> String {
        self.version.clone()
    }
    pub fn num_categories(&mut self) -> u128 {
        self.brain.num_templates()
    }
    pub fn load_brain(&mut self, dirname: impl Into<PathBuf>) {
        let dir_clone = dirname.into();
        if self.verbose_mode {
            println!("Loading brain from {}", dir_clone.clone().display());
        }
        self.brain.restore(dir_clone.clone());
        if self.verbose_mode {
            println!("Done ({} categories)", self.brain.num_templates());
        }
    }
    pub fn save_brain(&mut self, dirname: impl Into<PathBuf>) {
        let dir_clone = dirname.into();
        if self.verbose_mode {
            println!("Saving brain to {}", dir_clone.clone().display());
        }
        self.brain.save(dir_clone.clone());
        if self.verbose_mode {
            println!("Done");
        }
    }
    pub fn save_sessions(&mut self, dirname: impl Into<PathBuf>) {
        let dir_clone = dirname.into();
        let mut session_save = sonicobject::SonicPersistObject::new(dir_clone);
        session_save.insert("sessions", self.sessions.clone().value);
        session_save.flush();
    }
    pub fn load_sessions(&mut self, dirname: impl Into<PathBuf>) {
        let dir_clone = dirname.into();
        let session_save = sonicobject::SonicPersistObject::new(dir_clone);
        self.sessions = session_save.get("sessions");
        //session_save.insert("sessions", self.sessions.clone().value);
        //session_save.flush();
    }
    pub fn get_predicate(&self, name: impl Into<SonicSerdeObject>, session_id: Option<&str>) -> SonicObject {
        let mut session_name = self.global_session_id.as_str();
        if session_id.is_some() {
            session_name = session_id.unwrap();
        }
        //println!("sn {}", session_name);
        let nameclone = name.into();
        if self.sessions.contains(session_name) {
            //println!("s {:?}", self.sessions.get(session_name).unwrap().keys());
            //println!("b {}", json!(&name).to_string());
            if self.sessions.get(session_name).unwrap().contains(nameclone.clone()) {
                return self.sessions.get(session_name).unwrap().get(nameclone.clone()).unwrap();
            }
        }
        SonicObject::new(String::new())
    }
    pub fn set_predicate(&mut self, name: impl Into<SonicSerdeObject>, value: impl Into<SonicSerdeObject>, session_id: Option<&str>) {
        let mut g = self.global_session_id.clone();
        let mut session_name = g.as_str();
        if session_id.is_some() {
            session_name = session_id.unwrap();
        }
        self.add_session(session_name);
        let mut this_session = self.sessions.get(session_name).unwrap();
        this_session.insert(name, value);
        self.sessions.insert(session_name, this_session.value);
    }
    pub fn get_bot_predicate(&self, name: impl Into<String>) -> String {
        let name_clone = name.into();
        if self.bot_predicates.contains_key(name_clone.as_str()) {
            return self.bot_predicates[name_clone.as_str()].clone();
        } else {
            return String::new();
        }
    }
    pub fn set_bot_predicate(&mut self, name: impl Into<String>, value: impl Into<String>) {
        let name_clone = name.into();
        self.bot_predicates.insert(name_clone.clone(), value.into());
        if name_clone.clone().as_str() == "name" {
            self.brain.set_bot_name(self.get_bot_predicate("name").clone())
        }
    }
    pub fn delete_session(&mut self, session_id: &str) {
        if self.sessions.contains(session_id) {
            self.sessions.remove(session_id)
        }
    }
    pub fn get_session_data(&self, session_id: Option<&str>) -> SonicObject {
        if session_id.is_some() {
            if self.sessions.contains(session_id.unwrap()) {
                return self.sessions.get(session_id.unwrap()).unwrap().clone();
            } else {
                return SonicObject::new(SonicSerdeObject::new_map());
            }
        } else {
            return self.sessions.clone();
        }
    }
    pub fn learn(&mut self, filename: &str) {
        let mut filepathlist = glob::glob(filename).unwrap();
        for filepath in filepathlist {
            let mut starttime = std::time::Instant::now();
            let mut fpathbuf: PathBuf = filepath.unwrap();
            if self.verbose_mode {
                println!("Loading {}...", fpathbuf.display());
            }
            //println!("{}", fpathbuf.clone().as_path().display());
            //let file = File::open(fpathbuf.clone()).unwrap();
            //let mut file = BufReader::new(file);
            
            let mut aparser = aimlparser::AimlParser::new();
            let text = std::fs::read_to_string(fpathbuf).unwrap();
            let doc = roxmltree::Document::parse(&text).unwrap();
            for elem in doc.root().children() {
                //for attr in elem.attributes() {
                //    println!("{}", attr.name());
                //}
                self.rec(&mut aparser, elem);
            }
            /*
            let mut parser = ParserConfig::new()
            .whitespace_to_characters(true)
            .create_reader(&mut file);
            //let mut next_result = parser.next();
            //let mut prev_result = next_result.clone();
            let mut start = true;
            let mut aparser = aimlparser::AimlParser::new();
            //while next_result != prev_result || start {
            for next_result in parser {
                //println!("nah");
                start = false;
                match next_result.clone() {
                    Ok(XmlEvent::StartElement { name, attributes, namespace }) => {
                        //println!("name {}", name.clone());
                        //if name.namespace.is_some() && name.prefix.is_some() {
                        //    aparser.start_element_ns((name.namespace.unwrap(), name.local_name.clone()), format!("{}:{}", name.prefix.unwrap(), name.local_name), crate::attr_list_to_hashmap(attributes));
                        //} else {
                            aparser.start_element(name.local_name, crate::attr_list_to_hashmap(attributes));
                        //}
                    },
                    Ok(XmlEvent::EndElement { name }) => {
                        aparser.end_element(name.local_name);
                    },
                    Ok(XmlEvent::ProcessingInstruction { name, data }) => {
                        ();
                    },
                    Ok(XmlEvent::StartDocument { version, encoding, standalone }) => {
                        ();
                    },
                    Ok(XmlEvent::EndDocument) => {
                        break;
                    },
                    Ok(XmlEvent::Characters(z)) => {
                        aparser.characters(z)
                    },
                    Ok(XmlEvent::Whitespace(_z)) => {
                        ();
                    },
                    Ok(XmlEvent::CData(_z)) => {
                        ();
                    },
                    Ok(XmlEvent::Comment(_z)) => {
                        ();
                    },
                    Err(e) => {
                        eprintln!("FATAL PARSE ERROR in file {}: {}", fpathbuf.as_path().display(), e)
                    }    
                }
                //prev_result = next_result.clone();
                //next_result = parser.next();
            }
            */
            //println!("ok");
            //println!("{:?}", aparser.categories.clone());
            //println!("{}", aparser.categories.clone().keys());
            //println!("{}", aparser.categories.clone().keys().len());
            //if aparser.categories.clone().keys().len() > 0 {
                //println!("bah");
            for (key, tem) in aparser.categories.clone().into_iter() {
                //println!("adding");
                self.brain.add(key, tem);
                //println!("added");
            }
            //}
            //aparser.categories.clone().into_par_iter().for_each_with(self.brain, |brain, (key, tem)| brain.add(key, tem));
            //println!("ok2");
            //println!("{:?}", self.verbose_mode);
            if self.verbose_mode {
                println!("done ({} seconds)", starttime.elapsed().as_secs_f32());
            }
            //println!("");
        }
        //println!("past files");
    }
    fn rec(&mut self, aparser: &mut aimlparser::AimlParser, x: roxmltree::Node) {
        if x.is_text() {
            aparser.characters(x.text().unwrap().to_string());
        } else if x.is_element() {
            let mut z = x.attributes();
            let mut attr = crate::attr_list_to_hashmap(z);
            aparser.start_element(x.tag_name().name().to_string(), attr);
            //println!("<{}>", x.tag_name().name());
            for y in x.children() {
                self.rec(aparser, y);
            }
            aparser.end_element(x.tag_name().name().to_string())
            //println!("</{}>", x.tag_name().name());
        }
    }
    fn _respond(&mut self, input: &str, session_id: &str) -> String {
        if input.len() == 0 {
            return String::new();
        }
        //println!("1");
        let mut input_stack = self.get_predicate(self.input_stack.clone(), Some(session_id));
        if input_stack.collect::<Vec<SonicObject>>().len() > self.max_recursion_depth as usize {
            if self.verbose_mode {
                eprintln!("WARNING: maximum recursion depth exceeded (input='{}')", input.clone());
                return String::new();
            }
        }
        //println!("2");
        //println!("{}", session_id);
        //println!("{:?}", self.input_stack);
        //println!
        input_stack = self.get_predicate(self.input_stack.clone(), Some(session_id));
        //println!("{:?}", input_stack);
        input_stack.push(input);
        self.set_predicate(self.input_stack.clone(), input_stack.value, Some(session_id));
        let mut subbed_input = self.subbers["normal"].clone().sub(input.to_string());
        //println!("subbed input '{}'", subbed_input);
        let mut output_history = self.get_predicate(self.output_history.clone(), Some(session_id));
        //let mut topic = self.get_predicate("topic", Some(session_id));
        let mut that: String;
        if output_history.clone().collect::<Vec<SonicObject>>().len() > 0 {
            that = output_history.clone().last().unwrap().value.as_str().unwrap().to_string();
        } else {
            that = String::new();
        }
        let mut subbed_that = self.subbers["normal"].clone().sub(that);
        let mut topic = self.get_predicate("topic", Some(session_id)).value.as_str().unwrap().to_string();
        let mut subbed_topic = self.subbers["normal"].clone().sub(topic);
        let mut response = String::new();
        //println!("root keys len {}", self.brain.root.keys().len());
        
        let mut elem = self.brain.r#match(subbed_input, subbed_that, subbed_topic);
        if elem.is_none() {
            if self.verbose_mode {
                eprintln!("WARNING: No match found for input: {}", input);
            }
        } else {
            response.push_str(self.process_element(elem.unwrap(), session_id).trim());
            response.push_str(" ");
        }
        input_stack = self.get_predicate(self.input_stack.clone(), Some(session_id));
        input_stack.removeindex(input_stack.collectvec().len() - 1);
        self.set_predicate(self.input_stack.clone(), input_stack.clone().value, Some(session_id));
        response
    }
    pub fn respond_session(&mut self, input: &str, session_id: &str) -> String {
        //println!("start respond input '{}'", input);
        if input.len() == 0 {
            return String::new();
        } else {
            self.add_session(session_id);
            //println!("got past add session");
            let mut sentences = crate::utils::sentences(input.to_string());
            //println!("{:?}", sentences);
            let mut final_response = String::new();
            for s in sentences {
                //println!("s {}", s);
                //println!("{:?}", self.input_history.clone());
                let mut input_history: SonicObject = self.get_predicate(self.input_history.clone(), Some(session_id));
                //println!("{:?}", input_history);
                //println!("input history val {:?}, s {:?}", input_history.clone().value, s.clone());
                input_history.push(s.clone());
                while input_history.collectvec().len() > self.max_history_size as usize {
                    input_history.removeindex(0);
                }
                self.set_predicate(self.input_history.clone(), input_history.value, Some(session_id));
                let mut response = self._respond(s.clone().as_str(), session_id);
                //println!("response {}", response.clone());
                let mut output_history: SonicObject = self.get_predicate(self.output_history.clone(), Some(session_id));
                output_history.push(response.clone());
                while output_history.collectvec().len() > self.max_history_size as usize {
                    output_history.removeindex(0);
                }
                self.set_predicate(self.output_history.clone(), output_history.value, Some(session_id));
                final_response.push_str(format!("{}  ", response).as_str());
            }
            final_response = final_response.trim().to_string();
            if self.get_predicate(self.input_stack.clone(), Some(session_id)).collectvec().len() == 0 {
                return final_response;
            } else {
                return String::new();
            }
        }
    }
    pub fn respond(&mut self, input: &str) -> String {
        let session_id_string = self.global_session_id.clone();
        self.respond_session(input, session_id_string.as_str())
    }
    //pub fn bootstrap(&mut self, brain_file: Option<&str>, learn_files: Option<Vec<&str>>, commands: Option<Vec<&str>>, chdir: Option<&str>) {
    //    let mut start = std::time::Instant::now();
    //    if brain_file.is_some() {
    //        self.load_brain(PathBuf::from(brain_file.unwrap()));
    //    }
    //}
    pub fn process_element(&mut self, elem: SonicObject, session_id: &str) -> String {
        let elem_name_val = elem.getindexvalue(0).unwrap();
        //println!("process {}", elem_name_val.as_str().unwrap());
        if elem_name_val.as_str().unwrap() == "bot" {
            return self.process_bot(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "condition" {
            return self.process_condition(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "date" {
            return self.process_date(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "formal" {
            return self.process_formal(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "gender" {
            return self.process_gender(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "get" {
            return self.process_get(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "gossip" {
            return self.process_gossip(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "id" {
            return self.process_id(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "input" {
            return self.process_input(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "javascript" {
            return self.process_javascript(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "learn" {
            return self.process_learn(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "li" {
            return self.process_li(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "lowercase" {
            return self.process_lowercase(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "person" {
            return self.process_person(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "person2" {
            return self.process_person2(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "random" {
            return self.process_random(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "text" {
            return self.process_text(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "sentence" {
            return self.process_sentence(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "set" {
            return self.process_set(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "size" {
            return self.process_size(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "sr" {
            return self.process_sr(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "srai" {
            return self.process_srai(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "star" {
            return self.process_star(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "system" {
            return self.process_system(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "template" {
            return self.process_template(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "that" {
            return self.process_that(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "thatstar" {
            return self.process_that_star(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "think" {
            return self.process_think(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "topicstar" {
            return self.process_topic_star(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "uppercase" {
            return self.process_uppercase(elem.clone(), session_id)
        } else if elem_name_val.as_str().unwrap() == "version" {
            return self.process_version(elem.clone(), session_id)
        } else {
            eprintln!("WARNING: No handler found for <{}> element", elem_name_val.as_str().unwrap());
            return String::new();
        }
    }
    fn process_bot(&mut self, elem: SonicObject, _session_id: &str) -> String {
        let mut attr_name = elem.getindex(1).unwrap().getvalue("name").unwrap().as_str().unwrap().to_string();
        self.get_bot_predicate(attr_name).to_string()
    }
    
    fn process_condition(&mut self, elem: SonicObject, session_id: &str) -> String {
        let mut found_match: bool;
        let mut response = String::new();
        //let mut name = None;
        let mut attr = elem.getindex(1).unwrap();
        //self.proc_cond_try1(attr.clone(), elem.clone(), session_id.clone(), &mut response.clone())
        let mut list_items: Vec<SonicObject> = Vec::new();
        if attr.contains("name") && attr.contains("value") {
            let mut val = self.get_predicate(attr.getvalue("name").unwrap().as_str().unwrap(), Some(session_id));
            if val == attr.get("value").unwrap() {
                for e in elem.collect::<Vec<SonicObject>>()[2..].iter() {
                    response.push_str(self.process_element(e.clone(), session_id).as_str());
                }
                return response;
            }
        } else {
            let mut name = match attr.getvalue("name") {
                Ok(z) => Some(z.as_str().unwrap().to_string()),
                Err(_) => None,
            };
            for e in elem.collect::<Vec<SonicObject>>()[2..].iter() {
                if e.getindexvalue(0).unwrap().as_str().unwrap() == "li" {
                    list_items.push(e.clone());
                }
            }
            if list_items.len() == 0 {
                return String::new();
            }
            found_match = false;
            for li in list_items.clone() {
                let mut li_attr = li.getindex(1).unwrap();
                if li_attr.collectvec().len() == 0 && &li == list_items.clone().last().unwrap() {
                    continue;
                }
                let mut li_name = name.clone();
                if li_name.is_none() {
                    li_name = Some(li_attr.getvalue("name").unwrap().as_str().unwrap().to_string());
                }
                let mut li_value = li_attr.get("value").unwrap();
                if self.get_predicate(li_name.unwrap(), Some(session_id)) == li_value {
                    found_match = true;
                    response.push_str(self.process_element(li.clone(), session_id).as_str());
                    break;
                }
            }
            if !found_match {
                let mut li = list_items.iter().last().unwrap();
                let mut li_attr = li.getindex(1).unwrap();
                if !(li_attr.contains("name") || li_attr.contains("value")) {
                    response.push_str(self.process_element(li.clone(), session_id).as_str());
                }
            }
        }
        response
    }
    pub fn process_date(&mut self, _elem: SonicObject, _session_id: &str) -> String {
        format!("{}", Utc::now())
    }
    pub fn process_formal(&mut self, elem: SonicObject, session_id: &str) -> String {
        let mut response = String::new();
        for e in elem.collectvec()[2..].iter() {
            response.push_str(self.process_element(e.clone(), session_id).as_str());
        }
        crate::cap_words(response)
    }
    pub fn process_gender(&mut self, elem: SonicObject, session_id: &str) -> String {
        let mut response = String::new();
        for e in elem.collectvec()[2..].iter() {
            response.push_str(self.process_element(e.clone(), session_id).as_str());
        }
        self.subbers.clone()["gender"].clone().sub(response)
    }
    pub fn process_get(&mut self, elem: SonicObject, session_id: &str) -> String {
        self.get_predicate(elem.getindex(1).unwrap().get("name").unwrap().value, Some(session_id)).value.as_str().unwrap().to_string()
    }
    pub fn process_think(&mut self, elem: SonicObject, session_id: &str) -> String {
        for e in elem.collectvec()[2..].iter() {
            self.process_element(e.clone(), session_id);
        }
        String::new()
    }
    fn process_gossip(&mut self, elem: SonicObject, session_id: &str) -> String {
        self.process_think(elem.clone(), session_id)
    }
    fn process_id(&mut self, _elem: SonicObject, session_id: &str) -> String {
        session_id.to_string()
    }
    fn process_input(&mut self, elem: SonicObject, session_id: &str) -> String {
        let mut input_history = self.get_predicate(self.input_history.clone(), Some(session_id));
        let index: usize = match elem.getindex(1).unwrap().getvalue("index") {
            Ok(z) => z.as_str().unwrap().parse().unwrap(),
            Err(_) => 1,
        };
        if input_history.collectvec().len() > index {
            return input_history.getindexvalue(input_history.collectvec().len() - index).unwrap().as_str().unwrap().to_string();
        } else {
            if self.verbose_mode {
                eprintln!("No such index {} while processing <input> element.", index)
            }
            return String::new();
        }
    }
    fn process_javascript(&mut self, elem: SonicObject, session_id: &str) -> String {
        self.process_think(elem.clone(), session_id)
    }
    fn process_learn(&mut self, elem: SonicObject, session_id: &str) -> String {
        let mut filename = String::new();
        for e in elem.collectvec()[2..].iter() {
            filename.push_str(self.process_element(e.clone(), session_id).as_str());
        }
        self.learn(&filename);
        String::new()
    }
    fn process_li(&mut self, elem: SonicObject, session_id: &str) -> String {
        let mut response = String::new();
        for e in elem.collectvec()[2..].iter() {
            response.push_str(self.process_element(e.clone(), session_id).as_str());
        }
        response
    }
    fn process_lowercase(&mut self, elem: SonicObject, session_id: &str) -> String {
        let mut response = String::new();
        for e in elem.collectvec()[2..].iter() {
            response.push_str(self.process_element(e.clone(), session_id).as_str());
        }
        response.to_lowercase()
    }
    fn process_person(&mut self, elem: SonicObject, session_id: &str) -> String {
        let mut response = String::new();
        for e in elem.collectvec()[2..].iter() {
            response.push_str(self.process_element(e.clone(), session_id).as_str());
        }
        if elem.collectvec()[2..].iter().len() == 0 {
            response.push_str(self.process_element(SonicObject::new(SonicSerdeObject::Vec(vec!["star".into(), SonicSerdeObject::new_map()])), session_id).as_str());
        }
        self.subbers.clone()["person"].clone().sub(response)
    }
    fn process_person2(&mut self, elem: SonicObject, session_id: &str) -> String {
        let mut response = String::new();
        for e in elem.collectvec()[2..].iter() {
            response.push_str(self.process_element(e.clone(), session_id).as_str());
        }
        if elem.collectvec()[2..].iter().len() == 0 {
            response.push_str(self.process_element(SonicObject::new(SonicSerdeObject::Vec(vec!["star".into(), SonicSerdeObject::new_map()])), session_id).as_str());
        }
        self.subbers.clone()["person2"].clone().sub(response)
    }
    fn process_random(&mut self, elem: SonicObject, session_id: &str) -> String {
        let mut list_items: Vec<SonicObject> = Vec::new();
        for e in elem.collectvec()[2..].iter() {
            if e.getindexvalue(0).unwrap().as_str().unwrap() == "li" {
                list_items.push(e.clone());
            }
        }
        if list_items.len() == 0 {
            return String::new();
        }
        self.process_element(list_items.choose(&mut rand::thread_rng()).unwrap().clone(), session_id)
    }
    fn process_sentence(&mut self, elem: SonicObject, session_id: &str) -> String {
        let mut response = String::new();
        for e in elem.collectvec()[2..].iter() {
            response.push_str(self.process_element(e.clone(), session_id).as_str());
        }
        response = response.trim().to_string();
        match response.split_once(" ") {
            Some(z) => {
                let mut words = z.clone();
                let t = crate::cap_words(words.0.to_string());
                words.0 = t.as_str();
                return format!("{} {}", words.0, words.1);
            },
            None => {
                return String::new();
            }
        }
    }
    fn process_set(&mut self, elem: SonicObject, session_id: &str) -> String {
        //println!("proc set");
        let mut value = String::new();
        //println!("set elem {:?}", elem);
        for e in elem.collectvec()[2..].iter() {
            value.push_str(self.process_element(e.clone(), session_id).as_str());
        }
        //println!("set value {}", value);
        self.set_predicate(elem.getindex(1).unwrap().getvalue("name").unwrap().as_str().unwrap(), value.clone(), Some(session_id));
        value
    }
    fn process_size(&mut self, _elem: SonicObject, _session_id: &str) -> String {
        format!("{}", self.num_categories())
    }
    fn process_sr(&mut self, _elem: SonicObject, session_id: &str) -> String {
        let mut star = self.process_element(SonicObject::new(SonicSerdeObject::Vec(vec!["star".into(), SonicSerdeObject::new_map()])), session_id);
        self._respond(star.as_str(), session_id)
    }
    fn process_srai(&mut self, elem: SonicObject, session_id: &str) -> String {
        let mut new_input = String::new();
        for e in elem.collectvec()[2..].iter() {
            new_input.push_str(self.process_element(e.clone(), session_id).as_str());
        }
        self._respond(new_input.as_str(), session_id)
    }
    fn process_star(&mut self, elem: SonicObject, session_id: &str) -> String {
        let index: usize = match elem.getindex(1).unwrap().getvalue("index") {
            Ok(z) => {
                z.as_str().unwrap().parse().unwrap()
            },
            Err(sonicobject::SonicObjectError::KeyError(_z)) => {
                1 as usize
            },
            Err(e) => {
                panic!("{}", e)
            }
        };
        let mut input_stack = self.get_predicate(self.input_stack.clone(), Some(session_id));
        let mut input = self.subbers["normal"].clone().sub(input_stack.last().unwrap().value.as_str().unwrap().to_string());
        let mut output_history = self.get_predicate(self.output_history.clone(), Some(session_id));
        let mut that: String;
        if output_history.collectvec().len() > 0 {
            that = self.subbers["normal"].clone().sub(output_history.last().unwrap().value.as_str().unwrap().to_string());
        } else {
            that = String::new();
        }
        let mut topic = self.get_predicate("topic", Some(session_id));
        self.brain.star("star".to_string(), input, that, topic.value.as_str().unwrap().to_string(), index)
    }
    //pub fn set_allow_commands(&mut self, allow: bool) {
    //    self.allow_commands = allow;
    //}
    //pub fn get_allow_commands(&self) -> bool {
    //    self.allow_commands
    //}
    //#[cfg(target_os = "windows")]
    fn process_system(&mut self, _elem: SonicObject, _session_id: &str) -> String {
        //if !self.allow_commands {
        //    return "In order for me to respond to that, I need to run a command.  Unfortunately, I'm not allowed to do that.".to_string();
        //} else {
        //    let mut command = String::new();
        //    for e in elem.collectvec()[2..].iter() {
        //        command.push_str(self.process_element(e.clone(), session_id).as_str());
        //    }
        //    command
        //}
        return "In order for me to respond to that, I need to run a command.  Unfortunately, I'm not allowed to do that.".to_string();
    }
    fn process_template(&mut self, elem: SonicObject, session_id: &str) -> String {
        let mut response = String::new();
        for e in elem.collectvec()[2..].iter() {
            response.push_str(self.process_element(e.clone(), session_id).as_str());
        }
        response
    }
    fn process_text(&mut self, elem: SonicObject, _session_id: &str) -> String {
        if !elem.getindexvalue(2).unwrap().is_string() {
            panic!("Text element contents are not text");
        } else {
            /// The code below can't be implemented directly in rust.  But it seems to be only for efficiency
            // if elem[1]["xml:space"] == "default":
            // elem[2] = re.sub(r"\s+", " ", elem[2])
            // elem[1]["xml:space"] = "preserve"
            
            return self.reg.replace_all(elem.getindexvalue(2).unwrap().as_str().unwrap(), " ").to_string();
        }
    }
    fn process_that(&mut self, elem: SonicObject, session_id: &str) -> String {
        let mut output_history = self.get_predicate(self.output_history.clone(), Some(session_id));
        let mut index: usize = 1;
        match elem.getindex(1).unwrap().getvalue("index") {
            Ok(z) => {
                index = z.as_str().unwrap().to_string().split(",").collect::<Vec<&str>>()[0].parse().unwrap();
            },
            Err(_) => {
                ();
            }
        }
        if output_history.clone().collectvec().len() >= index {
            return output_history.clone().collectvec()[output_history.clone().collectvec().len() - index].value.as_str().unwrap().to_string();
        } else {
            if self.verbose_mode {
                eprintln!("No such index {} while processing <that> element.", index);
            }
            return String::new();
        }
    }
    fn process_that_star(&mut self, elem: SonicObject, session_id: &str) -> String {
        let index: usize = match elem.getindex(1).unwrap().getvalue("index") {
            Ok(z) => {
                z.as_str().unwrap().parse().unwrap()
            },
            Err(sonicobject::SonicObjectError::KeyError(_z)) => {
                1 as usize
            },
            Err(e) => {
                panic!("{}", e)
            }
        };
        let mut input_stack = self.get_predicate(self.input_stack.clone(), Some(session_id));
        let mut input = self.subbers["normal"].clone().sub(input_stack.last().unwrap().value.as_str().unwrap().to_string());
        let mut output_history = self.get_predicate(self.output_history.clone(), Some(session_id));
        let mut that: String;
        if output_history.collectvec().len() > 0 {
            that = self.subbers["normal"].clone().sub(output_history.last().unwrap().value.as_str().unwrap().to_string());
        } else {
            that = String::new();
        }
        let mut topic = self.get_predicate("topic", Some(session_id));
        self.brain.star("thatstar".to_string(), input, that, topic.value.as_str().unwrap().to_string(), index)
    }
    fn process_topic_star(&mut self, elem: SonicObject, session_id: &str) -> String {
        let index: usize = match elem.getindex(1).unwrap().getvalue("index") {
            Ok(z) => {
                z.as_str().unwrap().parse().unwrap()
            },
            Err(sonicobject::SonicObjectError::KeyError(_z)) => {
                1 as usize
            },
            Err(e) => {
                panic!("{}", e)
            }
        };
        let mut input_stack = self.get_predicate(self.input_stack.clone(), Some(session_id));
        let mut input = self.subbers["normal"].clone().sub(input_stack.last().unwrap().value.as_str().unwrap().to_string());
        let mut output_history = self.get_predicate(self.output_history.clone(), Some(session_id));
        let mut that: String;
        if output_history.collectvec().len() > 0 {
            that = self.subbers["normal"].clone().sub(output_history.last().unwrap().value.as_str().unwrap().to_string());
        } else {
            that = String::new();
        }
        let mut topic = self.get_predicate("topic", Some(session_id));
        self.brain.star("topicstar".to_string(), input, that, topic.value.as_str().unwrap().to_string(), index)
    }
    fn process_uppercase(&mut self, elem: SonicObject, session_id: &str) -> String {
        let mut response = String::new();
        for e in elem.collectvec()[2..].iter() {
            response.push_str(self.process_element(e.clone(), session_id).as_str());
        }
        response.to_uppercase()
    }
    fn process_version(&mut self, _elem: SonicObject, _session_id: &str) -> String {
        self.version()
    }
}
