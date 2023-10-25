use relm4::gtk::traits::{ButtonExt, ToggleButtonExt, WidgetExt};
use relm4::prelude::*;

pub struct HeaderView;

#[derive(Debug)]
pub enum HeaderMsg {
    SimpleView,
    ComponentView,
}

#[relm4::component(pub)]
impl SimpleComponent for HeaderView {
    type Init = ();
    type Input = ();
    type Output = HeaderMsg;

    fn init(
        _params: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = HeaderView;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    view! {
        #[root]
        gtk::HeaderBar {
            #[wrap(Some)]
            set_title_widget = &gtk::Box {
                add_css_class: "linked",
                #[name = "group"]
                gtk::ToggleButton {
                    set_label: "Simple View",
                    set_active: true,
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.output(HeaderMsg::SimpleView).unwrap()
                        }
                    }
                },
                gtk::ToggleButton {
                    set_label: "Component View",
                    set_group: Some(&group),
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.output(HeaderMsg::ComponentView).unwrap()
                        }
                    }
                },
            }
        }
    }
}
