mod instance_ext;
mod node_ext;
mod object_ext;
mod option_ext;
mod packed_scene_ext;
mod ref_ext;
mod resource_loader_ext;
mod result_ext;
mod spatial_ext;
mod tref_ext;
mod vector2_ext;
mod vector3_ext;

pub use crate::unsafe_functions::instance_ext::*;
pub use crate::unsafe_functions::node_ext::*;
pub use crate::unsafe_functions::object_ext::*;
pub use crate::unsafe_functions::option_ext::*;
pub use crate::unsafe_functions::packed_scene_ext::*;
pub use crate::unsafe_functions::ref_ext::*;
pub use crate::unsafe_functions::resource_loader_ext::*;
pub use crate::unsafe_functions::result_ext::*;
pub use crate::unsafe_functions::spatial_ext::*;
pub use crate::unsafe_functions::vector2_ext::*;
pub use crate::unsafe_functions::vector3_ext::*;

/// Same functionality as `panic!()`, but also outputs to the godot output.
#[macro_export]
macro_rules! godot_panic {
    ($($args:tt)*) => {
        {
            gdnative::prelude::godot_error!($($args)*);
            panic!($($args)*);
        }
    }
}

/// Same functionality as `assert!()`, but also outputs to the godot output.
#[macro_export]
macro_rules! godot_assert {
    ($condition:expr $(,)?) => {
        if !$condition {
            gdnative::godot_error!("Assertion error: {}", stringify!($condition));
            panic!("Assertion error: {}", stringify!($condition));
        }
    };
    ($condition:expr, $($args:tt)*) => {
        if !$condition {
            gdnative::godot_error!($($args)*);
            panic!($($args)*);
        }
    };
}

/// Same functionality as `debug_assert!()`, but also outputs to the godot output.
#[macro_export]
macro_rules! godot_debug_assert {
    ($condition:expr $(,)?) => {

        if cfg!(debug_assertions) && !$condition {
            gdnative::godot_error!("Assertion error: {}", stringify!($condition));
            panic!("Assertion error: {}", stringify!($condition));
        }
    };
    ($condition:expr, $($args:tt)*) => {
        if cfg!(debug_assertions) && !$condition {
            gdnative::godot_error!($($args)*);
            panic!($($args)*);
        }
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn godot_assert_true() {
        godot_assert!(true)
    }

    #[test]
    fn godot_assert_message_true() {
        godot_assert!(true, "this should not {}", "happen")
    }

    #[test]
    fn godot_debug_assert_true() {
        godot_debug_assert!(true)
    }

    #[test]
    fn godot_debug_assert_message_true() {
        godot_debug_assert!(true, "this should not {}", "happen")
    }
}
