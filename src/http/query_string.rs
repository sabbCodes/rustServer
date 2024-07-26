// use std::collections::btree_map::Entry;
use std::collections::HashMap;
use std::fmt::Debug;
// use std::vec;

#[derive(Debug)]
pub struct QueryString<'a> {
    data: HashMap<&'a str, Value<'a>>,
}

#[derive(Debug)]
pub enum Value<'a> {
    Single(&'a str),
    Multiple(Vec<&'a str>),
}

impl <'a> QueryString<'a> {
    pub fn get(&self, key:&str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'a> From<&'a str> for QueryString<'a> {
    fn from(s: &'a str) -> Self {
        let mut data  = HashMap::new();
        for sub_str in s.split("&"){
            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find("="){
                key = &sub_str[..i];
                val = &sub_str[i+1..];
            }

            data.entry(key)
                .and_modify(|existing_map: &mut Value| match existing_map {
                    Value::Single(old_val) => {
                        *existing_map = Value::Multiple(vec![old_val, val]);
                    }
                    Value::Multiple(vec) => {vec.push(val)}
                })
                .or_insert(Value::Single(val));
        }
        QueryString { data: data }
    }
}