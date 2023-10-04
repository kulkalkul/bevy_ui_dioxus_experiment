use bevy::{prelude::{NodeBundle, default, TextBundle, ImageBundle, ButtonBundle}, text::Text};

#[derive(Debug, Clone, Default)]
pub struct SimpleNode {

}

impl SimpleNode {
    pub fn bundle(&self) -> NodeBundle {
        NodeBundle {
            ..default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TextNode {
    pub text: Text,
}

impl TextNode {
    pub fn bundle(&self) -> TextBundle {
        TextBundle {
            text: self.text.clone(),
            ..default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ImageNode {

}

impl ImageNode {
    pub fn bundle(&self) -> ImageBundle {
        ImageBundle {
            ..default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ButtonNode {

}

impl ButtonNode {
    pub fn bundle(&self) -> ButtonBundle {
        ButtonBundle {
            ..default()
        }
    }
}