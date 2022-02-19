use gdnative::object::bounds::{AssumeSafeLifetime, LifetimeConstraint};
use gdnative::prelude::{GodotObject, Instance, NativeClass, Ref, Shared, TInstance, TRef, Unique};

pub trait InstanceExt<T: NativeClass> {
    fn expect_safe<'a, 'r>(&'r self) -> TInstance<'a, T, Shared>
    where
        AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<<T::Base as GodotObject>::Memory>;

    fn assume_unique(self) -> Instance<T, Unique>;
}

impl<T: NativeClass> InstanceExt<T> for Instance<T, Shared> {
    fn expect_safe<'a, 'r>(&'r self) -> TInstance<'a, T, Shared>
    where
        AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<<T::Base as GodotObject>::Memory>,
    {
        unsafe { self.assume_safe() }
    }

    fn assume_unique(self) -> Instance<T, Unique> {
        unsafe { self.assume_unique() }
    }
}
