use std::cell::RefCell;

use bevy::{ui::Style, prelude::Component};
use dioxus::{prelude::IntoAttributeValue, core::{exports::bumpalo::{Bump, boxed::Box as BumpBox}, AttributeValue, AnyValue}};

pub trait BevyAttribute {
    type Component: Component;
    fn component(&self) -> Self::Component;
}

#[derive(PartialEq)]
pub struct AttributeStyle(pub Style);

impl BevyAttribute for AttributeStyle {
    type Component = Style;
    fn component(&self) -> Style {
        self.0.clone()
    }
}

impl<'a> IntoAttributeValue<'a> for AttributeStyle {
    fn into_value(self, bump: &'a Bump) -> AttributeValue<'a> {
        let boxed: BumpBox<'a, dyn AnyValue> = unsafe { BumpBox::from_raw(bump.alloc(self)) };
        AttributeValue::Any(RefCell::new(Some(boxed)))
    }
}