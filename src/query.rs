/// This module models out the query language for gsjf.
///
/// Every query is separated by a | character
/// Queries themselves are made up of dot-separated pieces (.)
/// Key selections are simple strings:
///     - key selects "key" from the top-level object
/// Array selections are done via the [] operator
///     - [] selects the entire array
///     - [1] selects the first (0-indexed) element
///     - []
///
/// Some query examples:
/// key => value
/// key.key => value
/// key.[1] => first element of an array
/// [1] => first element of the array
///

#[derive(Debug, PartialEq)]
pub struct Query<'a> {
    raw: &'a [u8],
    pub(crate) components: Vec<QueryComponent<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct QueryComponent<'a> {
    pub(crate) path: &'a [u8],
}

impl<'a> Query<'a> {
    pub fn from(raw: &'a str) -> Query<'a> {
        let mut components: Vec<QueryComponent> = vec!();
        for piece in raw.split(".") {
            components.push(QueryComponent{ path: piece.as_bytes() });
        }
        Query{ raw: raw.as_bytes(), components }
    }

    // fn empty() -> Query<'a> {
    //     Query {
    //         raw: "".as_bytes(),
    //         components: vec!(),
    //     }
    // }
}

/// glob queries map to the standard unix glob patterns:
///     - *: matches any characters
///     - ?: matches any character
///     - [] matches characters in here
/// TODO: just use rust glob? it probably exists: https://research.swtch.com/glob
fn glob(query: &[u8], candidate: &[u8]) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use crate::query::Query;

    #[test]
    fn single_path() {
        // let expectation = Query{path: "foo".as_bytes(), sub_query: None};
        // assert_eq!(Query::from("foo"), expectation);
    }
}