use std::cell::Ref;
use relm4::gtk::prelude::ListItemExt;

use gtk::gio::ListStore;
use gtk::glib::prelude::*;
use gtk::glib::BoxedAnyObject;
use relm4::{gtk::traits::WidgetExt, prelude::*};

use crate::column_views::grid_cell::{Entry, GridCell};

struct Row {
    mem_addr: String,
    mem_contents: String,
}

pub struct MemoryView {
    view: gtk::ColumnView,
}

#[derive(Debug)]
pub enum MemoryViewMsg {
    UpdateMemory (Vec<u8>),
}

#[relm4::component(pub)]
impl SimpleComponent for MemoryView {
    type Input = MemoryViewMsg;
    type Output = crate::Msg;
    type Init = ();

    fn init(
        _: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let register_store = ListStore::new::<BoxedAnyObject>();

        let sel = gtk::SingleSelection::new(Some(register_store));

        let view = gtk::ColumnView::new(Some(sel));

        let column_1_factory = gtk::SignalListItemFactory::new();
        let column_2_factory = gtk::SignalListItemFactory::new();

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
                name: r.mem_addr.to_string(),
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
                name: r.mem_contents.to_string(),
            };
            child.set_entry(&ent);
        });
        let column_1 = gtk::ColumnViewColumn::new(Some("Memory Address"), Some(column_1_factory));
        let column_2 = gtk::ColumnViewColumn::new(Some("Memory Contents"), Some(column_2_factory));

        column_1.set_expand(true);
        column_2.set_expand(true);
        view.append_column(&column_1);
        view.append_column(&column_2);
        view.set_show_row_separators(true);
        view.set_show_column_separators(true);

        let model = MemoryView { view };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _: ComponentSender<Self>) {
        match msg {
            MemoryViewMsg::UpdateMemory(new_contents) => {
                let register_store = ListStore::new::<BoxedAnyObject>();
                new_contents.iter().enumerate().for_each(|(idx, val)| {
                    register_store.append(&BoxedAnyObject::new(Row {
                        mem_addr: format!("0x{:08x}", idx),
                        mem_contents: format!("0x{:02x}", val),
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
