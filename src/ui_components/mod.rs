use std::sync::{Arc, Mutex};

use crate::cpu::cpu_interface::CPUInterface;

pub mod column_views;
pub mod component_view;
pub mod simple_view;

#[derive(Debug)]
pub enum CPUViewMessage {
    Update(Arc<Mutex<dyn CPUInterface>>),
    Resize((i32, i32)),
    None,
}
