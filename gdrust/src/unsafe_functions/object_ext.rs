use crate::godot_panic;
use gdnative::object::ownership::Ownership;
use gdnative::prelude::{NativeClass, Object, SubClass, TInstance, TRef};

/// An error when using `try_as_instance`.
pub enum TryAsError {
    /// We were unable to get as an instance because the node was not of the correct type.
    Cast,
    /// We were unable to get as an instance because the expected script was not attached to the
    /// Node.
    Instance,
}

pub trait ObjectExt<'a, A: Ownership, Class: SubClass<Object>> {
    /// Tries to cast a given node as `T`. Returns `Ok` with the `TInstance` if found. Returns `Err`
    /// if it was unable to get the `TInstance`:
    /// # Errors
    /// `TryAsError::Cast`: If the given node is not the correct type for the script.
    /// `TryAsError::Instance`: If the given node does not have the correct script attached.
    fn try_as_instance<T: NativeClass>(self) -> Result<TInstance<'a, T, A>, TryAsError>
    where
        <T as NativeClass>::Base: SubClass<Class>;

    /// Expects the passed in node has the `T` script attached. Panics if not. Same as `try_as_instance`
    /// but panics on `Err`
    /// # Panics
    /// If either the given node is not the correct type for the script, or the given node does not have the correct script attached
    fn expect_as_instance<T: NativeClass>(self) -> TInstance<'a, T, A>
    where
        <T as NativeClass>::Base: SubClass<Class>;
}

impl<'a, A: Ownership, Class: SubClass<Object>> ObjectExt<'a, A, Class> for TRef<'a, Class, A> {
    fn try_as_instance<T: NativeClass>(self) -> Result<TInstance<'a, T, A>, TryAsError>
    where
        <T as NativeClass>::Base: SubClass<Class>,
    {
        self.cast::<<T as NativeClass>::Base>()
            .ok_or(TryAsError::Cast)
            .and_then(|x| x.cast_instance().ok_or(TryAsError::Instance))
    }

    fn expect_as_instance<T: NativeClass>(self) -> TInstance<'a, T, A>
    where
        <T as NativeClass>::Base: SubClass<Class>,
    {
        use gdnative::object::GodotObject;

        match self.try_as_instance() {
            Ok(x) => x,
            Err(TryAsError::Cast) => godot_panic!(
                "Expected to cast to {}, but that was not found",
                <T as NativeClass>::Base::class_name()
            ),
            Err(TryAsError::Instance) => godot_panic!(
                "Expected Node to have {} attached, but it did not",
                <T as NativeClass>::Base::class_name()
            ),
        }
    }
}
