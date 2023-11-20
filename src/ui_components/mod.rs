use crate::cpu::{cpu_interface::CPUInterface, single_cycle_cpu::SingleCycleCPU};

use self::column_views::Radices;

pub mod column_views;
pub mod component_view;
pub mod simple_view;
pub mod history;

#[derive(Debug)]
pub enum CPUViewMessage {
    Update(SingleCycleCPU),
    ChangeRadix(Radices),
    Resize((i32, i32)),
    None,
}
