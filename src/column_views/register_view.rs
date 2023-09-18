use relm4::gtk::prelude::ListItemExt;
use std::cell::Ref;

use gtk::gio::ListStore;
use gtk::glib::prelude::*;
use gtk::glib::BoxedAnyObject;
use relm4::{gtk::traits::WidgetExt, prelude::*};

use crate::column_views::grid_cell::{Entry, GridCell};

const REG_NUMBERS: [&'static str; 35] = [
    "", "", "", "$0", "$1", "$2", "$3", "$4", "$5", "$6", "$7", "$8", "$9", "$10", "$11", "$12",
    "$13", "$14", "$15", "$16", "$17", "$18", "$19", "$20", "$21", "$22", "$23", "$24", "$25",
    "$26", "$27", "$28", "$29", "$30", "$31",
];

const REG_ALIAS: [&'static str; 35] = [
    "$pc", "$hi", "$lo", "$zero", "$at", "$v0", "$v1", "$a0", "$a1", "$a2", "$a3", "$t0", "$t1",
    "$t2", "$t3", "$t4", "$t5", "$t6", "$t7", "$s0", "$s1", "$s2", "$s3", "$s4", "$s5", "$s6",
    "$s7", "$t8", "$t9", "$k0", "$k1", "$gp", "$sp", "$fp", "$ra",
];

struct Row {
    reg_num: String,
    reg_alias: String,
    reg_val: String,
}

pub struct RegisterView {
    view: gtk::ColumnView,
}

#[derive(Debug)]
pub enum RegViewMsg {
    UpdateRegisters(Vec<u32>),
}

#[relm4::component(pub)]
impl SimpleComponent for RegisterView {
    type Input = RegViewMsg;
    type Output = crate::Msg;
    type Init = ();

    fn init(
        _: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let register_store = ListStore::new::<BoxedAnyObject>();

        (0..35).for_each(|idx| {
            register_store.append(&BoxedAnyObject::new(Row {
                reg_num: format!("{}", REG_NUMBERS[idx]),
                reg_alias: format!("{}", REG_ALIAS[idx]),
                reg_val: format!("0x{:08x}", 0),
            }))
        });

        let sel = gtk::SingleSelection::new(Some(register_store));

        let view = gtk::ColumnView::new(Some(sel));

        let column_1_factory = gtk::SignalListItemFactory::new();
        let column_2_factory = gtk::SignalListItemFactory::new();
        let column_3_factory = gtk::SignalListItemFactory::new();

        column_1_factory.connect_setup(move |_factory, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let row = GridCell::new();
            item.set_child(Some(&row));
        });

        column_1_factory.connect_bind(move |_factory, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let child = item.child().and_downcast::<GridCell>().unwrap();
            let entry = item.item().and_downcast::<BoxedAnyObject>().unwrap();
            let r: Ref<Row> = entry.borrow();
            let ent = Entry {
                name: r.reg_num.to_string(),
            };
            child.set_entry(&ent);
        });

        column_2_factory.connect_setup(move |_factory, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let row = GridCell::new();
            item.set_child(Some(&row));
        });

        column_2_factory.connect_bind(move |_factory, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let child = item.child().and_downcast::<GridCell>().unwrap();
            let entry = item.item().and_downcast::<BoxedAnyObject>().unwrap();
            let r: Ref<Row> = entry.borrow();
            let ent = Entry {
                name: r.reg_alias.to_string(),
            };
            child.set_entry(&ent);
        });

        column_3_factory.connect_setup(move |_factory, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let row = GridCell::new();
            item.set_child(Some(&row));
        });

        column_3_factory.connect_bind(move |_factory, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let child = item.child().and_downcast::<GridCell>().unwrap();
            let entry = item.item().and_downcast::<BoxedAnyObject>().unwrap();
            let r: Ref<Row> = entry.borrow();
            let ent = Entry {
                name: r.reg_val.to_string(),
            };
            child.set_entry(&ent);
        });

        let column_1 = gtk::ColumnViewColumn::new(Some("Register Number"), Some(column_1_factory));
        let column_2 = gtk::ColumnViewColumn::new(Some("Register Alias"), Some(column_2_factory));
        let column_3 =
            gtk::ColumnViewColumn::new(Some("Register Contents"), Some(column_3_factory));

        column_1.set_expand(true);
        column_2.set_expand(true);
        column_3.set_expand(true);
        view.append_column(&column_1);
        view.append_column(&column_2);
        view.append_column(&column_3);
        view.set_show_row_separators(true);
        view.set_show_column_separators(true);

        let model = RegisterView { view };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _: ComponentSender<Self>) {
        match msg {
            RegViewMsg::UpdateRegisters(new_contents) => {
                let register_store = ListStore::new::<BoxedAnyObject>();

                let mut back = new_contents[..32].to_owned();
                let mut front = new_contents[32..].to_owned();

                front.reverse();
                front.append(&mut back);

                front.iter().enumerate().for_each(|(idx, val)| {
                    register_store.append(&BoxedAnyObject::new(Row {
                        reg_num: format!("{}", REG_NUMBERS[idx]),
                        reg_alias: format!("{}", REG_ALIAS[idx]),
                        reg_val: format!("0x{:08x}", val),
                    }))
                });

                let sel = gtk::SingleSelection::new(Some(register_store));

                self.view.set_model(Some(&sel));
            }
        }
    }

    view! {
        register_view = gtk::ScrolledWindow {
                set_hexpand: true,
                set_vexpand: true,
                set_margin_all: 5,
                set_child = Some(&model.view),
            }
    }
}
