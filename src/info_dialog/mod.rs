use relm4::gtk::traits::{DialogExt, GtkWindowExt, WidgetExt};
use relm4::prelude::*;

pub struct InfoDialog {
    contents: String,
    visible: bool,
}

#[derive(Debug)]
pub enum DialogMsg {
    Show(String),
    Close,
}

#[relm4::component(pub)]
impl SimpleComponent for InfoDialog {
    type Input = DialogMsg;
    type Output = ();
    type Init = ();

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = InfoDialog {
            contents: "".to_string(),
            visible: false,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: DialogMsg, _sender: ComponentSender<Self>) {
        match msg {
            DialogMsg::Show(contents) => {
                self.contents = contents;
                self.visible = true;
            }
            DialogMsg::Close => {
                self.visible = false;
            }
        }
    }

    view! {
        gtk::MessageDialog {
            #[watch]
            set_text: Some(&model.contents),
            #[watch]
            set_visible: model.visible,
            set_message_type: gtk::MessageType::Info,
            set_modal: true,
            add_button: ("Ok", gtk::ResponseType::None),
            connect_response[sender] => move |_, _| {
                sender.input(DialogMsg::Close);
            },
        }
    }
}
