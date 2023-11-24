use crate::cpu::single_cycle_cpu::SingleCycleCPU;

use super::CPUInterface;
use super::CPUViewMessage;
use gtk::prelude::*;
use relm4::drawing::DrawHandler;
use relm4::gtk::cairo::{Context, Operator};
use relm4::gtk::traits::BoxExt;
use relm4::prelude::*;

pub struct ComponentView {
    handler: DrawHandler,
}

#[relm4::component(pub)]
impl SimpleComponent for ComponentView {
    type Input = CPUViewMessage;
    type Output = ();
    type Init = ();

    fn init(
        _: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ComponentView {
            handler: DrawHandler::new(),
        };

        let area = model.handler.drawing_area();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        let cx = self.handler.get_context();
        match msg {
            CPUViewMessage::Update(cpu) => {
                cx.set_source_rgba(255.0, 255.0, 255.0, 255.0);
                cx.paint().expect("Could not fill context");
            }
            CPUViewMessage::Resize((x, y)) => {
                cx.set_source_rgba(255.0, 255.0, 255.0, 255.0);
                cx.paint().expect("Could not fill context");
            }
            CPUViewMessage::ChangeRadix(_) => {}
            CPUViewMessage::None => {}
        }
    }

    view! {
        #[root]
        gtk::Box {
            set_spacing: 5,
            set_margin_all: 5,
            #[local_ref]
            area -> gtk::DrawingArea {
                set_vexpand: true,
                set_hexpand: true,
                connect_resize[sender] => move |_, x, y| {
                    sender.input(CPUViewMessage::Resize((x, y)))
                }
            },
        },
    }
}

impl ComponentView {
    fn draw_view(&mut self, cpu: SingleCycleCPU) {}
}
