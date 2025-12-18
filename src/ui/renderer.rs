use crate::ui::{Preview, Selector};

pub struct Renderer {
    pub preview: Preview,
    pub selector: Selector,
}

impl Renderer {
    pub fn new(preview: Preview, selector: Selector) -> Self {
        Renderer { preview, selector }
    }
}
