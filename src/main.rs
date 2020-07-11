mod minify;
mod validation;
mod engine;
mod query;
mod result;
mod reader;

/// syntax:
/// .path
/// .[0]
/// .key.next
///
fn main() {
    let json: &[u8] = r#"
{
  "name": "alex",
  "boy": true,
  "age": 32,
  "hobbies": ["cooking", "guitar"],
  "nested": {"foo": "bar", "herp": false, "something": null},
  "something":  "else"
}
    "#.as_bytes();

    let json2: &[u8] = r#"
{
  "name": "alex",
  "boy": true,
  "age": 32,
  "nested": {"foo": "bar", "herp": false, "something": null},
  "something":  "else"
}
    "#.as_bytes();
    unsafe {
        // engine::extract(json, "something".as_bytes());
        // engine::extract(json, "hobbies".as_bytes());
        // engine::extract(json, "nested".as_bytes());
        // engine::extract(json, "nested.herp".as_bytes());
        engine::validate(json);
        println!("{}", std::str::from_utf8_unchecked(&minify::minify(json).unwrap()));
    }
}