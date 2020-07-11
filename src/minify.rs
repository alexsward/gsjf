use crate::reader::{JSONReader, Reader};
use std::borrow::Borrow;

pub fn minify(json: &[u8]) -> Option<Box<[u8]>> {
    let mut reader: JSONReader = JSONReader::new(json);
    let mut new: Vec<u8> = vec!();
    while let Some(token) = reader.next() {
        match token {
            t if t > b' ' => {
                new.push(t);
                if t == b'"' {
                    while let Some(end_string_candidate) = reader.next() {
                        new.push(end_string_candidate);
                        match end_string_candidate {
                            b'"' => if reader.last() != Some(b'\\') {
                                break
                            }
                            _ => ()
                        }
                    }
                }
            }
            _ => {}
        }
        if token > b' ' {}
    }
    Some(new.into_boxed_slice())
}