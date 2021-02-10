//! This module contains all resources to call and handle events.

use std::{any::Any, collections::BTreeMap, rc::Rc};

use dces::entity::Entity;

use crate::widget_base::StatesContext;

pub use self::drop::*;
pub use self::editable::*;
pub use self::event_adapter::*;
pub use self::event_handler::*;
pub use self::event_queue::*;
pub use self::focus::*;
pub use self::key::*;
pub use self::mouse::*;
pub use self::system::*;
pub use self::text_input::*;
pub use self::window::*;

mod drop;
mod editable;
mod event_adapter;
mod event_handler;
mod event_queue;
mod focus;
mod key;
mod key_event;
mod mouse;
mod mouse_event;
mod system;
mod text_input;
mod window;

pub use self::key_event::*;
pub use self::mouse_event::*;

/// Defines the strategy of an event how it moves through the tree.
#[derive(Debug, Clone, PartialEq)]
pub enum EventStrategy {
    // /// From root to leaf.
    // TopDown,
    /// From leaf to root.
    BottomUp,

    /// Occurs direct.
    Direct,
}

/// Used to define an event.
pub trait Event: Any {
    fn strategy(&self) -> EventStrategy {
        EventStrategy::BottomUp
    }
}

pub type EventHandlerMap = BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>;

pub type TriggerHandler = dyn Fn(&mut StatesContext, Entity) + 'static;
