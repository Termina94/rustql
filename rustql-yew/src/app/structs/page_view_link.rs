use std::{cell::RefCell, rc::Rc};
use yew::{Component, ComponentLink};
use crate::app::components::page_view::PageView;

#[derive(Debug, Default, Clone)]
pub struct PageViewLink<T: Component> {
    pub link: Rc<RefCell<Option<ComponentLink<T>>>>
}

impl PageViewLink<PageView> {
    pub fn new() -> PageViewLink<PageView> {
        PageViewLink {
            link: Rc::new(RefCell::new(None)),
        }
    }
}

impl PartialEq for PageViewLink<PageView> {
    fn eq(&self, other: &Self) -> bool {
        self.link.as_ptr() == other.link.as_ptr()
    }
}