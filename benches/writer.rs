#![feature(test)]

#[macro_use]
extern crate json;
extern crate test;

// use std::io::Write;
use test::Bencher;
use json::writer::{JsonWriter, ValueWriter};
use json::{ToJson, Generator, DumpGenerator};

#[bench]
fn make_string(b: &mut Bencher) {
    b.iter(|| {
        r#"{"foo":"bar","baz":"qux","doge":["to","the","moon"]}"#.to_owned()
    });
}

#[bench]
fn dummy_write(b: &mut Bencher) {
    b.iter(|| {
        let mut gen = DumpGenerator::new();

        gen.write_char(b'{').unwrap();
        gen.write_str("foo").unwrap();
        gen.write_char(b':').unwrap();
        gen.write_str("bar").unwrap();
        gen.write_char(b',').unwrap();
        gen.write_str("baz").unwrap();
        gen.write_char(b':').unwrap();
        gen.write_str("qux").unwrap();
        gen.write_char(b',').unwrap();
        gen.write_str("doge").unwrap();
        gen.write_char(b':').unwrap();
        gen.write_char(b'[').unwrap();
        gen.write_str("to").unwrap();
        gen.write_char(b',').unwrap();
        gen.write_str("the").unwrap();
        gen.write_char(b',').unwrap();
        gen.write_str("moon").unwrap();
        gen.write_char(b']').unwrap();
        gen.write_char(b'}').unwrap();

        gen.consume()
    });
}

#[bench]
fn api_write(b: &mut Bencher) {
    b.iter(|| {
        JsonWriter::new().object()
            .key("foo").value("bar")
            .key("baz").value("qux")
            .key("doge").array()
                .value("to")
                .value("the")
                .value("moon")
                .close()
            .close()
    });
}

#[bench]
fn json_alloc(b: &mut Bencher) {
    b.iter(|| {
        let _obj = object!{
            "foo" => "bar",
            "baz" => "qux",
            "doge" => array!["to", "the", "moon"]
        };
    });
}

#[bench]
fn json_stringify(b: &mut Bencher) {
    b.iter(|| {
        let obj = object!{
            "foo" => "bar",
            "baz" => "qux",
            "doge" => array!["to", "the", "moon"]
        };

        json::stringify(obj)
    });
}
