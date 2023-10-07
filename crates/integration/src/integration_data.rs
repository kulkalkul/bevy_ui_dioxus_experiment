use bevy::{prelude::{World, BuildWorldChildren, Entity, Parent, Children, DespawnRecursiveExt, Component}, text::{Text, TextStyle, TextSection}, ui::Style};
use dioxus::core::{Mutations, Mutation, ElementId, BorrowedAttributeValue};

use crate::{template_map::TemplateMap, element_map::ElementMap, node::{Element, NodeChild, ChildNode, RootNode}, nodes::TextNode, attributes::{AttributeStyle, BevyAttribute}};

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
            let parent = parent_entity(world, entity);
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
            "style" => apply_attribute_to_component::<AttributeStyle, _>(world, entity, value),
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

fn apply_attribute_to_component<A, T>(
    world: &mut World,
    entity: Entity,
    value: BorrowedAttributeValue,
) where
    T: Component + Default,
    A: BevyAttribute<Component = T> + 'static,
{
    use BorrowedAttributeValue as Val;
    let component = world.get_mut::<T>(entity);

    match (value, component) {
        (Val::Any(value), Some(mut component)) => {
            let value = value.as_any().downcast_ref::<A>().unwrap();
            *component = value.component();
        },
        (Val::Any(value), None) => {
            let value = value.as_any().downcast_ref::<A>().unwrap();
            world
                .entity_mut(entity)
                .insert(value.component());
        }
        (Val::None, Some(mut component)) => {
            *component = T::default();
        },
        (Val::None, None) => {
            world.entity_mut(entity).insert(T::default());
        }
        _ => panic!("invalid attribute type"),
    }
}