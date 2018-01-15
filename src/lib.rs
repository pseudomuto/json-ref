#![deny(missing_docs)]

//! A simple tool for resolving/validation json-refs across documents.
//!
//! json-ref provides a CLI for joining multiple JSON/YAML documents by resolving `$ref` fields
//! according to the [JSON Ref](https://tools.ietf.org/html/draft-pbryan-zyp-json-ref-03) specs.

mod commands;

pub use commands::Command;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
