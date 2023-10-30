use std::sync::{Arc, Mutex};

use crate::cpu::cpu_interface::CPUInterface;

use self::column_views::Radices;

pub mod column_views;
pub mod component_view;
pub mod simple_view;

#[derive(Debug)]
pub enum CPUViewMessage {
    Update(Arc<Mutex<dyn CPUInterface>>),
    ChangeRadix(Radices),
    Resize((i32, i32)),
    None,
}
