use crate::unsafe_functions::option_ext::OptionExt;
use gdnative::api::SceneTree;
use gdnative::prelude::{NativeClass, Node, NodePath, Shared, SubClass, TInstance, TRef};

pub trait NodeExt<P: Into<NodePath>> {
    fn expect_instance<'a, T>(&self, path: P) -> TInstance<'a, T>
    where
        T: NativeClass,
        T::Base: SubClass<Node>;

    /// Gets a typed node from a node path. This has an explicit `unsafe` block, and can panic. The
    /// unsafe code is calling `assume_safe` on the node at `path`.
    /// # Panics
    /// - If no node is found at the path.
    /// - If a node is found at the path, but is not the correct type.
    ///
    /// # GdScript equivalent
    /// ```gdscript
    /// get_node(path)
    /// ```
    fn expect_node<'a, T: SubClass<Node>>(&self, path: P) -> TRef<'a, T>;
}

pub trait NodeTreeExt {
    /// Gets the parent node with a type. This has an explicit `unsafe` block, and can panic. The
    /// unsafe code is calling `assume_safe` on the parent node.
    /// # Panics
    /// - If no parent is found (root node).
    /// - If a node is found at the path, but is not the correct type.
    ///
    /// # GdScript equivalent
    /// ```gdscript
    /// get_parent()
    /// ```
    fn expect_parent<'a, T: SubClass<Node>>(&self) -> TRef<'a, T>;

    /// Gets the scene tree. This has an explicit `unsafe` block, and can panic. The unsafe code is
    /// calling `assume_safe` on the scene tree.
    /// # Panics
    /// - If the scene tree is not found.
    ///
    /// # GdScript Equivalent
    /// ```gdscript
    /// get_tree()
    /// ```
    fn expect_tree<'a>(&self) -> TRef<'a, SceneTree>;
}

impl<T: SubClass<Node>, P: Into<NodePath>> NodeExt<P> for T {
    fn expect_instance<'a, N>(&self, path: P) -> TInstance<'a, N>
    where
        N: NativeClass,
        N::Base: SubClass<Node>,
    {
        let path = path.into();
        let path_name = path.to_string();
        unsafe {
            self.upcast()
                .get_node(path)
                .godot_expect(format!("Could not find a node at {}", path_name).as_str())
                .assume_safe()
                .cast::<N::Base>()
                .godot_expect("Could not cast")
                .cast_instance()
                .expect("Could not cast instance")
        }
    }

    fn expect_node<'a, Child>(&self, path: P) -> TRef<'a, Child>
    where
        Child: SubClass<Node>,
    {
        let path = path.into();
        let path_name = path.to_string();
        unsafe {
            self.upcast()
                .get_node(path)
                .godot_expect(format!("Could not find a node at {}", path_name).as_str())
                .assume_safe()
                .cast::<Child>()
                .godot_expect("Could not cast")
        }
    }
}

impl<T: SubClass<Node>> NodeTreeExt for T {
    fn expect_parent<'a, Child: SubClass<Node>>(&self) -> TRef<'a, Child> {
        unsafe {
            self.upcast()
                .get_parent()
                .godot_expect("Could not get a parent node")
                .assume_safe()
                .cast::<Child>()
                .godot_expect("Could not cast")
        }
    }

    fn expect_tree<'a>(&self) -> TRef<'a, SceneTree, Shared> {
        unsafe {
            self.upcast()
                .get_tree()
                .godot_expect("Expected scene tree, but couldn't find it")
                .assume_safe()
        }
    }
}
