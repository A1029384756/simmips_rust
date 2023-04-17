use std::cell::Ref;

use gtk::gio::ListStore;
use gtk::glib::prelude::*;
use gtk::glib::BoxedAnyObject;
use relm4::{gtk::traits::WidgetExt, prelude::*};

use crate::column_views::grid_cell::{Entry, GridCell};

struct Row {
    reg_num: String,
    reg_alias: String,
    reg_val: String,
}

pub struct RegisterView {
    view: gtk::ColumnView,
}

#[derive(Debug)]
pub enum ViewMsg {
    UpdateRegisters(Vec<u8>),
}

#[relm4::component(pub)]
impl SimpleComponent for RegisterView {
    type Input = ViewMsg;
    type Output = crate::Msg;
    type Init = ();

    fn init(
        _: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let register_store = ListStore::new(BoxedAnyObject::static_type());

        (0..10000).for_each(|i| {
            register_store.append(&BoxedAnyObject::new(Row {
                reg_num: format!("num: {}", i),
                reg_alias: format!("alias: {}", i),
                reg_val: format!("val: {}", i),
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

        view.append_column(&column_1);
        view.append_column(&column_2);
        view.append_column(&column_3);

        let model = RegisterView { view };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _: ComponentSender<Self>) {
        match msg {
            ViewMsg::UpdateRegisters(_new_contents) => {
                let register_store = ListStore::new(BoxedAnyObject::static_type());

                (0..10000).for_each(|i| {
                    register_store.append(&BoxedAnyObject::new(Row {
                        reg_num: format!("num2: {}", i),
                        reg_alias: format!("alias2: {}", i),
                        reg_val: format!("val2: {}", i),
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
