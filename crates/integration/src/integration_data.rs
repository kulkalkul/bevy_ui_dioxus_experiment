use bevy::{prelude::{World, BuildWorldChildren, Entity, Parent, Children, DespawnRecursiveExt}, text::{Text, TextStyle, TextSection}, ui::Style};
use dioxus::core::{Mutations, Mutation, ElementId, BorrowedAttributeValue};

use crate::{template_map::TemplateMap, element_map::ElementMap, ui_node::{Element, NodeChild, ChildNode, RootNode}, bevy_node::TextNode, attributes::Attr};

#[derive(Default, Debug)]
pub struct IntegrationData {
    template_map: TemplateMap,
    element_map: ElementMap,
    stack: Vec<Entity>,
}

impl IntegrationData {
    pub fn set_root(&mut self, root: Entity) {
        self.element_map.set(ElementId(0), root);
        self.stack.push(root);
    }
    pub fn update_dom(&mut self, world: &mut World, mutations: Mutations) {
        for template in mutations.templates {
            self.template_map.add(template);
        }
    
        for edit in mutations.edits {
            match edit {
                Mutation::AppendChildren { id, m }
                    => self.append_children(world, id, m),
                Mutation::AssignId { path, id }
                    => self.assign_id(world, path, id),
                Mutation::CreatePlaceholder { id }
                    => self.create_placeholder(world, id),
                Mutation::CreateTextNode { value, id }
                    => self.create_text_node(world, value, id),
                Mutation::HydrateText { path, value, id }
                    => self.hydrate_text(world, path, value, id),
                Mutation::LoadTemplate { name, index, id }
                    => self.load_template(world, name, index, id),
                Mutation::ReplaceWith { id, m }
                    => self.replace_with(world, id, m),
                Mutation::ReplacePlaceholder { path, m }
                    => self.replace_placeholder(world, path, m),
                Mutation::InsertAfter { id, m }
                    => self.insert_after(world, id, m),
                Mutation::InsertBefore { id, m }
                    => self.insert_before(world, id, m),
                Mutation::SetAttribute { name, value, id, .. }
                    => self.set_attribute(world, name, value, id),
                Mutation::SetText { value, id } =>
                    self.set_text(world, value, id),
                Mutation::NewEventListener { name, id } => todo!(),
                Mutation::RemoveEventListener { name, id } => todo!(),
                Mutation::Remove { id }
                    => self.remove(world, id),
                Mutation::PushRoot { id }
                    => self.push_root(world, id),
            _ => (),
            }
        }
    }
    fn append_children(&mut self, world: &mut World, id: ElementId, m: usize) {
        let children = self.stack.split_off(self.stack.len() - m);
        let parent = self.element_map.get(id);

        let mut parent = world.entity_mut(parent);

        for child in children {
            parent.add_child(child);
        }
    }
    fn assign_id(&mut self, world: &mut World, path: &[u8], id: ElementId) {
        let child = child_at_path(&self.stack, world, path);
        self.element_map.set(id, child);
    }
    fn create_placeholder(&mut self, world: &mut World, id: ElementId) {
        let entity = world.spawn_empty().id();
        self.element_map.set(id, entity);
        self.stack.push(entity);
    }
    fn create_text_node(&mut self, world: &mut World, value: &str, id: ElementId) {
        let node = TextNode {
            text: Text::from_section(value, TextStyle::default()),
        };
        let entity = world.spawn(node.bundle()).id();
        
        self.element_map.set(id, entity);
        self.stack.push(entity);
    }
    fn hydrate_text(&mut self, world: &mut World, path: &[u8], value: &str, id: ElementId) {
        let entity = child_at_path(&self.stack, world, path);
        self.element_map.set(id, entity);
        
        if let Some(mut text) = world.get_mut::<Text>(entity) {
            // Same as set_text
            text.sections[0].value = value.to_owned();
        } else {
            // Do we need to preserve node styles?
            world.entity_mut(entity).despawn_recursive();

            let node = TextNode {
                text: Text::from_section(value, TextStyle::default()),
            };
            world.spawn(node.bundle());
        }
    }
    fn load_template(&mut self, world: &mut World, name: &str, index: usize, id: ElementId) {
        let entity = match &self.template_map.map[name][index] {
            RootNode::ElementWithChildren {
                element,
                children,
            } => {
                let parent = match element {
                    Element::Div { node } => world.spawn(node.bundle()),
                    Element::Image { node } => world.spawn(node.bundle()),
                    Element::Button { node } => world.spawn(node.bundle()),
                }.id();

                let mut current = parent;
                let mut created = Entity::PLACEHOLDER;
                let mut stack = Vec::with_capacity(children.nodes.len());

                for node_child in &children.nodes {
                    match node_child {
                        NodeChild::Node(node) => {
                            world.entity_mut(current).with_children(|builder| {
                                created = match node {
                                    ChildNode::Element { element } => match element {
                                        Element::Div { node } => builder.spawn(node.bundle()),
                                        Element::Image { node } => builder.spawn(node.bundle()),
                                        Element::Button { node } => builder.spawn(node.bundle()),
                                    },
                                    ChildNode::Text { node } => builder.spawn(node.bundle()),
                                    ChildNode::PlaceHolder => builder.spawn_empty(),
                                }.id();
                            });
                        },
                        NodeChild::In => {
                            stack.push(current);
                            current = created;
                        },
                        NodeChild::Out => {
                            current = stack.pop().expect("shouldn't have empty vec at this stage");
                        },
                    }
                }

                parent
            },
            RootNode::Element { element } => {
                match element {
                    Element::Div { node } => world.spawn(node.bundle()),
                    Element::Image { node } => world.spawn(node.bundle()),
                    Element::Button { node } => world.spawn(node.bundle()),
                }.id()
            },
            RootNode::Text { node } => {
                world.spawn(node.bundle()).id()
            },
            RootNode::PlaceHolder => world.spawn_empty().id(),
        };

        self.element_map.set(id, entity);
        self.stack.push(entity);
    }
    fn replace_with(&mut self, world: &mut World, id: ElementId, m: usize) {
        let to_replace = self.stack.split_off(self.stack.len() - m);
        let old = self.element_map.get(id);

        // Not the most performant impl, but Children's
        // field is private
        let parent = parent_entity(world, old);
        add_children_relative(world, parent, old, to_replace, ChildRelation::Before);
        despawn_child(world, old);
    }
    fn replace_placeholder(&mut self, world: &mut World, path: &[u8], m: usize) {
        let to_replace = self.stack.split_off(self.stack.len() - m);
        let child = child_at_path(&self.stack, world, path);
        let parent = parent_entity(world, child);
        
        // Same as replace_with
        add_children_relative(world, parent, child, to_replace, ChildRelation::Before);
        despawn_child(world, child);
    }
    fn insert_after(&mut self, world: &mut World, id: ElementId, m: usize) {
        let to_insert = self.stack.split_off(self.stack.len() - m);
        let old = self.element_map.get(id);

        let parent = parent_entity(world, old);
        add_children_relative(world, parent, old, to_insert, ChildRelation::After);
    }
    fn insert_before(&mut self, world: &mut World, id: ElementId, m: usize) {
        let to_insert = self.stack.split_off(self.stack.len() - m);
        let old = self.element_map.get(id);

        let parent = parent_entity(world, old);
        add_children_relative(world, parent, old, to_insert, ChildRelation::Before);
    }
    fn set_attribute(
        &mut self,
        world: &mut World,
        name: &str,
        value: BorrowedAttributeValue,
        id: ElementId,
    ) {
        let entity = self.element_map.get(id);

        match name {
            "display" => update_style(world, entity, value, |s| &mut s.display),
            "position_type" => update_style(world, entity, value, |s| &mut s.position_type),
            "overflow" => update_style(world, entity, value, |s| &mut s.overflow),
            "direction" => update_style(world, entity, value, |s| &mut s.direction),
            "left" => update_style(world, entity, value, |s| &mut s.left),
            "right" => update_style(world, entity, value, |s| &mut s.right),
            "top" => update_style(world, entity, value, |s| &mut s.top),
            "bottom" => update_style(world, entity, value, |s| &mut s.bottom),
            "width" => update_style(world, entity, value, |s| &mut s.width),
            "height" => update_style(world, entity, value, |s| &mut s.height),
            "min_width" => update_style(world, entity, value, |s| &mut s.min_width),
            "min_height" => update_style(world, entity, value, |s| &mut s.min_height),
            "max_width" => update_style(world, entity, value, |s| &mut s.max_width),
            "max_height" => update_style(world, entity, value, |s| &mut s.max_height),
            "aspect_ratio" => update_style(world, entity, value, |s| &mut s.aspect_ratio),
            "align_items" => update_style(world, entity, value, |s| &mut s.align_items),
            "justify_items" => update_style(world, entity, value, |s| &mut s.justify_items),
            "align_self" => update_style(world, entity, value, |s| &mut s.align_self),
            "justify_self" => update_style(world, entity, value, |s| &mut s.justify_self),
            "align_content" => update_style(world, entity, value, |s| &mut s.align_content),
            "justify_content" => update_style(world, entity, value, |s| &mut s.justify_content),
            "margin" => update_style(world, entity, value, |s| &mut s.margin),
            "padding" => update_style(world, entity, value, |s| &mut s.padding),
            "border" => update_style(world, entity, value, |s| &mut s.border),
            "flex_direction" => update_style(world, entity, value, |s| &mut s.flex_direction),
            "flex_wrap" => update_style(world, entity, value, |s| &mut s.flex_wrap),
            "flex_grow" => update_style(world, entity, value, |s| &mut s.flex_grow),
            "flex_shrink" => update_style(world, entity, value, |s| &mut s.flex_shrink),
            "flex_basis" => update_style(world, entity, value, |s| &mut s.flex_basis),
            "row_gap" => update_style(world, entity, value, |s| &mut s.row_gap),
            "column_gap" => update_style(world, entity, value, |s| &mut s.column_gap),
            "grid_auto_flow" => update_style(world, entity, value, |s| &mut s.grid_auto_flow),
            "grid_template_rows" => update_style(world, entity, value, |s| &mut s.grid_template_rows),
            "grid_template_columns" => update_style(world, entity, value, |s| &mut s.grid_template_columns),
            "grid_auto_rows" => update_style(world, entity, value, |s| &mut s.grid_auto_rows),
            "grid_auto_columns" => update_style(world, entity, value, |s| &mut s.grid_auto_columns),
            "grid_row" => update_style(world, entity, value, |s| &mut s.grid_row),
            "grid_column" => update_style(world, entity, value, |s| &mut s.grid_column),
            _ => panic!("invalid attribute name"),
        };
    }
    fn set_text(&mut self, world: &mut World, value: &str, id: ElementId) {
        let entity = self.element_map.get(id);
        if let Some(mut text) = world.get_mut::<Text>(entity) {
            // Multi section text with DOM wouldn't be compatible
            // So assuming every text consists of 1 section is OK
            // I think
            let style = text.sections[0].style.clone();
            text.sections = vec![TextSection::new(value, style)];
        }
    }
    fn remove(&mut self, world: &mut World, id: ElementId) {
        let entity = self.element_map.get(id);
        world.entity_mut(entity).despawn_recursive();
    }
    fn push_root(&mut self, world: &mut World, id: ElementId) {
        let entity = self.element_map.get(id);
        self.stack.push(entity);
    }
}

