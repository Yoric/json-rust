// This is a private module that contans `PartialEq` and `From` trait
// implementations for `JsonValue`.

use std::collections::{ BTreeMap, HashMap };
use JsonValue;

use short::{ self, Short };
use number::Number;
use object::Object;

macro_rules! implement_eq {
    ($to:ident, $from:ty) => {
        impl PartialEq<$from> for JsonValue {
            fn eq(&self, other: &$from) -> bool {
                match *self {
                    JsonValue::$to(ref value) => value == other,
                    _                         => false
                }
            }
        }

        impl<'a> PartialEq<$from> for &'a JsonValue {
            fn eq(&self, other: &$from) -> bool {
                match **self {
                    JsonValue::$to(ref value) => value == other,
                    _                         => false
                }
            }
        }

        impl PartialEq<JsonValue> for $from {
            fn eq(&self, other: &JsonValue) -> bool {
                match *other {
                    JsonValue::$to(ref value) => value == self,
                    _ => false
                }
            }
        }
    }
}

macro_rules! implement {
    ($to:ident, $from:ty as num) => {
        impl From<$from> for JsonValue {
            fn from(val: $from) -> JsonValue {
                JsonValue::$to(val.into())
            }
        }

        implement_eq!($to, $from);
    };
    ($to:ident, $from:ty) => {
        impl From<$from> for JsonValue {
            fn from(val: $from) -> JsonValue {
                JsonValue::$to(val)
            }
        }

        implement_eq!($to, $from);
    }
}

impl<'a> From<&'a str> for JsonValue {
    fn from(val: &'a str) -> JsonValue {
        if val.len() <= short::MAX_LEN {
            JsonValue::Short(unsafe { Short::from_slice(val) })
        } else {
            JsonValue::String(val.into())
        }
    }
}

impl<T: Into<JsonValue>> From<Option<T>> for JsonValue {
    fn from(val: Option<T>) -> JsonValue {
        match val {
            Some(val) => val.into(),
            None      => JsonValue::Null,
        }
    }
}

impl<T: Into<JsonValue>> From<Vec<T>> for JsonValue {
    fn from(val: Vec<T>) -> JsonValue {
        JsonValue::Array(val.into_iter().map(Into::into).collect())
    }
}

impl<'a, T: Clone + Into<JsonValue>> From<&'a [T]> for JsonValue {
    fn from(val: &[T]) -> JsonValue {
        JsonValue::Array(val.into_iter().map(Clone::clone).map(Into::into).collect())
    }
}

macro_rules! impl_arr {
    ($count:expr) => {
        impl<T: Clone + Into<JsonValue>> From<[T; $count]> for JsonValue {
            fn from(val: [T; $count]) -> JsonValue {
                JsonValue::Array(val.into_iter().map(Clone::clone).map(Into::into).collect())
            }
        }
    }
}

impl_arr!(0);
impl_arr!(1);
impl_arr!(2);
impl_arr!(3);
impl_arr!(4);
impl_arr!(5);
impl_arr!(6);
impl_arr!(7);
impl_arr!(8);
impl_arr!(9);
impl_arr!(10);
impl_arr!(11);
impl_arr!(12);
impl_arr!(13);
impl_arr!(14);
impl_arr!(15);
impl_arr!(16);
impl_arr!(17);
impl_arr!(18);
impl_arr!(19);
impl_arr!(20);
impl_arr!(21);
impl_arr!(22);
impl_arr!(23);
impl_arr!(24);
impl_arr!(25);
impl_arr!(26);
impl_arr!(27);
impl_arr!(28);
impl_arr!(29);
impl_arr!(30);
impl_arr!(31);
impl_arr!(32);

impl From<HashMap<String, JsonValue>> for JsonValue {
    fn from(mut val: HashMap<String, JsonValue>) -> JsonValue {
        let mut object = Object::with_capacity(val.len());

        for (key, value) in val.drain() {
            object.insert(&key, value);
        }

        JsonValue::Object(object)
    }
}

impl From<BTreeMap<String, JsonValue>> for JsonValue {
    fn from(mut val: BTreeMap<String, JsonValue>) -> JsonValue {
        let mut object = Object::with_capacity(val.len());

        for (key, value) in val.iter_mut() {
            object.insert(key, value.take());
        }

        JsonValue::Object(object)
    }
}

impl<'a> PartialEq<&'a str> for JsonValue {
    fn eq(&self, other: &&str) -> bool {
        match *self {
            JsonValue::Short(ref value)  => value == *other,
            JsonValue::String(ref value) => value == *other,
            _ => false
        }
    }
}

impl<'a> PartialEq<JsonValue> for &'a str {
    fn eq(&self, other: &JsonValue) -> bool {
        match *other {
            JsonValue::Short(ref value)  => value == *self,
            JsonValue::String(ref value) => value == *self,
            _ => false
        }
    }
}

impl PartialEq<str> for JsonValue {
    fn eq(&self, other: &str) -> bool {
        match *self {
            JsonValue::Short(ref value)  => value == other,
            JsonValue::String(ref value) => value == other,
            _ => false
        }
    }
}

impl<'a> PartialEq<JsonValue> for str {
    fn eq(&self, other: &JsonValue) -> bool {
        match *other {
            JsonValue::Short(ref value)  => value == self,
            JsonValue::String(ref value) => value == self,
            _ => false
        }
    }
}

implement!(String, String);
implement!(Number, isize as num);
implement!(Number, usize as num);
implement!(Number, i8 as num);
implement!(Number, i16 as num);
implement!(Number, i32 as num);
implement!(Number, i64 as num);
implement!(Number, u8 as num);
implement!(Number, u16 as num);
implement!(Number, u32 as num);
implement!(Number, u64 as num);
implement!(Number, f32 as num);
implement!(Number, f64 as num);
implement!(Number, Number);
implement!(Object, Object);
implement!(Boolean, bool);
