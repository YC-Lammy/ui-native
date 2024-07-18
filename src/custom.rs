use std::any::{Any, TypeId};
use  std::sync::Arc;

use crate::shadow_tree::component::{CoreComponent, CustomNode};

/// native element shadow node implementation
pub trait NativeElementShadowNodeImpl: Any + Clone + Send + Sync{
    type NativeNode: NativeElementNativeNodeImpl<Changes = Self::Changes>;
    type Changes: Send + Sync + 'static;
    /// build the native node. It is guaranteed that this will be called on the main thread.
    fn build_native(&self) -> Self::NativeNode;
    /// compares two shadow node and report any changes.
    /// These changes will be committed to the native node via `commit_changes`.
    fn compare(&self, old: &Self) -> Vec<Self::Changes>;
    /// return an iterator of child nodes if any
    fn children(&self) -> impl Iterator<Item = &mut dyn crate::private::ElementLike>;
}

/// native element native node implementation
pub trait NativeElementNativeNodeImpl: crate::imp::NativeElement{
    type Changes: 'static;
    /// commit any changes since last render
    fn commit_changes(&self, changes: &[Self::Changes]);
    /// measures the actual size of the element, return None if not native dependent
    fn measure(&self) -> Option<(f32, f32)>{
        None
    }
    /// set the width of the element
    fn set_width(&self, width: f32);
    /// set the height of the element
    fn set_height(&self, height: f32);
}

/// a wrapper trait to perform dynamic dispatch
pub(crate) trait NativeCustomElement: crate::imp::NativeElement{
    fn commit_custom_changes(&self, changes: Box<dyn Any>);
    fn measure_custom_size(&self) -> Option<(f32, f32)>;
    fn set_custom_width(&self, width: f32);
    fn set_custom_height(&self, height: f32);
    /// workaround for experimental trait upcasting.
    /// trait upcasting coercion is experimental
    /// see issue #65991 <https://github.com/rust-lang/rust/issues/65991>
    fn as_native_element(&self) -> &dyn crate::imp::NativeElement;
}

/// implement the wrapper trait
impl<T, C: 'static> NativeCustomElement for T where T: NativeElementNativeNodeImpl<Changes = C> {
    fn commit_custom_changes(&self, changes: Box<dyn Any>) {
        let changes = changes.downcast_ref::<Vec<C>>().unwrap();
        self.commit_changes(&changes);
    }
    fn measure_custom_size(&self) -> Option<(f32, f32)>{
        self.measure()
    }
    fn set_custom_width(&self, width: f32) {
        self.set_width(width)
    }
    fn set_custom_height(&self, height: f32) {
        self.set_height(height)
    }
    fn as_native_element(&self) -> &dyn crate::imp::NativeElement {
        self
    }
}

#[derive(Clone)]
pub(crate) struct CustomElementWrapper{
    type_id: TypeId,
    children: Vec<CoreComponent>,
    render_children: Arc<dyn Fn(&mut Vec<CoreComponent>) + Send + Sync>,
    compare: Arc<dyn Fn(&dyn Any) -> Box<dyn Any + Send + Sync> + Send + Sync>,
    build_native: Arc<dyn Fn() -> Box<dyn NativeCustomElement> + Send + Sync>
}

impl CustomElementWrapper{
    pub fn node_type_id(&self) -> TypeId{
        self.type_id
    }
    pub fn children_mut(&mut self) -> &mut [CoreComponent]{
        &mut self.children
    }
    pub fn children(&self) -> &[CoreComponent]{
        &self.children
    }
    pub fn compare(&self, old: &Self) -> Box<dyn Any + Send + Sync>{
        (self.compare)(old)
    }
    pub fn build_native_func(&self) -> Arc<dyn Fn() -> Box<dyn NativeCustomElement> + Send + Sync>{
        self.build_native.clone()
    }
}

impl crate::private::NativeElement for CustomElementWrapper{
    fn core_component(&self) -> CoreComponent {
        CoreComponent::Custom(Box::new(CustomNode{
            id: None,
            wrapper: self.clone()
        }))
    }
    fn on_state_change(&mut self, _ctx: &crate::Context) {}
    fn render(&mut self) {
        (self.render_children)(&mut self.children)
    }
}

impl crate::private::ElementLike for CustomElementWrapper{
    fn as_native(&mut self) -> Option<&mut dyn crate::private::NativeElement> {
        Some(self)
    }
    fn as_element(&mut self) -> Option<&mut dyn crate::Element> {
        None
    }
}

impl<T, C: Send + Sync +'static> crate::Element for T where T:NativeElementShadowNodeImpl<Changes = C>{
    fn on_state_change(&self, _ctx: &crate::Context) {}
    fn render(&self) -> crate::ElementLike {
        let shadow = Arc::new(self.clone());
        let shadow1 = shadow.clone();
        let shadow2 = shadow.clone();
        let shadow3 = shadow.clone();

        let render_children = move |child_components: &mut Vec<CoreComponent>|{
            let children = shadow1.children();

            for child in children{
                let component = match child.render(){
                    Ok(comp) => comp,
                    Err(mut e) => loop{
                        match e.render(){
                            Ok(comp) => break comp,
                            Err(r) => e = r
                        }
                    }
                };
                child_components.push(component);
            }
        };
        let compare = move |old: &dyn Any|{
            let old = old.downcast_ref::<Self>().unwrap();
            let changes = shadow2.compare(old);
            let re: Box<dyn Any + Send + Sync> = Box::new(changes);
            return re
        };
        let build_native = move ||{
            let elem = shadow3.build_native();
            let elem: Box<dyn NativeCustomElement> = Box::new(elem);
            return elem
        };

        Box::new(CustomElementWrapper{
            type_id: self.type_id(),
            children: Vec::new(),
            render_children: Arc::new(render_children),
            compare: Arc::new(compare),
            build_native: Arc::new(build_native)
        })
    }
}