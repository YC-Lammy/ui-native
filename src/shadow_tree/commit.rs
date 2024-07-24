use std::sync::Arc;

use super::command::Command;
use super::component::CoreComponent;
use super::NodeID;

/// commits the tree, perform necessary checks and transformation
/// encodes the tree representation in commands to be executed by the native tree
pub fn commit_tree(tree: &mut CoreComponent, old: Option<&CoreComponent>) -> Vec<Command> {
    let mut commands = Vec::new();

    tree_generate_command(tree, old, &mut commands);

    commands.push(Command::MountRoot {
        node: tree.id().expect("expecting id"),
    });

    return commands;
}

/// generate commands to commit to native tree
fn tree_generate_command(
    mut current: &mut CoreComponent,
    old: Option<&CoreComponent>,
    cmd: &mut Vec<Command>,
) -> NodeID {
    match (&mut current, old) {
        // both new and old is view
        (CoreComponent::View(v), Some(CoreComponent::View(ov))) => {
            // set the id
            v.id = ov.id;

            if !Arc::ptr_eq(&v.style, &ov.style){
                cmd.push(
                    Command::SetStyle { 
                        node: v.id.unwrap(), 
                        style: v.style.clone()
                    }
                )
            }

            let mut v_iter = v.children.iter_mut();
            let mut ov_iter = ov.children.iter();

            let mut i = 0;

            loop {
                let child = v_iter.next();
                let old_child = ov_iter.next();

                match (child, old_child) {
                    (Some(child), Some(old_child)) => {
                        tree_generate_command(child, Some(old_child), cmd);

                        if child.id() != old_child.id() {
                            // replace child
                            cmd.push(Command::ViewSetChild {
                                id: v.id.unwrap(),
                                child: child.id().unwrap(),
                                index: i,
                            })
                        }
                    }
                    // remove old component
                    (None, Some(old)) => {
                        cmd.push(Command::ViewRemoveChild {
                            id: v.id.unwrap(),
                            child: old.id().unwrap(),
                            index: i,
                        });
                        cmd.push(Command::RemoveNode {
                            node: old.id().unwrap(),
                        });
                    }
                    // add new child component
                    (Some(child), None) => {
                        // create child
                        let child_id = tree_generate_command(child, None, cmd);
                        // add child
                        cmd.push(Command::ViewSetChild {
                            id: v.id.unwrap(),
                            child: child_id,
                            index: i,
                        })
                    }
                    // finish iteration
                    (None, None) => break,
                };

                i += 1;
            }
        }
        // old and new not the same
        (CoreComponent::View(view), old_component) => {
            // remove the old node
            if let Some(old) = old_component {
                cmd.push(Command::RemoveNode {
                    node: old.id().unwrap(),
                });
            }

            // create new id
            view.id = Some(NodeID::new_unique());

            // create the view
            cmd.push(Command::ViewCreate {
                id: view.id.unwrap(),
                style: view.style.clone(),
            });

            // get the parent id
            let parent_id = view.id.unwrap();

            // loop through child in element
            for (i, child) in view.children.iter_mut().enumerate() {
                // generate command for child element
                let child_id = tree_generate_command(child, None, cmd);

                // add child to element
                cmd.push(Command::ViewSetChild {
                    id: parent_id,
                    child: child_id,
                    index: i,
                })
            }
        }
        (CoreComponent::ImageView(v), Some(CoreComponent::ImageView(ov))) => {
            v.id = ov.id;

            if !Arc::ptr_eq(&v.style, &ov.style){
                cmd.push(
                    Command::SetStyle { 
                        node: v.id.unwrap(), 
                        style: v.style.clone()
                    }
                )
            }
        }
        (CoreComponent::ImageView(v), old_component) => {
            // remove the old node
            if let Some(old) = old_component {
                cmd.push(Command::RemoveNode {
                    node: old.id().unwrap(),
                });
            }
            v.id = Some(NodeID::new_unique());

            // create image view
            cmd.push(Command::ImageViewCreate { 
                id: v.id.unwrap(),
                style: v.style.clone(),
            });
        }
        (CoreComponent::ScrollView(v), Some(CoreComponent::ScrollView(ov))) => {
            // set the id
            v.id = ov.id;

            if !Arc::ptr_eq(&v.style, &ov.style){
                cmd.push(
                    Command::SetStyle { 
                        node: v.id.unwrap(), 
                        style: v.style.clone()
                    }
                )
            }

            match &mut v.child{
                Some(child) => {
                    // generate command for child
                    tree_generate_command(child, ov.child.as_ref(), cmd);

                    // check if child is the same
                    if child.id() != ov.child.as_ref().and_then(|c|c.id()){
                        cmd.push(
                            Command::ScrollViewSetChild { 
                                id: v.id.unwrap(), 
                                child: child.id().unwrap()
                            }
                        )
                    }
                }
                None => {
                    // remove child
                    if ov.child.is_some(){
                        cmd.push(Command::ScrollViewRemoveChild { 
                            id: v.id.unwrap()
                        })
                    }
                }
            }
            
        }
        (CoreComponent::ScrollView(v), old_component) => {
            // remove the old node
            if let Some(old) = old_component {
                cmd.push(Command::RemoveNode {
                    node: old.id().unwrap(),
                });
            }

            v.id = Some(NodeID::new_unique());

            // create scroll view
            cmd.push(Command::ScrollViewCreate { 
                id: v.id.unwrap(), 
                style: v.style.clone()
            });
        }
        (CoreComponent::Button(b), Some(CoreComponent::Button(ob))) => {
            b.id = ob.id;

            if !Arc::ptr_eq(&b.style, &ob.style){
                cmd.push(
                    Command::SetStyle { 
                        node: b.id.unwrap(), 
                        style: b.style.clone()
                    }
                )
            }

            // update diabled if changed
            if b.disabled != ob.disabled {
                cmd.push(Command::ButtonSetDisabled {
                    id: b.id.unwrap(),
                    disabled: b.disabled,
                });
            }

            // update text if changed
            if b.title != ob.title {
                cmd.push(Command::ButtonSetLabelText {
                    id: b.id.unwrap(),
                    label: b.title.clone(),
                });
            }

            // update click callback if changed
            match (&b.on_click, &ob.on_click) {
                (Some(c1), Some(c2)) => {
                    // if the two callbacks are not equal, update callback
                    if !Arc::ptr_eq(c1, c2) {
                        // set callback
                        cmd.push(Command::ButtonSetOnClick {
                            id: b.id.unwrap(),
                            on_click: Some(c1.clone()),
                        })
                    }
                }
                // no callbacks
                (None, None) => {}
                _ => {
                    cmd.push(Command::ButtonSetOnClick {
                        id: b.id.unwrap(),
                        on_click: b.on_click.clone(),
                    });
                }
            }
        }
        (CoreComponent::Button(b), old_component) => {
            // remove the old node
            if let Some(old) = old_component {
                cmd.push(Command::RemoveNode {
                    node: old.id().unwrap(),
                });
            }

            b.id = Some(NodeID::new_unique());

            cmd.push(Command::ButtonCreate {
                id: b.id.unwrap(),
                style: b.style.clone()
            });

            cmd.push(Command::ButtonSetDisabled {
                id: b.id.unwrap(),
                disabled: b.disabled,
            });

            cmd.push(Command::ButtonSetLabelText {
                id: b.id.unwrap(),
                label: b.title.clone(),
            });

            cmd.push(Command::ButtonSetOnClick {
                id: b.id.unwrap(),
                on_click: b.on_click.clone(),
            });
        }
        (CoreComponent::Text(t), Some(CoreComponent::Text(ot))) => {
            t.id = ot.id;

            if !Arc::ptr_eq(&t.style, &ot.style){
                cmd.push(
                    Command::SetStyle { 
                        node: t.id.unwrap(), 
                        style: ot.style.clone()
                    }
                )
            }

            if t.text != ot.text {
                cmd.push(Command::TextSetText {
                    id: t.id.unwrap(),
                    text: t.text.clone(),
                })
            }
        }
        (CoreComponent::Text(t), old_component) => {
            // remove the old node
            if let Some(old) = old_component {
                cmd.push(Command::RemoveNode {
                    node: old.id().unwrap(),
                });
            }

            t.id = Some(NodeID::new_unique());

            cmd.push(Command::TextCreate {
                id: t.id.unwrap(),
                style: t.style.clone(),
                text: t.text.clone(),
            });
        }
        (CoreComponent::TextInput(t), Some(CoreComponent::TextInput(ot))) => {
            t.id = ot.id;

            if !Arc::ptr_eq(&t.style, &ot.style){
                cmd.push(
                    Command::SetStyle { 
                        node: t.id.unwrap(), 
                        style: t.style.clone()
                    }
                )
            }
        }
        (CoreComponent::TextInput(t), old_component) => {
            // remove the old node
            if let Some(old) = old_component {
                cmd.push(Command::RemoveNode {
                    node: old.id().unwrap(),
                });
            }

            t.id = Some(NodeID::new_unique());

            cmd.push(Command::TextInputCreate {
                id: t.id.unwrap(),
                style: t.style.clone(),
            });

            if let Some(bg_text) = &t.background_text{
                cmd.push(Command::TextInputSetBGText { 
                    id: t.id.unwrap(), 
                    text: bg_text.clone()
                })
            }
            
        }
        (CoreComponent::StackNavigator(s), Some(CoreComponent::StackNavigator(os))) => {
            // we handle stack navigator differently.
            // stack navigator has a unique id that is referenced by the reusable `StackNavigator`

            // if they do not have the same id, the new one must be added
            if s.id != os.id {
                cmd.push(Command::StackNavigatorCreate {
                    id: s.id,
                    style: s.style.clone()
                });
            }
        }
        (CoreComponent::StackNavigator(s), old_component) => {}
        (CoreComponent::FlatList(f), Some(CoreComponent::FlatList(of))) => {
            f.id = of.id;

            if !Arc::ptr_eq(&f.style, &of.style){
                cmd.push(
                    Command::SetStyle { 
                        node: f.id.unwrap(), 
                        style: f.style.clone()
                    }
                )
            }

            cmd.push(Command::FlatListSetGetItem {
                id: f.id.unwrap(),
                get_item: f.get_item.clone(),
            });

            cmd.push(Command::FlatListSetGetLen {
                id: f.id.unwrap(),
                get_len: f.get_len.clone(),
            });

            cmd.push(Command::FlatListSetRender {
                id: f.id.unwrap(),
                render: f.render.clone(),
            });
        }
        (CoreComponent::FlatList(f), old_component) => {}
        (CoreComponent::Custom(custom), old_component) => {
            // check if the old component is also a custom component
            if let Some(CoreComponent::Custom(old_custom)) = old_component {
                // two custom comonents are the same only if they have the same type id
                if custom.wrapper.node_type_id() == old_custom.wrapper.node_type_id() {
                    // assign old id to new node
                    custom.id = old_custom.id;

                    // compare the two nodes
                    let changes = custom.wrapper.compare(&old_custom.wrapper);
                    // commit changes
                    cmd.push(Command::CustomCommitChanges {
                        id: custom.id.unwrap(),
                        changes: changes,
                    });

                    // no further action, return
                    return custom.id.unwrap();
                }
            };

            // remove the old node
            if let Some(old) = old_component {
                cmd.push(Command::RemoveNode {
                    node: old.id().unwrap(),
                });
            }

            // create new id
            custom.id = Some(NodeID::new_unique());

            // create the component
            cmd.push(Command::CustomCreate {
                id: custom.id.unwrap(),
                style: custom.wrapper.style(),
                build_fn: custom.wrapper.build_native_func(),
            });
        }
    };

    return current.id().unwrap();
}
