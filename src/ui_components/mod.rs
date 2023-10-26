use std::sync::{Arc, Mutex};

use crate::cpu::cpu_interface::CPUInterface;

pub mod column_views;
pub mod component_view;
pub mod header;
pub mod simple_view;

pub trait CPUView {
    fn update(&self, cpu: Arc<Mutex<dyn CPUInterface>>);
}
