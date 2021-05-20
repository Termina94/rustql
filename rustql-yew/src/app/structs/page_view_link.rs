use crate::app::components::page_view::PageView;
use std::{cell::RefCell, rc::Rc};
use yew::{Component, ComponentLink};

#[derive(Debug, Default, Clone)]
pub struct CustomLink<T: Component> {
    pub link: Rc<RefCell<Option<ComponentLink<T>>>>,
}

impl<T: Component> CustomLink<T> {
    pub fn new() -> CustomLink<T> {
        CustomLink {
            link: Rc::new(RefCell::new(None)),
        }
    }

    pub fn send_message(&self, msg: T::Message) {
        if let Some(link) = self.link.borrow().as_ref() {
            link.send_message(msg);
        }
    }
}

impl<T: Component> PartialEq for CustomLink<T> {
    fn eq(&self, other: &Self) -> bool {
        self.link.as_ptr() == other.link.as_ptr()
    }
}
