use std::error::Error;
use crate::reader::{JSONReader, Reader};

/// Validates the read data
pub trait Validator {

    fn validate(&mut self) -> bool;

    fn validate_object(&mut self) -> bool;

    fn validate_array(&mut self) -> bool;

    fn validate_string(&mut self) -> bool;

    fn validate_boolean(&mut self) -> bool;

    fn validate_null(&mut self) -> bool;
}

impl Validator for JSONReader<'_> {
    fn validate(&mut self) -> bool {
        while let Some(token) = self.next() {
            match token {
                b'"' => {
                    self.validate_string();
                },
                b't' | b'f' => {
                    self.validate_boolean();
                },
                b'n' => {
                    self.validate_null();
                },
                b'{' => {
                    self.validate_object();
                },
                b'[' => {},
                _ => {}
            }
        }
        true
    }

    fn validate_object(&mut self) -> bool {
        while let Some(token) = self.next() {
            match token {
                b'}' => return true,
                b' ' | b'\t' | b'\n' | b'r' => (), // whitespace
                b'"' => { // we're now into a candidate for a key"
                    if !self.validate_string() {
                        return false
                    }
                    while let Some(maybe_colon) = self.next() {
                        match maybe_colon {
                            b':' => break,
                            b' ' | b'\t' | b'\n' | b'r' => continue,
                            _ => return false
                        }
                    }
                    if !self.validate() {
                        return false
                    }
                    while let Some(maybe_comma) = self.next() {
                        match maybe_comma {
                            b',' => break,
                            b' ' | b'\t' | b'\n' | b'r' => continue,
                            b'}' => return true,
                            _ => return false
                        }
                    }
                },
                _ => return false
            }
        }
        false // would have expected to hit the case b'}' => return true
    }

    fn validate_array(&mut self) -> bool {
        while let Some(token) = self.next() {
            match token {
                b']' => return true,
                b' ' | b'\t' | b'\n' | b'r' => (), // whitespace
                _ => {
                    if !self.validate() {
                        return false
                    }
                    while let Some(maybe_comma) = self.next() {
                        match maybe_comma {
                            b',' => break,
                            b' ' | b'\t' | b'\n' | b'r' => continue,
                            b']' => return true,
                            _ => return false
                        }
                    }
                }
            }
        }
        false
    }

    fn validate_string(&mut self) -> bool {
        while let Some(token) = self.next() {
            match token {
                b'"' => return true,
                b'\\' => { // escape characters
                    let escaped: Option<u8> = self.next();
                    if escaped.is_none() {
                        return false
                    }
                    let proper_escaped = match escaped.unwrap() {
                        b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't' => true,
                        b'u' => { // \uHEX,HEX,HEX,HEX
                            for _ in 0..4 {
                                if let Some(maybe_hex) = self.next() {
                                    match maybe_hex {
                                        b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' => (),
                                        _ => return false
                                    }
                                } else {
                                    return false
                                }
                            }
                            true
                        },
                        _ => false
                    };
                    if !proper_escaped {
                        return false
                    }
                }
                _ => ()
            }
        }
        true
    }

    fn validate_boolean(&mut self) -> bool {
        match self.current() {
            Some(b't') => self.next() == Some(b'r') && self.next() == Some(b'u') && self.next() == Some(b'e'),
            Some(b'f') => self.next() == Some(b'a') && self.next() == Some(b'l') && self.next() == Some(b's') && self.next() == Some(b'e'),
            _ => false
        }
    }

    fn validate_null(&mut self) -> bool {
        match self.current() {
            Some(b'n') => self.next() == Some(b'u') && self.next() == Some(b'l') && self.next() == Some(b'l'),
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::{JSONReader, Reader};
    use crate::validation::Validator;

    const JSON_BOOLEAN: &[u8] = r#"{"foo": true, "bar": false}"#.as_bytes();
    const JSON_NULL: &[u8] = r#"{"is": null, "isnt":nil}"#.as_bytes();
    const JSON_STRING: &[u8] = r#"{}"#.as_bytes();

    #[test]
    fn test_validate_boolean() {
        let mut reader: JSONReader = JSONReader::new(JSON_BOOLEAN);
        reader.seek(9);
        assert_eq!(reader.validate_boolean(), true);
        reader.seek(22);
        assert_eq!(reader.validate_boolean(), true);
    }

    #[test]
    fn test_validate_null() {
        let mut reader: JSONReader = JSONReader::new(JSON_NULL);
        reader.seek(7);
        assert_eq!(reader.validate_null(), true);
        reader.seek(44);
        assert_eq!(reader.validate_null(), false);
    }

    #[test]
    fn test_validate_string() {
    }
}