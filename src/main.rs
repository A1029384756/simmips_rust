mod cpu;
mod ui_components;

use adw::prelude::*;
use gtk::glib;
use relm4::{factory::FactoryVecDeque, prelude::*};

use relm4_icons::icon_name;
use ui_components::{
    column_views::Radices,
    cpu_simulation::{CPUSimulation, SimulationMsg, SimulationOutput},
};

use crate::ui_components::preferences::{Preferences, UpdatePreferencesOutput};

struct App {
    simulations: FactoryVecDeque<CPUSimulation>,
    preferences_menu: Controller<Preferences>,
    tab_count: usize,
}

#[derive(Debug)]
pub enum Msg {
    ShowMessage(String),

    ShowPreferences,
    ResizeHistory(usize),
    ChangeRadix(Radices),

    NewTab,

    Ignore,
}

#[relm4::component]
impl Component for App {
    type Input = Msg;
    type Output = ();
    type CommandOutput = ();
    type Init = ();

    view! {
        adw::Window {
            set_default_size: (1000, 800),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &gtk::Label { set_markup: "<b>SIMMIPS</b>" },

                    pack_end = &gtk::Button {
                        set_icon_name: icon_name::SETTINGS,
                        connect_clicked => Msg::ShowPreferences,
                        set_tooltip_text: Some("Preferences"),
                    },

                    pack_end = &gtk::Button {
                        #[watch]
                        set_icon_name: icon_name::PLUS,
                        connect_clicked[sender] => move |_| { sender.input(Msg::NewTab) },
                        set_tooltip_text: Some("New Tab"),
                    },
                },

                adw::TabBar {
                    set_view: Some(tab_view),
                },

                #[local_ref]
                tab_view -> adw::TabView {}
            },
        },
    }

    fn init(
        _: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut simulations = FactoryVecDeque::builder()
            .launch(adw::TabView::default())
            .forward(sender.input_sender(), |output| match output {
                SimulationOutput::ShowMessage(message) => Msg::ShowMessage(message),
            });

        let preferences_menu =
            Preferences::builder()
                .launch(())
                .forward(sender.input_sender(), |msg| match msg {
                    UpdatePreferencesOutput::HistorySizeChanged(size) => Msg::ResizeHistory(size),
                    UpdatePreferencesOutput::RadixChanged(radix) => Msg::ChangeRadix(radix),
                });

        simulations.guard().push_back(1);
        let model = Self {
            simulations,
            preferences_menu,
            tab_count: 1,
        };

        let tab_view = model.simulations.widget();

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Msg, sender: ComponentSender<Self>, root: &Self::Root) {
        match msg {
            Msg::ShowMessage(message) => {
                let dialog = adw::MessageDialog::builder()
                    .transient_for(root)
                    .body(message)
                    .build();
                dialog.add_response("Ok", "Ok");
                dialog.set_default_response(Some("Ok"));
                dialog.connect_response(
                    None,
                    glib::clone!(@strong sender=>move|dialog,_|{dialog.close();}),
                );
                dialog.present();
            }
            Msg::ShowPreferences => self.preferences_menu.widget().present(),
            Msg::ChangeRadix(radix) => self
                .simulations
                .broadcast(SimulationMsg::ChangeRadix(radix)),
            Msg::ResizeHistory(size) => self
                .simulations
                .broadcast(SimulationMsg::ResizeHistory(size)),
            Msg::NewTab => {
                self.tab_count += 1;
                self.simulations.guard().push_back(self.tab_count);
                sender
                    .input_sender()
                    .emit(Msg::ChangeRadix(self.preferences_menu.model().radix));
                sender.input_sender().emit(Msg::ResizeHistory(
                    self.preferences_menu.model().history_size,
                ));
            }
            Msg::Ignore => {}
        }
    }
}

fn main() {
    let app = RelmApp::new("org.simmips.gui");
    relm4_icons::initialize_icons();
    app.run::<App>(());
}
