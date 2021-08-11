mod utils;

use utils::*;
use std::collections::HashMap;

pub fn is_some<T>(opt: Option<T>) -> bool{
    match opt {
        Some(_) => true,
        None => false,
    }
}
pub fn cut_last_char(s: &str) -> &str {
    let mut chars = s.chars();
    chars.next_back();
    chars.as_str()
}
pub fn cut_first_char(s: &str) -> &str {
    let mut chars = s.chars();
    chars.next();
    chars.as_str()
}
pub fn get_arg_value(args: Vec<String>, key: &str) -> String {
    match args.iter().position(|n| format!("-{}", key).eq(n)) {
        Some(v) => args[v+1].to_string(),
        None => "".to_string(),
    }
}

pub struct Args {
    strings: HashMap<String, String>,
    bools:   HashMap<String, bool>,
    ints:    HashMap<String, i32>,
}

impl Args {
    pub fn new(schema: &str, args: Vec<String>) -> Args{
        let mut self_ = Args {
            strings: HashMap::new(), 
            bools: HashMap::new(), 
            ints: HashMap::new(),
        };
        self_.implement_shema(schema);
        self_.parse_arguments(args);
        self_
    }

    fn implement_shema(&mut self, schema: &str) {
        for s in schema.split(",") {
            match s.chars().last().unwrap() {
                '#' => { &self.ints.insert(cut_last_char(s).to_string(), 0); },
                '*' => { &self.strings.insert(cut_last_char(s).to_string(), String::new()); },
                _ => { &self.bools.insert(s.to_string(), false); },
            }
        }
    }

    fn parse_arguments(&mut self, args: Vec<String>) {
        let args_ = args.clone();
        for mut arg in args {
            arg = cut_first_char(arg.as_str()).to_string();
            if self.is_bool(&arg) { 
                &self.bools.insert(
                    arg, 
                    true
                ); 
            } else if self.is_i32(&arg) { 
                &self.ints
                .insert(
                    arg.clone(), 
                    get_arg_value(
                        args_.clone(), 
                        &arg.clone()
                    ).parse::<i32>().unwrap()
                ); 
            } else if self.is_str(&arg) { 
                &self.strings.insert(
                    arg.clone(), 
                    get_arg_value(
                        args_.clone(), 
                        &arg.clone()
                    ).to_string()
                ); 
            }
        }
    }

    pub fn is_bool(&self, key: &str) -> bool{
        is_some(
            self.clone().bools
            .keys().into_iter()
            .find(|k| k.as_str() == key))
    }    
    pub fn is_i32(&self, key: &str) -> bool{
        is_some(
            self.clone().ints
            .keys().into_iter()
            .find(|k| k.as_str() == key))
    }
    pub fn is_str(&self, key: &str) -> bool{
        is_some(
            self.clone().strings
            .keys().into_iter()
            .find(|k| k.as_str() == key))
    }
    
    
    pub fn get_i32(&self, name: &str) -> i32{
        match &self.ints.get(name) {
            Some(v) => **v,
            None => 0,
        }
    }
    pub fn get_str(&self, name: &str) -> &str{
        match &self.strings.get(name) {
            Some(v) => v.as_str(),
            None => "",
        }
    }
    pub fn get_bool(&self, name: &str) -> bool{
        match &self.bools.get(name) {
            Some(v) => **v,
            None => false,
        }
    }
}
