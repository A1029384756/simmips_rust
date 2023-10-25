use crate::ui_components::column_views::RadixValues;
use crate::ui_components::column_views::RadixValues::{BINARY, HEX};
use relm4::{gtk::traits::WidgetExt, prelude::*};
use relm4::{
    typed_view::column::{LabelColumn, TypedColumnView},
    ComponentParts, ComponentSender, SimpleComponent,
};

pub struct MemoryRow {
    addr: u32,
    value: String,
}

pub struct AddressColumn;

impl LabelColumn for AddressColumn {
    type Item = MemoryRow;

    type Value = u32;

    const COLUMN_NAME: &'static str = "Memory Address";

    const ENABLE_SORT: bool = false;

    fn get_cell_value(item: &Self::Item) -> Self::Value {
        item.addr
    }

    fn format_cell_value(value: &Self::Value) -> String {
        format!("0x{:08x}", value)
    }
}

pub struct MemoryColumn;

impl LabelColumn for MemoryColumn {
    type Item = MemoryRow;

    type Value = String;

    const COLUMN_NAME: &'static str = "Memory Contents";

    const ENABLE_SORT: bool = false;

    fn get_cell_value(item: &Self::Item) -> Self::Value {
        item.value.clone()
    }
}

pub struct MemoryView {
    radix: RadixValues,
    view_wrapper: TypedColumnView<MemoryRow, gtk::NoSelection>,
}

#[derive(Debug)]
pub enum MemoryMsg {
    UpdateMemory(Vec<u8>),
}

#[relm4::component(pub)]
impl SimpleComponent for MemoryView {
    type Input = MemoryMsg;
    type Output = crate::Msg;
    type Init = ();

    fn init(
        _: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut view_wrapper = TypedColumnView::<MemoryRow, gtk::NoSelection>::new();
        view_wrapper.append_column::<AddressColumn>();
        view_wrapper.append_column::<MemoryColumn>();

        view_wrapper.get_columns().iter().for_each(|(_, c)| {
            c.set_expand(true);
        });

        (0..1024).for_each(|idx| {
            view_wrapper.append(MemoryRow {
                addr: idx,
                value: "0x00000000".to_owned(),
            });
        });

        let model = MemoryView {
            radix: RadixValues::HEX,
            view_wrapper,
        };

        let my_view = &model.view_wrapper.view;
        my_view.set_show_row_separators(true);
        my_view.set_show_column_separators(true);
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            MemoryMsg::UpdateMemory(new_mem) => {
                self.view_wrapper.clear();
                new_mem.into_iter().enumerate().for_each(|(idx, val)| {
                    self.view_wrapper.append(MemoryRow {
                        addr: idx as u32,
                        value: match self.radix {
                            RadixValues::BINARY => format!("0b{:08b}", val),
                            RadixValues::HEX => format!("0x{:08x}", val),
                            RadixValues::DECIMAL => format!("{:08}", val),
                        },
                    });
                })
            }
        }
    }

    view! {
        memory_view = gtk::ScrolledWindow {
            set_hexpand: true,
            set_vexpand: true,
            set_margin_all: 5,
            #[local_ref]
            my_view -> gtk::ColumnView {}
        }
    }
}
