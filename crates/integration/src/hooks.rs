use std::{rc::Rc, cell::{RefCell, RefMut}};

use bevy::{ecs::query::{WorldQuery, ROQueryItem}, prelude::{QueryState, World}, ui::Style};
use dioxus::prelude::{ScopeState, use_state};

use crate::{integration::BevyWorld, attributes::AttributeStyle};


pub fn use_world(cx: &ScopeState) -> &mut BevyWorld {
    cx.use_hook(|| {
        cx.consume_context::<BevyWorld>().unwrap().clone()
    })
}

pub fn use_query<'a, Q: WorldQuery>(
    world: &mut BevyWorld,
) -> QueryState<Q> {
    world.borrow_mut().query::<Q>()
}