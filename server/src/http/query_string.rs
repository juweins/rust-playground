use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub struct QueryString<'buffer> {
    data: HashMap<&'buffer str, Value<'buffer>>
}

#[derive(Debug)]
pub enum Value<'buffer> {
    Single(&'buffer str),
    Multiple(Vec<&'buffer str>), // this is a heap allocated array
}

impl <'buffer> QueryString<'buffer> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// splits the query_string on '&' and puts fragments into HashMap
// example string: search=query+string&customer=john -> key=[value]
impl <'buffer> From<&'buffer str> for QueryString<'buffer> {
    fn from(slice: &'buffer str) -> Self{
        let mut data = HashMap::new();

        for fragment in slice.split('&'){
            let mut key = fragment;
            let mut value = "";

            if let Some(index) = fragment.find('=') {
                key = &fragment[..index];
                value = &fragment[index +1..];
            }

            // look for key in hashmap, insert value if not present
            data.entry(key)
            .and_modify(|existing: &mut Value| match existing {
                Value::Single(prev_value) => {
                    *existing = Value::Multiple(vec![prev_value, value]);
                }
                Value::Multiple(vector) => {vector.push(value)}
            })
            .or_insert(Value::Single(value));

        }


        QueryString{data}

    }
}