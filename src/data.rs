use std::collections::HashMap;
use std::convert::From;
use std::fmt;
use std::ops::{Index, IndexMut};

use indexmap::IndexMap;
use serde_json::{json, Value as JsonValue};

use crate::error::Result;

pub trait Data {
    fn to_json(&self) -> Result<String>;
    fn to_cesr(&self) -> Result<String>;
    fn to_cesrb(&self) -> Result<Vec<u8>>;
}

type Array = Vec<Value>;
type Object = IndexMap<String, Value>;

#[derive(Debug, PartialEq, Clone)]
pub struct Number {
    f: f64,
    i: i64,
    float: bool,
}

impl From<f64> for Number {
    fn from(f: f64) -> Self {
        Self { f, i: 0, float: true }
    }
}

impl From<i64> for Number {
    fn from(i: i64) -> Self {
        Self { f: 0.0, i, float: false }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Value {
    Null,
    Boolean(bool),
    Number(Number),
    String(String),
    Array(Array),
    Object(Object),
}

impl Value {
    pub fn to_bool(&self) -> bool {
        bool::from(self)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        String::from(self)
    }

    pub fn to_i64(&self) -> i64 {
        i64::from(self)
    }

    pub fn to_f64(&self) -> f64 {
        f64::from(self)
    }

    pub fn to_vec(&self) -> Vec<Value> {
        Vec::from(self)
    }

    pub fn to_map(&self) -> IndexMap<String, Value> {
        IndexMap::from(self)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = match self.to_json() {
            Ok(j) => j,
            Err(_) => return Err(fmt::Error),
        };
        write!(f, "{}", json)
    }
}

impl Index<usize> for Value {
    type Output = Value;
    fn index(&self, i: usize) -> &Self::Output {
        match self {
            Value::Array(a) => &a[i],
            Value::Object(o) => &o[i],
            _ => todo!(),
        }
    }
}

impl Index<&str> for Value {
    type Output = Value;
    fn index(&self, i: &str) -> &Self::Output {
        match self {
            Value::Object(o) => &o[i],
            _ => todo!(),
        }
    }
}

impl IndexMut<usize> for Value {
    fn index_mut(&mut self, i: usize) -> &mut Value {
        match self {
            Value::Array(a) => &mut a[i],
            Value::Object(o) => &mut o[i],
            _ => todo!(),
        }
    }
}

impl IndexMut<&str> for Value {
    fn index_mut(&mut self, i: &str) -> &mut Value {
        match self {
            Value::Object(o) => &mut o[i],
            _ => todo!(),
        }
    }
}

impl Data for Value {
    fn to_json(&self) -> Result<String> {
        Ok(match self {
            Self::Null => "null".to_string(),
            Self::Boolean(b) => json!(b).to_string(),
            Self::Number(n) => {
                if n.float {
                    json!(n.f).to_string()
                } else {
                    json!(n.i).to_string()
                }
            }
            Self::String(s) => json!(s).to_string(),
            Self::Array(a) => {
                let mut v = Vec::new();
                for element in a {
                    v.push(element.to_json()?);
                }
                format!("[{}]", v.join(","))
            }
            Self::Object(o) => {
                let mut v = Vec::new();
                for (key, value) in o {
                    v.push(format!("{}:{}", json!(key), value.to_json()?));
                }
                format!("{{{}}}", v.join(","))
            }
        })
    }

    fn to_cesr(&self) -> Result<String> {
        unimplemented!();
    }

    fn to_cesrb(&self) -> Result<Vec<u8>> {
        unimplemented!();
    }
}

impl From<f32> for Value {
    fn from(x: f32) -> Self {
        Self::Number(Number::from(x as f64))
    }
}

impl From<f64> for Value {
    fn from(x: f64) -> Self {
        Self::Number(Number::from(x))
    }
}

impl From<i8> for Value {
    fn from(i: i8) -> Self {
        Self::Number(Number::from(i as i64))
    }
}

impl From<i16> for Value {
    fn from(i: i16) -> Self {
        Self::Number(Number::from(i as i64))
    }
}

impl From<i32> for Value {
    fn from(i: i32) -> Self {
        Self::Number(Number::from(i as i64))
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Self::Number(Number::from(i))
    }
}

impl From<u8> for Value {
    fn from(i: u8) -> Self {
        Self::Number(Number::from(i as i64))
    }
}

impl From<u16> for Value {
    fn from(i: u16) -> Self {
        Self::Number(Number::from(i as i64))
    }
}

impl From<u32> for Value {
    fn from(i: u32) -> Self {
        Self::Number(Number::from(i as i64))
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<&String> for Value {
    fn from(s: &String) -> Self {
        Self::String(s.clone())
    }
}

impl From<&[Value]> for Value {
    fn from(a: &[Value]) -> Self {
        Self::Array(a.to_vec())
    }
}

impl From<&HashMap<String, Value>> for Value {
    fn from(h: &HashMap<String, Value>) -> Self {
        let mut map = IndexMap::new();
        for (k, v) in h {
            map.insert(k.to_string(), v.clone());
        }
        Self::Object(map)
    }
}

impl From<&IndexMap<String, Value>> for Value {
    fn from(m: &IndexMap<String, Value>) -> Self {
        Self::Object(m.clone())
    }
}

impl From<&JsonValue> for Value {
    fn from(v: &JsonValue) -> Self {
        match v {
            JsonValue::Null => Self::Null,
            JsonValue::Bool(b) => Self::Boolean(*b),
            JsonValue::Number(n) => {
                if n.to_string().contains('.') {
                    Self::Number(Number::from(n.as_f64().unwrap()))
                } else {
                    Self::Number(Number::from(n.as_i64().unwrap()))
                }
            }
            JsonValue::String(s) => Self::String(s.clone()),
            JsonValue::Array(a) => {
                let mut v = Array::new();
                for e in a {
                    v.push(Self::from(e));
                }
                Self::Array(v)
            }
            JsonValue::Object(o) => {
                let mut m = Object::new();
                for (k, v) in o {
                    m.insert(k.clone(), Self::from(v));
                }
                Self::Object(m)
            }
        }
    }
}

impl From<&Value> for String {
    fn from(v: &Value) -> Self {
        match v {
            Value::String(s) => s.clone(),
            _ => todo!(),
        }
    }
}

impl From<&Value> for bool {
    fn from(v: &Value) -> Self {
        match v {
            Value::Boolean(b) => *b,
            _ => todo!(),
        }
    }
}

impl From<&Value> for i64 {
    fn from(v: &Value) -> Self {
        match v {
            Value::Number(n) => {
                if !n.float {
                    n.i
                } else {
                    n.f as i64
                }
            }
            _ => todo!(),
        }
    }
}

impl From<&Value> for f64 {
    fn from(v: &Value) -> Self {
        match v {
            Value::Number(n) => {
                if n.float {
                    n.f
                } else {
                    n.i as f64
                }
            }
            _ => todo!(),
        }
    }
}

impl From<&Value> for Vec<Value> {
    fn from(v: &Value) -> Self {
        match v {
            Value::Array(a) => a.clone(),
            _ => todo!(),
        }
    }
}

impl From<&Value> for IndexMap<String, Value> {
    fn from(v: &Value) -> Self {
        match v {
            Value::Object(o) => o.clone(),
            _ => todo!(),
        }
    }
}

#[macro_export]
macro_rules! data {
    ($($data:tt)+) => {
        data_internal!($($data)+)
    };
}

macro_rules! data_internal {
    // arrays

    // Done with trailing comma.
    (@array [$($elems:expr,)*]) => {
        data_internal_vec![$($elems,)*]
    };

    // Done without trailing comma.
    (@array [$($elems:expr),*]) => {
        data_internal_vec![$($elems),*]
    };

    // Next element is `null`.
    (@array [$($elems:expr,)*] null $($rest:tt)*) => {
        data_internal!(@array [$($elems,)* data_internal!(null)] $($rest)*)
    };

    // Next element is `true`.
    (@array [$($elems:expr,)*] true $($rest:tt)*) => {
        data_internal!(@array [$($elems,)* data_internal!(true)] $($rest)*)
    };

    // Next element is `false`.
    (@array [$($elems:expr,)*] false $($rest:tt)*) => {
        data_internal!(@array [$($elems,)* data_internal!(false)] $($rest)*)
    };

    // Next element is an array.
    (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
        data_internal!(@array [$($elems,)* data_internal!([$($array)*])] $($rest)*)
    };

    // Next element is a map.
    (@array [$($elems:expr,)*] {$($map:tt)*} $($rest:tt)*) => {
        data_internal!(@array [$($elems,)* data_internal!({$($map)*})] $($rest)*)
    };

    // Next element is an expression followed by comma.
    (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        data_internal!(@array [$($elems,)* data_internal!($next),] $($rest)*)
    };

    // Last element is an expression with no trailing comma.
    (@array [$($elems:expr,)*] $last:expr) => {
        data_internal!(@array [$($elems,)* data_internal!($last)])
    };

    // Comma after the most recent element.
    (@array [$($elems:expr),*] , $($rest:tt)*) => {
        data_internal!(@array [$($elems,)*] $($rest)*)
    };

    // Unexpected token after most recent element.
    (@array [$($elems:expr),*] $unexpected:tt $($rest:tt)*) => {
        data_unexpected!($unexpected)
    };

    // objects

    // Done.
    (@object $object:ident () () ()) => {};

    // Insert the current entry followed by trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        let _ = $object.insert(($($key)+).into(), $value);
        data_internal!(@object $object () ($($rest)*) ($($rest)*));
    };

    // Current entry followed by unexpected token.
    (@object $object:ident [$($key:tt)+] ($value:expr) $unexpected:tt $($rest:tt)*) => {
        data_unexpected!($unexpected);
    };

    // Insert the last entry without trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr)) => {
        let _ = $object.insert(($($key)+).into(), $value);
    };

    // Next value is `null`.
    (@object $object:ident ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => {
        data_internal!(@object $object [$($key)+] (data_internal!(null)) $($rest)*);
    };

    // Next value is `true`.
    (@object $object:ident ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
        data_internal!(@object $object [$($key)+] (data_internal!(true)) $($rest)*);
    };

    // Next value is `false`.
    (@object $object:ident ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
        data_internal!(@object $object [$($key)+] (data_internal!(false)) $($rest)*);
    };

    // Next value is an array.
    (@object $object:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        data_internal!(@object $object [$($key)+] (data_internal!([$($array)*])) $($rest)*);
    };

    // Next value is a map.
    (@object $object:ident ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        data_internal!(@object $object [$($key)+] (data_internal!({$($map)*})) $($rest)*);
    };

    // Next value is an expression followed by comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        data_internal!(@object $object [$($key)+] (data_internal!($value)) , $($rest)*);
    };

    // Last value is an expression with no trailing comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        data_internal!(@object $object [$($key)+] (data_internal!($value)));
    };

    // Missing value for last entry. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)+) (:) $copy:tt) => {
        // "unexpected end of macro invocation"
        data_internal!();
    };

