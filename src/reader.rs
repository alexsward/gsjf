use std::borrow::Borrow;

pub trait Reader {

    /// Returns the current position of the [Reader]
    fn position(&mut self) -> usize;

    /// Next retrieves the next token, or None if nothing is available
    /// The position will be moved forward
    fn next(&mut self) -> Option<u8>;

    /// Retrieves the previous token, moves the position back by one
    fn prev(&mut self) -> Option<u8>;

    /// Retrieves the next token, if it exists.
    /// Does not advance the Reader's position
    fn peek(&mut self) -> Option<u8>;

    fn last(&self) -> Option<u8>;

    /// Retrieves the current token, does not advance the position
    fn current(&self) -> Option<u8>;

    /// Reads a &str
    fn read_string(&mut self) -> Option<(usize, usize)>;

    fn read_number(&mut self) -> Option<JSONRange>;

    /// Reads a JSON object (array | object)
    fn read_json(&mut self) -> Option<(usize, usize)>;

    /// Reads a known number of bytes (for things like null, false, true, etc)
    fn read_known(&mut self, bytes: usize) -> Option<(usize, usize)>;

    /// Finds a key in the current level of the JSON object
    fn find_key(&mut self, key: &[u8]) -> Option<(usize, usize)>;

    fn read_value(&mut self) -> Option<JSONValue>;

    /// Seeks to the provided position
    fn seek(&mut self, offset: usize);

    /// Resets to the beginning
    fn reset(&mut self);

    /// Scans through an open-close pair and returns the end position
    /// Example: reader.scan(b'{', b'}') scans through a JSON object
    fn scan(&mut self, open: u8, close: u8) -> Option<usize>;

    /// Skips ahead to the next occurrence of [val]
    fn skip_to_next(&mut self, val: u8) -> Option<usize>;

    fn skip_to_previous(&mut self, val: u8) -> Option<usize>;

    fn skip_past_whitespace(&mut self);

    fn print_at(&self, start: usize, end: usize);

    fn select(&mut self, range: JSONRange) -> Option<&[u8]>;
}

pub struct JSONValue {
    pub(crate) json_type: JSONType,
    pub(crate) range: JSONRange,
}

impl JSONValue {
    pub fn new(json_type: JSONType, range: (usize, usize)) -> JSONValue {
        JSONValue{ json_type, range, }
    }
}

pub enum JSONType {
    ARRAY,
    OBJECT,
    NULL,
    STRING,
    BOOLEAN,
    NUMBER
}

pub struct JSONReader<'a> {
    data: &'a [u8],
    offset: usize,
}

impl JSONReader<'_> {
    pub fn new(data: &[u8]) -> JSONReader {
        JSONReader{data, offset: 0}
    }
}

type JSONRange = (usize, usize);

