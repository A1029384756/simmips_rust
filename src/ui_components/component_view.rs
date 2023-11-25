use super::CPUViewMessage;
use gdk_pixbuf::gio::MemoryInputStream;
use gdk_pixbuf::Pixbuf;
use gtk::prelude::*;
use relm4::drawing::DrawHandler;
use relm4::prelude::*;

pub struct ComponentView {
    handler: DrawHandler,
    imgs: Vec<Vec<u8>>,
}

#[relm4::component(pub)]
impl SimpleComponent for ComponentView {
    type Input = CPUViewMessage;
    type Output = ();
    type Init = ();

    view! {
        #[root]
        gtk::Box {
            set_spacing: 5,
            set_margin_all: 5,
            set_baseline_position: gtk::BaselinePosition::Center,
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

    fn init(
        _: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let imgs = vec![
            Vec::from(include_bytes!("resources/component_view.svg")),
            Vec::from(include_bytes!("resources/component_view_2.svg")),
        ];

        let model = ComponentView {
            handler: DrawHandler::new(),
            imgs,
        };

        let area = model.handler.drawing_area();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        let cx = self.handler.get_context();
        match msg {
            CPUViewMessage::Update(_cpu) => {}
            CPUViewMessage::Resize(size) => {
                let base_buf =
                    Pixbuf::new(gdk_pixbuf::Colorspace::Rgb, true, 8, size.0, size.1).unwrap();

                self.imgs.iter().for_each(|img_data| {
                    let stream =
                        MemoryInputStream::from_bytes(&gdk_pixbuf::glib::Bytes::from(img_data));

                    let buf = gdk_pixbuf::Pixbuf::from_stream_at_scale(
                        &stream,
                        size.0 - 10,
                        size.1 - 10,
                        true,
                        None::<&gdk_pixbuf::gio::Cancellable>,
                    )
                    .unwrap();

                    buf.composite(
                        &base_buf,
                        0,
                        0,
                        buf.width(),
                        buf.height(),
                        0.0,
                        0.0,
                        1.0,
                        1.0,
                        gdk_pixbuf::InterpType::Nearest,
                        255,
                    );
                });
                cx.set_source_pixbuf(&base_buf, 0.0, 0.0);
                cx.paint().expect("Could not fill context");
            }
            CPUViewMessage::ChangeRadix(_) => {}
            CPUViewMessage::None => {}
        }
    }
}
