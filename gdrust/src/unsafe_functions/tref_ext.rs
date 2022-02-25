use gdnative::object::GodotObject;
use gdnative::prelude::{Ref, Shared, TRef};

pub trait TRefExt<T: GodotObject> {
    fn expect_shared(&self) -> Ref<T, Shared>;
}

impl<'a, T: GodotObject> TRefExt<T> for TRef<'a, T, Shared> {
    fn expect_shared(&self) -> Ref<T, Shared>
    where
        Self: Sized,
    {
        unsafe { self.assume_shared() }
    }
}
