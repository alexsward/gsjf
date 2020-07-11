use crate::reader::{JSONReader, Reader, JSONValue, JSONType};
use crate::query::Query;
use crate::validation::Validator;

pub unsafe fn extract(json: &[u8], raw: &[u8]) {
    let mut reader: JSONReader = JSONReader::new(json);
    let query: Query = Query::from(std::str::from_utf8_unchecked(raw));
    for (idx, selector) in query.components.iter().enumerate() {
        println!("performing selector: {}", std::str::from_utf8_unchecked(selector.path));
        match reader.find_key(selector.path) {
            None => return,
            Some((start, end)) => {
                reader.print_at(start, end);
                let val: JSONValue = reader.read_value().unwrap();
                match val.json_type {
                    JSONType::STRING => reader.print_at(val.range.0, val.range.1),
                    JSONType::OBJECT | JSONType::ARRAY => {
                        reader.print_at(val.range.0, val.range.1);
                        reader.seek(val.range.0);
                    },
                    JSONType::BOOLEAN => {
                        reader.print_at(val.range.0, val.range.1);
                    }
                    _ => println!("LOL WUT")
                }
            }
        };
    }
}

pub fn validate(json :&[u8]) {
    let mut reader: JSONReader = JSONReader::new(json);
    println!("valid json?: {}", reader.validate());
}