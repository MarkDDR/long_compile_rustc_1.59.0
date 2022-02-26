#![allow(dead_code)]

mod big_struct;

fn main() {
    let _foo: big_struct::BigStruct =
        serde_json::from_str(include_str!("../big_json.json")).unwrap();
    // generate_big_struct(100);
}

fn generate_big_struct(n: usize) {
    use std::fmt::Write;
    let mut out_struct = String::from(
        "#[derive(serde::Deserialize)]
pub struct BigStruct {
",
    );
    let mut out_json = String::from("{");

    for i in 0..n {
        writeln!(&mut out_struct, "field_{}: String,", i).unwrap();
        let trailing_comma = if i == n - 1 { "" } else { "," };
        writeln!(&mut out_json, r#""field_{}": "hello"{}"#, i, trailing_comma).unwrap();
    }
    writeln!(&mut out_struct, "}}").unwrap();
    writeln!(&mut out_json, "}}").unwrap();

    std::fs::write("src/big_struct.rs", &out_struct).unwrap();
    std::fs::write("big_json.json", &out_json).unwrap();
}