    // Missing colon and value for last entry. Trigger a reasonable error
    // message.
    (@object $object:ident ($($key:tt)+) () $copy:tt) => {
        // "unexpected end of macro invocation"
        data_internal!();
    };

    // Misplaced colon. Trigger a reasonable error message.
    (@object $object:ident () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `:`".
        data_unexpected!($colon);
    };

    // Found a comma inside a key. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `,`".
        data_unexpected!($comma);
    };

    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@object $object:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        data_internal!(@object $object ($key) (: $($rest)*) (: $($rest)*));
    };

    // Refuse to absorb colon token into key expression.
    (@object $object:ident ($($key:tt)*) (: $($unexpected:tt)+) $copy:tt) => {
        data_expect_expr_comma!($($unexpected)+);
    };

    // Munch a token into the current key.
    (@object $object:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        data_internal!(@object $object ($($key)* $tt) ($($rest)*) ($($rest)*));
    };

    // core logic

    (null) => {
        $crate::data::Value::Null
    };

    (true) => {
        $crate::data::Value::Boolean(true)
    };

    (false) => {
        $crate::data::Value::Boolean(false)
    };

    ([]) => {
        $crate::data::Value::Array($crate::data::Array::new())
    };

    ([ $($tt:tt)+ ]) => {{
        data_internal!(@array [] $($tt)+)
    }};

    ({}) => {
        $crate::data::Value::Object($crate::data::Object::new())
    };

    ({ $($tt:tt)+ }) => {{
        let mut object = $crate::data::Object::new();
        data_internal!(@object object () ($($tt)+) ($($tt)+));
        $crate::data::Value::Object(object)
    }};

    ($other:expr) => {
        $crate::data::Value::from($other)
    }
}