fn child_at_path(stack: &Vec<Entity>, world: &mut World, path: &[u8]) -> Entity {
    let mut current = stack
        .last()
        .expect("stack shouldn't be empty")
        .to_owned();
    
    // Maybe map parent <-> children relationship outside of ecs too?
    for &index in path {
        current = children_at(world, current, index as usize);
    }

    current
}

fn despawn_child(world: &mut World, child: Entity) {
    // Maybe use HierarchyEvent::ChildRemoved directly?
    let mut child = world.entity_mut(child);
    child.remove_parent();
    child.despawn();
}

fn parent_entity(world: &mut World, child: Entity) -> Entity {
    // I think we can safely assume this would never panic without a bug
    world
        .get::<Parent>(child)
        .expect("parent should exist")
        .get()
}

fn children_at(world: &mut World, parent: Entity, index: usize) -> Entity {
    world
        .get::<Children>(parent)
        .expect("children should exist")
        .get(index)
        .expect("child should exist")
        .to_owned()

}

enum ChildRelation {
    Before,
    After,
}

fn add_children_relative(
    world: &mut World,
    parent: Entity,
    child: Entity,
    children: Vec<Entity>,
    child_relation: ChildRelation,
) {
    // I think we can safely assume this would never panic without a bug
    let children_components = world
        .get::<Children>(parent)
        .expect("children should exist");

    let index = children_components.iter()
        .position(|entity| *entity == child)
        .expect("child should exist");

    let index = match child_relation {
        ChildRelation::Before => index,
        ChildRelation::After => index + 1,
    };

    world.entity_mut(parent).insert_children(index, &children);
}

fn update_style<T: Default + Clone + 'static>(
    world: &mut World,
    entity: Entity,
    value: BorrowedAttributeValue,
    selector: fn(&mut Style) -> &mut T,
) {
    use BorrowedAttributeValue as Val;
    let style = world.get_mut::<Style>(entity);

    let mut style = match style {
        Some(style) => style,
        None => {
            world.entity_mut(entity).insert(Style::default());
            world.get_mut(entity).unwrap()
        },
    };

    let field = selector(&mut style);

    match value {
        Val::Any(value) => {
            let value = value.as_any().downcast_ref::<Attr<T>>().unwrap();
            *field = value.0.clone();
        },
        Val::None => {
            *field = T::default();
        },
        _ => panic!("invalid attribute type"),
    }
}