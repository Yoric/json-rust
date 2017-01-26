use std::ptr;
use codegen::{ToJson, Generator, DumpGenerator};

pub struct JsonWriter {
    gen: DumpGenerator
}

impl JsonWriter {
    #[inline]
    pub fn new() -> Self {
        JsonWriter {
            gen: DumpGenerator::new()
        }
    }
}

pub trait ValueWriter: Sized {
    type Root;

    #[inline]
    fn gen(&mut self) -> &mut DumpGenerator;

    #[inline]
    fn pop(self) -> Self::Root;

    #[inline]
    fn before_value(&mut self) {}

    #[inline]
    fn object(mut self) -> EmptyObjectWriter<Self> {
        self.before_value();

        self.gen().write_char(b'{');

        EmptyObjectWriter {
            root: self
        }
    }

    #[inline]
    fn array(mut self) -> EmptyArrayWriter<Self> {
        self.before_value();

        self.gen().write_char(b'[');

        EmptyArrayWriter {
            root: self
        }
    }

    #[inline]
    fn value<T: ToJson<DumpGenerator>>(mut self, val: T) -> Self::Root {
        self.before_value();

        val.generate(self.gen());

        self.pop()
    }
}

impl ValueWriter for JsonWriter {
    type Root = String;

    #[inline]
    fn gen(&mut self) -> &mut DumpGenerator {
        &mut self.gen
    }

    #[inline]
    fn pop(self) -> Self::Root {
        self.gen.consume()
    }
}

#[derive(Debug)]
pub struct EmptyObjectWriter<V: ValueWriter> {
    root: V
}

#[derive(Debug)]
pub struct ObjectWriter<V: ValueWriter> {
    root: V
}

impl<V: ValueWriter> EmptyObjectWriter<V> {
    #[inline]
    pub fn close(mut self) -> V::Root {
        self.root.gen().write_char(b'}');

        self.root.pop()
    }

    #[inline]
    pub fn key(mut self, key: &str) -> ObjectValueWriter<ObjectWriter<V>> {
        self.root.gen().write_str(key);
        self.root.gen().write_char(b':');

        ObjectValueWriter {
            root: ObjectWriter {
                root: self.root
            }
        }
    }
}

impl<V: ValueWriter> ObjectWriter<V> {
    #[inline]
    pub fn close(mut self) -> V::Root {
        self.root.gen().write_char(b'}');

        self.root.pop()
    }

    #[inline]
    pub fn key(mut self, name: &str) -> ObjectValueWriter<Self> {
        self.root.gen().write_char(b',');
        self.root.gen().write_str(name);
        self.root.gen().write_char(b':');

        ObjectValueWriter {
            root: self
        }
    }
}

#[derive(Debug)]
pub struct ObjectValueWriter<ObjectWriter> {
    root: ObjectWriter
}

impl<V: ValueWriter> ValueWriter for ObjectValueWriter<ObjectWriter<V>> {
    type Root = ObjectWriter<V>;

    #[inline]
    fn gen(&mut self) -> &mut DumpGenerator {
        self.root.root.gen()
    }

    #[inline]
    fn pop(self) -> Self::Root {
        self.root
    }
}

#[derive(Debug)]
pub struct EmptyArrayWriter<V: ValueWriter> {
    root: V
}

#[derive(Debug)]
pub struct ArrayWriter<V: ValueWriter> {
    root: V
}

impl<V: ValueWriter> EmptyArrayWriter<V> {
    #[inline]
    pub fn close(mut self) -> V::Root {
        self.root.gen().write_char(b']');

        self.root.pop()
    }
}

impl<V: ValueWriter> ValueWriter for EmptyArrayWriter<V> {
    type Root = ArrayWriter<V>;

    #[inline]
    fn gen(&mut self) -> &mut DumpGenerator {
        self.root.gen()
    }

    #[inline]
    fn pop(self) -> Self::Root {
        ArrayWriter {
            root: self.root
        }
    }
}

impl<V: ValueWriter> ValueWriter for ArrayWriter<V> {
    type Root = ArrayWriter<V>;

    #[inline]
    fn gen(&mut self) -> &mut DumpGenerator {
        self.root.gen()
    }

    #[inline]
    fn pop(self) -> Self::Root {
        self
    }

    #[inline]
    fn before_value(&mut self) {
        self.root.gen().write_char(b',');
    }
}

impl<V: ValueWriter> ArrayWriter<V> {
    #[inline]
    pub fn close(mut self) -> V::Root {
        self.root.gen().write_char(b']');

        self.root.pop()
    }
}
