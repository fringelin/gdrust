use std::ops::{Deref, DerefMut};

use bevy::prelude::*;
use gdnative::prelude::*;

#[derive(Component)]
pub struct GodotObjRef<T: GodotObject> {
    value: Ref<T>,
}

impl<T: GodotObject> GodotObjRef<T> {
    pub fn new(value: Ref<T>) -> Self {
        Self { value }
    }
}

impl<T: GodotObject> Deref for GodotObjRef<T> {
    type Target = Ref<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: GodotObject> DerefMut for GodotObjRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[derive(Component)]
pub struct GodotObjInstance<T: NativeClass> {
    pub instance: Instance<T>,
}

impl<T: NativeClass> GodotObjInstance<T> {
    #[warn(dead_code)]
    pub fn new(instance: Instance<T>) -> Self {
        Self { instance }
    }
}

impl<T: NativeClass> Deref for GodotObjInstance<T> {
    type Target = Instance<T>;

    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}

impl<T: NativeClass> DerefMut for GodotObjInstance<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.instance
    }
}

#[derive(Component)]
pub struct GameNode;

#[derive(Component)]
pub struct PlayingGame;
