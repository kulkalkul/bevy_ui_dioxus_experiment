use std::cell::RefCell;

use dioxus::{prelude::IntoAttributeValue, core::{exports::bumpalo::{Bump, boxed::Box as BumpBox}, AttributeValue, AnyValue}};

#[derive(PartialEq)]
pub struct Attr<T>(pub T);

impl<'a, T: PartialEq + 'static> IntoAttributeValue<'a> for Attr<T> {
    fn into_value(self, bump: &'a Bump) -> AttributeValue<'a> {
        let boxed: BumpBox<'a, dyn AnyValue> = unsafe { BumpBox::from_raw(bump.alloc(self)) };
        AttributeValue::Any(RefCell::new(Some(boxed)))
    }
}