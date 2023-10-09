use bevy::{ecs::query::WorldQuery, prelude::QueryState};
use dioxus::prelude::ScopeState;

use crate::integration::BevyWorld;

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