macro_rules! data_internal_vec {
    ($($content:tt)*) => {{
        $crate::data::Value::Array(vec![$($content)*])
    }};
}

macro_rules! data_unexpected {
    () => {};
}

macro_rules! data_expect_expr_comma {
    ($e:expr , $($tt:tt)*) => {};
}

pub use data;

#[cfg(test)]
mod test {
    use crate::data::{data, Data, Value};

    #[test]
    fn data_macro() {
        let x: i64 = -1234567890;
        let s = "string".to_string();

        let mut d = data!({
            "thing": 2,
            "other thing": [&s, 1.666, x, true, {"nested array": [{}, []]}],
            "last thing": null
        });

        // to_json()
        assert_eq!(
            d.to_json().unwrap(),
            "{\"thing\":2,\"other thing\":[\"string\",1.666,-1234567890,true,{\"nested array\":[{},[]]}],\"last thing\":null}"
        );

        // query/indexing
        assert_eq!(d["thing"], d[0]); // we can index into an object with an integer or string key
        assert_ne!(d["thing"], d[1]);

        // display
        assert_eq!(format!("{}", d["thing"]), "2");

        // data extraction
        assert_eq!(d["last thing"], Value::Null);
        assert!(d["other thing"][3].to_bool());
        assert_eq!(d["thing"].to_i64(), 2);
        assert_eq!(d["other thing"][1].to_f64(), 1.666);
        assert_eq!(d["other thing"][0].to_string(), "string");
        assert_eq!(d["other thing"][4]["nested array"][1].to_vec(), vec![]);
        assert_eq!(d["other thing"][4]["nested array"][0].to_map(), indexmap::IndexMap::new());

        // mutability
        d["thing"] = data!({"something more complex": {"key": 987654321 }});
        assert_eq!(
            d.to_json().unwrap(),
            "{\"thing\":{\"something more complex\":{\"key\":987654321}},\"other thing\":[\"string\",1.666,-1234567890,true,{\"nested array\":[{},[]]}],\"last thing\":null}"
        );

        // serde_json parsing interop
        let v: serde_json::Value = serde_json::from_str(&d.to_json().unwrap()).unwrap();
        let d2 = Value::from(&v);
        assert_eq!(d.to_json().unwrap(), d2.to_json().unwrap());
    }
}
