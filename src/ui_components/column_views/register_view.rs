use super::RadixedValue;
use crate::ui_components::column_views::Radices;
use relm4::{gtk::traits::WidgetExt, prelude::*};
use relm4::{
    typed_view::column::{LabelColumn, TypedColumnView},
    ComponentParts, ComponentSender, SimpleComponent,
};

const REG_NUMBERS: [&str; 33] = [
    "", "$0", "$1", "$2", "$3", "$4", "$5", "$6", "$7", "$8", "$9", "$10", "$11", "$12", "$13",
    "$14", "$15", "$16", "$17", "$18", "$19", "$20", "$21", "$22", "$23", "$24", "$25", "$26",
    "$27", "$28", "$29", "$30", "$31",
];

const REG_ALIAS: [&str; 33] = [
    "$pc", "$zero", "$at", "$v0", "$v1", "$a0", "$a1", "$a2", "$a3", "$t0", "$t1", "$t2", "$t3",
    "$t4", "$t5", "$t6", "$t7", "$s0", "$s1", "$s2", "$s3", "$s4", "$s5", "$s6", "$s7", "$t8",
    "$t9", "$k0", "$k1", "$gp", "$sp", "$fp", "$ra",
];

#[derive(Debug)]
pub struct RegisterRow {
    reg_num: &'static str,
    reg_alias: &'static str,
    reg_val: RadixedValue<u32>,
}

pub struct RegNumColumn;

impl LabelColumn for RegNumColumn {
    type Item = RegisterRow;

    type Value = &'static str;

    const COLUMN_NAME: &'static str = "Register Name";

    const ENABLE_SORT: bool = false;

    fn get_cell_value(item: &Self::Item) -> Self::Value {
        item.reg_num
    }
}

pub struct RegAliasColumn;

impl LabelColumn for RegAliasColumn {
    type Item = RegisterRow;

    type Value = &'static str;

    const COLUMN_NAME: &'static str = "Register Alias";

    const ENABLE_SORT: bool = false;

    fn get_cell_value(item: &Self::Item) -> Self::Value {
        item.reg_alias
    }
}

pub struct RegisterColumn;

impl LabelColumn for RegisterColumn {
    type Item = RegisterRow;

    type Value = RadixedValue<u32>;

    const COLUMN_NAME: &'static str = "Register Contents";

    const ENABLE_SORT: bool = false;

    fn get_cell_value(item: &Self::Item) -> Self::Value {
        item.reg_val
    }

    fn format_cell_value(value: &Self::Value) -> String {
        match value.radix {
            Radices::Binary => format!("0b{:032b}", value.value),
            Radices::Hex => format!("0x{:08x}", value.value),
            Radices::Decimal => format!("{:010}", value.value),
        }
    }
}

pub struct RegisterView {
    view_wrapper: TypedColumnView<RegisterRow, gtk::NoSelection>,
    curr_radix: Radices,
}

#[derive(Debug)]
pub enum RegMsg {
    UpdateRegisters(Vec<u32>),
    UpdateRadix(Radices),
}

#[relm4::component(pub)]
impl SimpleComponent for RegisterView {
    type Input = RegMsg;
    type Output = crate::Msg;
    type Init = ();

    view! {
        register_view = gtk::ScrolledWindow {
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
        let mut view_wrapper = TypedColumnView::<RegisterRow, gtk::NoSelection>::new();
        view_wrapper.append_column::<RegNumColumn>();
        view_wrapper.append_column::<RegAliasColumn>();
        view_wrapper.append_column::<RegisterColumn>();

        view_wrapper.get_columns().iter().for_each(|(_, c)| {
            c.set_expand(true);
        });

        (0..33).for_each(|idx| {
            view_wrapper.append(RegisterRow {
                reg_num: REG_NUMBERS[idx],
                reg_alias: REG_ALIAS[idx],
                reg_val: RadixedValue {
                    radix: Radices::Hex,
                    value: 0,
                },
            });
        });

        let model = RegisterView {
            view_wrapper,
            curr_radix: Radices::Hex,
        };

        let my_view = &model.view_wrapper.view;
        my_view.set_show_row_separators(true);
        my_view.set_show_column_separators(true);
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _: ComponentSender<Self>) {
        match msg {
            RegMsg::UpdateRegisters(new_registers) => {
                let radix = self.curr_radix;

                self.view_wrapper.clear();
                let mut back = new_registers[..32].to_owned();
                let mut front = new_registers[32..].to_owned();

                front.reverse();
                front.append(&mut back);

                front.iter().enumerate().for_each(|(idx, val)| {
                    self.view_wrapper.append(RegisterRow {
                        reg_num: REG_NUMBERS[idx],
                        reg_alias: REG_ALIAS[idx],
                        reg_val: RadixedValue { radix, value: *val },
                    });
                });
            }
            RegMsg::UpdateRadix(radix) => {
                let mut new_list: Vec<u32> = Vec::new();

                (0..self.view_wrapper.len()).for_each(|v| {
                    self.view_wrapper.get(v).iter().for_each(|rv| {
                        new_list.push(rv.borrow().reg_val.value);
                    });
                });

                self.view_wrapper.clear();
                new_list.iter().enumerate().for_each(|(idx, val)| {
                    self.view_wrapper.append(RegisterRow {
                        reg_num: REG_NUMBERS[idx],
                        reg_alias: REG_ALIAS[idx],
                        reg_val: RadixedValue { radix, value: *val },
                    });
                });
            }
        }
    }
}
