mod utils;

use utils::*;
use std::collections::HashMap;

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
            match s.chars().last() {
                Some(chr) => match chr {
                    '#' => { let _ = &self.ints.insert(cut_last_char(s).to_string(), 0); },
                    '*' => { let _ = &self.strings.insert(cut_last_char(s).to_string(), String::new()); },
                    _ => { let _ = &self.bools.insert(s.to_string(), false); },
                },
                None => panic!("Invaild args schema")
            }
        }
    }

    fn parse_arguments(&mut self, args: Vec<String>) {
        let args_ = args.clone();
        for mut arg in args {
            arg = cut_first_char(arg.as_str()).to_string();
            if self.is_bool(&arg) { 
                let _ = &self.bools.insert(
                    arg, 
                    true
                ); 
            } else if self.is_i32(&arg) { 
                let _ = &self.ints
                .insert(
                    arg.clone(), 
                    match get_arg_value(
                        args_.clone(), 
                        &arg.clone()
                    ).parse::<i32>() {
                        Ok(v) => v,
                        Err(_) => 0
                    }
                ); 
            } else if self.is_str(&arg) { 
                let _ = &self.strings.insert(
                    arg.clone(), 
                    get_arg_value(
                        args_.clone(), 
                        &arg.clone()
                    ).to_string()
                ); 
            }
        }
    }

    fn is_type<T>(&self, key: &str, map: HashMap<String, T>) -> bool{
        is_some(
            map.keys().into_iter()
            .find(|k| k.as_str() == key))
    }

    pub fn is_bool(&self, key: &str) -> bool{
        self.is_type(key, self.bools.clone())
    }
    pub fn is_i32(&self, key: &str) -> bool{
        self.is_type(key, self.ints.clone())

    }
    pub fn is_str(&self, key: &str) -> bool{
        self.is_type(key, self.strings.clone())
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

