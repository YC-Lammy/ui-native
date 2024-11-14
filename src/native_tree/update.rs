use super::node::NativeComponent;
use super::{NativeImageViewImp, NativeTree};






impl<'a> NativeTree<'a>{
    pub(crate) fn check_update(&self){
        for (_, node) in &self.nodes{
            match node.component.as_ref(){
                NativeComponent::ImageView(im) => im.check_update(),
                _ => {}
            }
        }
    }
}