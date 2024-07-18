
mod view;

pub use view::HtmlView;

pub trait NativeElement{
    fn as_html_element(&self) -> &web_sys::HtmlElement;
}