impl Reader for JSONReader<'_> {

    fn position(&mut self) -> usize {
        match self.offset {
            0 => 0,
            _ => self.offset - 1
        }
    }

    fn next(&mut self) -> Option<u8> {
        if self.offset + 1 == self.data.len() {
            return None
        }
        let next: u8 = self.data[self.offset];
        self.offset += 1;
        Some(next)
    }

    fn prev(&mut self) -> Option<u8> {
        if self.offset == 0 {
            return None
        }
        let prev: u8 = self.data[self.offset - 1];
        self.offset -= 1;
        Some(prev)
    }

    fn peek(&mut self) -> Option<u8> {
        if self.offset + 1 == self.data.len() {
            return None
        }
        Some(self.data[self.offset])
    }

    fn last(&self) -> Option<u8> {
        if self.offset == 0 {
            return None
        }
        Some(self.data[self.offset - 1])
    }

    fn current(&self) -> Option<u8> {
        let token: u8 = match self.offset {
            0 => self.data[0],
            _ => self.data[self.offset - 1]
        };
        Some(token)
    }

    fn read_string(&mut self) -> Option<(usize, usize)> {
        let start: usize = self.offset;
        while let Some(token) = self.next() {
            match token {
                b'"' => break,
                b'\\' => self.next(), // advance one more position as it's an escaped quote
                _ => None
            };
        }
        Some((start, self.offset - 1)) // TODO: these -1's seem off
    }

    fn read_number(&mut self) -> Option<JSONRange> {
        None
    }

    fn read_json(&mut self) -> Option<(usize, usize)> {
        let start: usize = self.offset - 1; // TODO: these -1's seem off
        let end: Option<usize> = match self.current() {
            Some(b'{') => self.scan(b'{', b'}'),
            Some(b'[') => self.scan(b'[', b']'),
            _ => None
        };
        match end {
            None => None,
            Some(index) => Some((start, index))
        }
    }

    fn read_known(&mut self, bytes: usize) -> Option<(usize, usize)> {
        if self.offset + bytes > self.data.len() {
            return None
        }
        let start = self.offset - 1; // todo: feels gross? use self.position()
        self.seek(start + bytes);
        Some((start, self.offset))
    }

    fn find_key(&mut self, key: &[u8]) -> Option<(usize, usize)> {
        let mut hit_first_json: bool = false;
        while let Some(token) = self.next() {
            match token {
                b'{' | b'[' => {
                    if !hit_first_json {
                        hit_first_json = true;
                        continue;
                    }
                    if let None = self.read_json() {
                        // TODO: test case?
                        return None; // never ended, mis-formatted JSON
                    }
                },
                b'"' => {
                    let found: (usize, usize) = self.read_string().unwrap();
                    let extracted: &[u8] = self.data[found.0 .. found.1].borrow(); // TODO: give this a method
                    if extracted == key {
                        return Some(found);
                    }
                },
                _ => {}
            };
        };
        None
    }

    /// Assumes we're on a " after an object key
    fn read_value(&mut self) -> Option<JSONValue> {
        while let Some(token) = self.next() {
            match token {
                b'n' => return self.read_known(4).map(| range: JSONRange | {
                    JSONValue::new(JSONType::NULL, range)
                }),
                b'f' | b't' => return self.read_known(if token == b't' { 4 } else { 5 }).map(| range: JSONRange | {
                    JSONValue::new(JSONType::BOOLEAN, range)
                }),
                b'0'..=b'9' | b'-' | b'.' => return self.read_number().map(| range: JSONRange | {
                    JSONValue::new(JSONType::NUMBER, range)
                }),
                b'"' => return self.read_string().map(| range: JSONRange | {
                    JSONValue::new(JSONType::STRING, range)
                }),
                b'{' | b'[' => return self.read_json().map(| range: JSONRange | {
                    let json_type: JSONType = if token == b'{' { JSONType::OBJECT } else { JSONType::ARRAY };
                    JSONValue::new(json_type, range)
                }),
                _ => {}
            };
        }
        None
    }

    fn seek(&mut self, offset: usize) {
        self.offset = offset
    }

    fn reset(&mut self) {
        self.seek(0)
    }
    
    /// This function does not assume that you are already on an [open] character,
    /// it will first check if you are. If you are not, it assumes you are inside of an [open]
    fn scan(&mut self, open: u8, close: u8) -> Option<usize> {
        match self.current() {
            None => return None,
            Some(token) => {
                if token != open {
                    if self.skip_to_previous(open) == None {
                        return None
                    }
                }
            }
        }
        let mut depth: u8 = 1;
        while let Some(token) = self.next() {
            if token == open {
                depth += 1;
            } else if token == close {
                depth -= 1;
                if depth == 0 {
                    return Some(self.offset)
                }
            }
        }
        None
    }

    fn skip_to_next(&mut self, val: u8) -> Option<usize> {
        while let Some(token) = self.next() {
            if token == val {
                return Some(self.offset)
            }
        }
        None
    }

    fn skip_to_previous(&mut self, val: u8) -> Option<usize> {
        while let Some(token) = self.prev() {
            if token == val {
                return Some(self.offset);
            }
        }
        None
    }

    fn skip_past_whitespace(&mut self) {
        match self.peek() {
            None => return,
            Some(token) => {
                match token {
                    b' ' | b'\t' | b'\n' | b'\r' => {
                        self.next();
                    },
                    _ => return
                }
            }
        }
    }


    fn print_at(&self, start: usize, end: usize) {
        println!("printing: '{}'", std::str::from_utf8(self.data[start..end].borrow()).unwrap())
    }

    fn select(&mut self, range: (usize, usize)) -> Option<&[u8]> {
        Some(self.data[range.0..range.1].borrow())
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::{JSONReader, Reader};

    const JSON: &[u8] = r#"{"name":"alex","boy":true,"age":32,"hobbies":["cooking","guitar"],"nested":{"foo":"bar"}}"#.as_bytes();

    #[test]
    fn test_find_key() {
        assert_eq!(JSONReader::new(JSON).find_key("boy".as_bytes()), Some((16, 19)))
    }

    #[test]
    fn test_read_string() {
        let mut reader = JSONReader::new(JSON);
        reader.seek(9);
        assert_eq!(reader.read_string(), Some((9, 13)))
    }

    #[test]
    fn test_read_string__escaped() {
        // let mut reader: JSONReader = JSONReader::new("\"foo\"".as_bytes());
        // assert_eq!(reader.read_string(), )
    }
}