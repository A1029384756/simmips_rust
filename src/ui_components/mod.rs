use crate::cpu::single_cycle_cpu::SingleCycleCPU;

use self::column_views::Radices;

pub mod asm_view;
pub mod column_views;
pub mod component_view;
pub mod cpu_simulation;
pub mod history;
pub mod preferences;
pub mod simple_view;

#[derive(Debug)]
pub enum CPUViewMessage {
    Update(Box<SingleCycleCPU>),
    ChangeRadix(Radices),
    Resize((i32, i32)),
    None,
}

#[cfg(not(windows))]
#[macro_export]
macro_rules! main_separator {
    () => {
        "/"
    };
}

#[cfg(windows)]
#[macro_export]
macro_rules! main_separator {
    () => {
        r#"\"#
    };
}
