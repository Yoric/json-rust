use JsonValue;
use object::Object;
use number::Number;
use codegen::Generator;
use std::io;
use std::iter::Iterator;
use std::hash::Hash;
use std::collections::{BTreeMap, HashMap};

type IoResult = io::Result<()>;

pub trait ToJson<G: Generator> {
    fn generate(&self, &mut G) -> IoResult;
}

impl<G: Generator> ToJson<G> for JsonValue {
    fn generate(&self, gen: &mut G) -> IoResult {
        match *self {
            JsonValue::Null               => gen.write(b"null"),
            JsonValue::Short(ref short)   => gen.write_str(short.as_str()),
            JsonValue::String(ref string) => gen.write_str(string),
            JsonValue::Number(ref number) => gen.write_number(number),
            JsonValue::Boolean(true)      => gen.write(b"true"),
            JsonValue::Boolean(false)     => gen.write(b"false"),
            JsonValue::Array(ref array)   => generate_array(array.iter(), gen),
            JsonValue::Object(ref object) => generate_object(object.iter(), gen),
        }
    }
}

#[inline]
fn generate_array<'a, G, T, I>(mut iter: I, gen: &mut G) -> IoResult where
    G: Generator,
    T: ToJson<G> + 'a,
    I: Iterator<Item = &'a T> + 'a
{
    try!(gen.write_char(b'['));

    match iter.next() {
        Some(item) => {
            gen.indent();
            try!(gen.new_line());
            try!(ToJson::generate(item, gen));
        },
        None => {
            try!(gen.write_char(b']'));
            return Ok(());
        }
    }

    for item in iter {
        try!(gen.write_char(b','));
        try!(gen.new_line());
        try!(ToJson::generate(item, gen));
    }

    gen.dedent();
    try!(gen.new_line());
    gen.write_char(b']')
}

#[inline]
fn generate_object<'a, G, K, V, I>(mut iter: I, gen: &mut G) -> IoResult where
    G: Generator,
    K: AsRef<str> + 'a,
    V: ToJson<G> + 'a,
    I: Iterator<Item = (K, &'a V)> + 'a
{
    try!(gen.write_char(b'{'));

    if let Some((key, value)) = iter.next() {
        gen.indent();
        try!(gen.new_line());
        try!(gen.write_str(key.as_ref()));
        try!(gen.write_min(b": ", b':'));
        try!(ToJson::generate(value, gen));
    } else {
        try!(gen.write_char(b'}'));
        return Ok(());
    }

    for (key, value) in iter {
        try!(gen.write_char(b','));
        try!(gen.new_line());
        try!(gen.write_str(key.as_ref()));
        try!(gen.write_min(b": ", b':'));
        try!(ToJson::generate(value, gen));
    }

    gen.dedent();
    try!(gen.new_line());
    gen.write_char(b'}')
}

impl<G: Generator> ToJson<G> for () {
    #[inline]
    fn generate(&self, gen: &mut G) -> IoResult {
        gen.write(b"null")
    }
}

impl<G: Generator> ToJson<G> for bool {
    #[inline]
    fn generate(&self, gen: &mut G) -> IoResult {
        gen.write(if *self { b"true" } else { b"false" })
    }
}

impl<'a, G: Generator> ToJson<G> for &'a str {
    #[inline]
    fn generate(&self, gen: &mut G) -> IoResult {
        gen.write_str(self)
    }
}

impl<G: Generator> ToJson<G> for String {
    #[inline]
    fn generate(&self, gen: &mut G) -> IoResult {
        gen.write_str(self.as_str())
    }
}

impl<G: Generator, T: ToJson<G>> ToJson<G> for Option<T> {
    #[inline]
    fn generate(&self, gen: &mut G) -> IoResult {
        match *self {
            Some(ref val) => ToJson::generate(val, gen),
            None          => gen.write(b"null"),
        }
    }
}

impl<G: Generator, T: ToJson<G>> ToJson<G> for Vec<T> {
    fn generate(&self, gen: &mut G) -> IoResult {
        generate_array(self.iter(), gen)
    }
}

impl<'a, G: Generator, T: ToJson<G>> ToJson<G> for &'a [T] {
    fn generate(&self, gen: &mut G) -> IoResult {
        generate_array(self.iter(), gen)
    }
}

impl<'a, G, K, V> ToJson<G> for BTreeMap<K, V> where
    G: Generator,
    K: AsRef<str> + 'a,
    V: ToJson<G> + 'a,
{
    fn generate(&self, gen: &mut G) -> IoResult {
        generate_object(self.iter(), gen)
    }
}

impl<'a, G, K, V> ToJson<G> for HashMap<K, V> where
    G: Generator,
    K: AsRef<str> + 'a + Eq + Hash,
    V: ToJson<G> + 'a,
{
    fn generate(&self, gen: &mut G) -> IoResult {
        generate_object(self.iter(), gen)
    }
}

impl<G: Generator> ToJson<G> for Object {
    fn generate(&self, gen: &mut G) -> IoResult {
        generate_object(self.iter(), gen)
    }
}

impl<G: Generator, N: Clone + Into<Number>> ToJson<G> for N {
    #[inline]
    fn generate(&self, gen: &mut G) -> IoResult {
        gen.write_number(&self.clone().into())
    }
}
