use crate::cpu::{cpu_interface::CPUInterface, single_cycle_cpu::SingleCycleCPU};

use self::column_views::Radices;

pub mod column_views;
pub mod component_view;
pub mod cpu_simulation;
pub mod history;
pub mod preferences;
pub mod simple_view;

#[derive(Debug)]
pub enum CPUViewMessage {
    Update(SingleCycleCPU),
    ChangeRadix(Radices),
    Resize((i32, i32)),
    None,
}
