use std::any::Any;

pub struct EventBus {
    handlers: Vec<Box(dyn FnMut(Box(dyn Any))->())>
}

impl EventBus {
}