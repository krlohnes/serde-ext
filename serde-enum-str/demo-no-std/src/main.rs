#![no_std]

extern crate alloc;

use alloc::string::ToString as _;

use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Deserialize_enum_str, Serialize_enum_str, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
enum Foo {
    A,
    #[serde(rename = "B")]
    B,
    #[serde(other)]
    Other(alloc::string::String),
}

fn main() -> serde_json::Result<()> {
    assert_eq!(serde_json::to_string(&Foo::A).unwrap(), r#""a""#);
    assert_eq!(serde_json::from_str::<Foo>(r#""a""#).unwrap(), Foo::A);

    assert_eq!(
        serde_json::from_str::<Foo>(r#""c""#).unwrap(),
        Foo::Other("c".into())
    );

    assert_eq!(Foo::A.to_string(), "a");
    assert_eq!("a".parse::<Foo>().unwrap(), Foo::A);

    Ok(())
}