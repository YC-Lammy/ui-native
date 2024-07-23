use web_sys::wasm_bindgen::JsCast;

use crate::native_tree::NativeViewImp;

use super::NativeElement;

pub struct HtmlView {
    div: web_sys::HtmlDivElement,
}

impl NativeElement for HtmlView {
    fn as_html_element(&self) -> &web_sys::HtmlElement {
        &self.div
    }
}

impl NativeViewImp for HtmlView {
    fn new() -> Self {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let div = document
            .create_element("div")
            .expect("should be able to create div");

        return Self {
            div: div.unchecked_into(),
        };
    }
    fn set_width(&self, width: f32) {
        let _ = self
            .div
            .style()
            .set_property("width", &format!("{}px", width as i32));
    }
    fn set_height(&self, height: f32) {
        let _ = self
            .div
            .style()
            .set_property("height", &format!("{}px", height as i32));
    }
    fn set_visible(&self, visible: bool) {
        let visibility = if visible { "visible" } else { "hidden" };
        let _ = self.div.style().set_property("visibility", visibility);
    }
    fn insert_child(&self, index: usize, elem: &dyn NativeElement) {
        let children = self.div.children();
        if let Some(old) = children.get_with_index(index as u32) {
            let _ = self.div.replace_child(&old, elem.as_html_element());
        } else {
            let _ = self.div.append_child(elem.as_html_element());
        }
    }
    fn remove_child(&self, elem: &dyn NativeElement) {
        let _ = self.div.remove_child(elem.as_html_element());
    }
    fn set_child_position(&self, child: &dyn NativeElement, x: f32, y: f32) {
        let elem = child.as_html_element();
        let style = elem.style();
        let _ = style.set_property("position", "absolute");
        let _ = style.set_property("top", &format!("{}px", x as i32));
        let _ = style.set_property("left", &format!("{}px", y as i32));
    }
}
