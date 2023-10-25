use relm4::prelude::*;

use super::column_views::{memory_view::*, register_view::*};

pub struct SimpleView {
    register_view: Controller<RegisterView>,
    memory_view: Controller<MemoryView>,
}
