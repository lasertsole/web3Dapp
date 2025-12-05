use std::any::Any;

pub struct EventDelegate {
    handlers: Vec<Box(dyn FnMut(Box(dyn Any))->())>
}

impl EventDelegate {
}