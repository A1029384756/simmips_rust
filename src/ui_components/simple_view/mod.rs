use std::sync::{Arc, Mutex};

use gtk::prelude::*;
use num::FromPrimitive;
use relm4::gtk::traits::BoxExt;
use relm4::prelude::*;

use crate::cpu::cpu_interface::CPUInterface;

use super::column_views::{memory_view::*, register_view::*};
use super::CPUView;

pub struct SimpleView {
    register_view: Controller<RegisterView>,
    memory_view: Controller<MemoryView>,
}

#[derive(Debug)]
pub enum SimpleViewMessage {
    Foo,
}

#[relm4::component(pub)]
impl SimpleComponent for SimpleView {
    type Input = SimpleViewMessage;
    type Output = ();
    type Init = ();

    fn init(
        _: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let register_view: Controller<RegisterView> = RegisterView::builder()
            .launch(())
            .forward(sender.input_sender(), |_| SimpleViewMessage::Foo);

        let memory_view: Controller<MemoryView> = MemoryView::builder()
            .launch(())
            .forward(sender.input_sender(), |_| SimpleViewMessage::Foo);

        let model = SimpleView {
            register_view,
            memory_view,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            SimpleViewMessage::Foo => todo!(),
        }
    }

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 5,
            set_margin_all: 5,
            append: model.register_view.widget(),
            append: model.memory_view.widget(),
        },
    }
}

impl CPUView for SimpleView {
    fn update(&self, cpu: Arc<Mutex<dyn CPUInterface>>) {
        if let Ok(cpu) = cpu.lock() {
            self.register_view.emit(RegMsg::UpdateRegisters(
                (0..33)
                    .map(|idx| cpu.get_register(FromPrimitive::from_i32(idx).unwrap()))
                    .collect(),
            ));
            self.memory_view.emit(MemoryMsg::UpdateMemory(
                (0..cpu.get_memory_size())
                    .map(|idx| cpu.get_memory_byte(idx).unwrap())
                    .collect(),
            ));
        }
    }
}
