//! Minimal *in-house* ECS kernel
//!
//! # Gals
//! - Zero external dependencies
//! - Friendly, readable code for beginners
//! - Works in both `wasm32` and native targes
//!

use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Public alias for entity indentifiers
///
/// Wi keep it simple: a Monotonically increasing `u32` plus a recycled list
pub type Entity = u32;

// Entity allocation / recycling

#[derive(Default)]
pub struct EntityAllocator {
    next: Entity,
    recycled: Vec<Entity>,
}

impl EntityAllocator {
    /// Allocate a brand-new entity ID ( or reuse a recycled one)
    pub fn create(&mut self) -> Entity {
        self.recycled.pop().unwrap_or_else(|| {
            let id = self.next;
            self.next += 1;
            id
        })
    }

    /// Mark an entity ID as free for future reuse.
    pub fn destroy(&mut self, entity: Entity) {
        self.recycled.push(entity);
    }
}

// Component storage ( sparse vector)

/// Generic sparse storage for a single component type
///
/// Internally it is just `Vec<Option<T>>` indexed by `Entity`
///
/// *Pros*:
/// - Very small code
/// - Cache-friendly iteration
/// *Cons*:
/// - One vector per component type
/// - Dense IDs preferred
pub struct Storage<T> {
    data: Vec<Option<T>>,
}

impl<T> Storage<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Insert or replace a component for the given entity
    pub fn insert(&mut self, entity: Entity, component: T) {
        if entity as usize >= self.data.len() {
            self.data.resize_with(entity as usize + 1, || None);
        }
        self.data[entity as usize] = Some(component);
    }

    /// Immutable access
    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.data.get(entity as usize)?.as_ref()
    }
}
