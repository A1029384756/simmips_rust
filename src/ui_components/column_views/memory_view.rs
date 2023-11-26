use crate::ui_components::column_views::Radices;
use relm4::{gtk::traits::WidgetExt, prelude::*};
use relm4::{
    typed_view::column::{LabelColumn, TypedColumnView},
    ComponentParts, ComponentSender, SimpleComponent,
};

use super::RadixedValue;

pub struct MemoryRow {
    addr: u32,
    value: RadixedValue<u8>,
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

    type Value = RadixedValue<u8>;

    const COLUMN_NAME: &'static str = "Memory Contents";

    const ENABLE_SORT: bool = false;

    fn get_cell_value(item: &Self::Item) -> Self::Value {
        item.value
    }

    fn format_cell_value(value: &Self::Value) -> String {
        match value.radix {
            Radices::Binary => format!("0b{:032b}", value.value),
            Radices::Hex => format!("0x{:08x}", value.value),
            Radices::Decimal => format!("{:010}", value.value),
        }
    }
}

pub struct MemoryView {
    view_wrapper: TypedColumnView<MemoryRow, gtk::NoSelection>,
    curr_radix: Radices,
}

#[derive(Debug)]
pub enum MemoryMsg {
    UpdateMemory(Vec<u8>),
    UpdateRadix(Radices),
}

#[relm4::component(pub)]
impl SimpleComponent for MemoryView {
    type Input = MemoryMsg;
    type Output = crate::Msg;
    type Init = ();

    view! {
        memory_view = gtk::ScrolledWindow {
            set_hexpand: true,
            set_vexpand: true,
            set_margin_all: 5,
            #[local_ref]
            my_view -> gtk::ColumnView {}
        }
    }

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
                value: RadixedValue {
                    radix: Radices::Hex,
                    value: 0,
                },
            });
        });

        let model = MemoryView {
            view_wrapper,
            curr_radix: Radices::Hex,
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
                let radix = self.curr_radix;
                self.view_wrapper.clear();
                new_mem.into_iter().enumerate().for_each(|(idx, val)| {
                    self.view_wrapper.append(MemoryRow {
                        addr: idx as u32,
                        value: RadixedValue { radix, value: val },
                    });
                })
            }
            MemoryMsg::UpdateRadix(radix) => {
                self.curr_radix = radix;
                let mut new_list: Vec<u8> = Vec::new();

                (0..self.view_wrapper.len()).for_each(|v| {
                    self.view_wrapper.get(v).iter().for_each(|rv| {
                        new_list.push(rv.borrow().value.value);
                    });
                });

                self.view_wrapper.clear();
                new_list.iter().enumerate().for_each(|(idx, val)| {
                    self.view_wrapper.append(MemoryRow {
                        addr: idx as u32,
                        value: RadixedValue { radix, value: *val },
                    });
                });
            }
        }
    }
}
