use relm4::{binding::U8Binding, typed_view::column::{LabelColumn, RelmColumn}, gtk::Label};

struct RowItem {
    value: u8,
    binding: U8Binding,
}

impl RowItem {
    fn new(value: u8) -> Self {
        Self { value, binding: U8Binding::new(0) }
    }
}

struct AddressColumn;

impl LabelColumn for AddressColumn {
    type Item = RowItem;

    type Value = u8;

    const COLUMN_NAME: &'static str = "Memory Address";

    const ENABLE_SORT: bool = false;

    fn get_cell_value(item: &Self::Item) -> Self::Value {
        item.value
    }

    fn format_cell_value(value: &Self::Value) -> String {
        format!("0x{:08x}", value)
    }
}

struct MemoryColumn;

impl LabelColumn for MemoryColumn {
    type Item = RowItem;

    type Value = u8;

    const COLUMN_NAME: &'static str = "Memory Contents";

    const ENABLE_SORT: bool = false;

    fn get_cell_value(item: &Self::Item) -> Self::Value {
        item.value
    }

    fn format_cell_value(value: &Self::Value) -> String {
        format!("0x{:08x}", value)
    }
}
