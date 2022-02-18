use gdnative::object::bounds::{AssumeSafeLifetime, LifetimeConstraint};
use gdnative::object::GodotObject;
use gdnative::prelude::{Ref, Shared, TRef, Unique};

pub trait RefExt<T: GodotObject> {
    fn expect_safe<'a, 'r>(&'r self) -> TRef<'a, T, Shared>
    where
        AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<T::Memory>;

    fn assume_unique(self) -> Ref<T, Unique>;
}

impl<T: GodotObject> RefExt<T> for Ref<T, Shared> {
    fn expect_safe<'a, 'r>(&'r self) -> TRef<'a, T, Shared>
    where
        AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<T::Memory>,
    {
        unsafe { self.assume_safe() }
    }

    fn assume_unique(self) -> Ref<T, Unique> {
        unsafe { self.assume_unique() }
    }
}
