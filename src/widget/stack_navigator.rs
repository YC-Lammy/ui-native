use alloc::sync::Arc;
use parking_lot::RwLock;

use crate::private::{ElementLike, NativeElement};
use crate::shadow_tree::component::{
    CoreComponent, NavigatorCommand, StackNavigaterNode, StackNavigaterScreenOptions,
};
use crate::shadow_tree::NodeID;

#[derive(Debug, Default, Clone)]
pub struct StackNavigator {
    inner: Arc<StackNavigatorInner>,
}

#[derive(Debug, Default)]
struct StackNavigatorInner {
    /// node id of the corresponding navigator widget
    id: NodeID,
    commands: RwLock<Vec<NavigatorCommand>>,
}

impl StackNavigator {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(StackNavigatorInner {
                id: NodeID::new_unique(),
                commands: RwLock::new(Vec::new()),
            }),
        }
    }
    /// should be called during `render` to construct a widget
    pub fn navigator(&self) -> StackNavigatorElement {
        let nav_node = Box::new(StackNavigaterNode {
            id: self.inner.id,
            ..Default::default()
        });

        return StackNavigatorElement {
            navigator_inner: self.inner.clone(),
            tree_node: nav_node,
            children: Vec::new(),
        };
    }

    /// go to the page in index, noop if index is not valid
    pub fn goto_index(&self, index: usize) {
        self.inner
            .commands
            .write()
            .push(NavigatorCommand::Goto(index))
    }

    /// goback to the last page, noop if cannot go back
    pub fn goback(&self) {
        self.inner.commands.write().push(NavigatorCommand::Goback)
    }
}

pub struct StackNavigatorElement {
    navigator_inner: Arc<StackNavigatorInner>,
    ///
    tree_node: Box<StackNavigaterNode>,
    children: Vec<(Box<dyn ElementLike>, StackNavigaterScreenOptions)>,
}

impl StackNavigatorElement {
    pub fn with_screen<T>(mut self, component: T, options: StackNavigaterScreenOptions) -> Self
    where
        T: ElementLike,
    {
        self.children.push((Box::new(component), options));
        return self;
    }
}

impl NativeElement for StackNavigatorElement {
    fn on_state_change(&mut self, ctx: &crate::Context) {
        for (child, _) in &mut self.children {
            child.on_state_change(ctx)
        }
    }
    fn core_component(&self) -> CoreComponent {
        CoreComponent::StackNavigator(self.tree_node.clone())
    }
    fn render(&mut self) {
        // render all the children
        for (child, options) in &mut self.children {
            // render the child
            let mut elem = child.render();

            // keep rendering until a core component is reached
            let component = loop {
                match elem {
                    Err(mut e) => {
                        elem = e.render();
                    }
                    Ok(c) => break c,
                }
            };

            // push child
            self.tree_node.children.push(component);
            // push screen
            self.tree_node.screen_options.push(*options);
        }

        // acquire write lock
        let mut commands = self.navigator_inner.commands.write();
        // take and clear commands
        let commands = core::mem::replace(commands.as_mut(), Vec::new());

        self.tree_node.commands = commands;
    }
}

impl ElementLike for StackNavigatorElement {
    fn as_native(&mut self) -> Option<&mut dyn NativeElement> {
        Some(self)
    }
    fn as_element(&mut self) -> Option<&mut dyn crate::Element> {
        None
    }
}
