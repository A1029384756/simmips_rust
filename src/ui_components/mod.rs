use std::sync::{Arc, Mutex};

use crate::cpu::cpu_interface::CPUInterface;

pub mod column_views;
pub mod header;
pub mod info_dialog;
pub mod simple_view;
pub mod component_view;

pub trait CPUView {
    fn update(&self, cpu: Arc<Mutex<dyn CPUInterface>>);
}
