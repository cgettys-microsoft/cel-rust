use crate::common::types::Type;
use crate::common::value::Val;
use std::any::Any;
use std::string::String as StdString;

pub struct String(StdString);

impl Val for String {
    fn get_type(&self) -> Type {
        super::STRING_TYPE
    }

    fn into_inner(self) -> Box<dyn Any> {
        Box::new(self.0)
    }
}

impl From<StdString> for String {
    fn from(v: StdString) -> Self {
        Self(v)
    }
}

impl From<String> for StdString {
    fn from(v: String) -> Self {
        v.0
    }
}
