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
