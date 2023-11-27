mod cpu;
mod ui_components;

use std::path::PathBuf;

use adw::prelude::*;
use gtk::glib;
use relm4::{factory::FactoryVecDeque, prelude::*};

use relm4_components::open_dialog::{
    OpenDialog, OpenDialogMsg, OpenDialogResponse, OpenDialogSettings,
};
use relm4_components::save_dialog::{
    SaveDialog, SaveDialogMsg, SaveDialogResponse, SaveDialogSettings,
};
use relm4_icons::icon_name;
use ui_components::{
    column_views::Radices,
    cpu_simulation::{CPUSimulation, SimulationMsg, SimulationOutput},
};

use crate::ui_components::preferences::{Preferences, UpdatePreferencesOutput};

struct App {
    simulations: FactoryVecDeque<CPUSimulation>,
    preferences_menu: Controller<Preferences>,
    file_chooser: Controller<OpenDialog>,
    file_saver: Controller<SaveDialog>,
    file_tab: Option<DynamicIndex>,
    sidebar_button_visible: bool,
    sidebar_visible: bool,
    tab_count: usize,
    save_contents: String,
}

#[derive(Debug)]
pub enum Msg {
    ShowMessage(String),
    ShowSidebarButton(bool),
    ShowSidebar(bool),
    ShowPreferences,
    ResizeHistory(usize),
    ChangeRadix(Radices),
    ChangeTheme,
    NewTab,
    OpenRequest(DynamicIndex),
    OpenResponse(PathBuf),
    SaveRequest(DynamicIndex, String, String),
    SaveResponse(PathBuf),
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

                    pack_start = &gtk::ToggleButton {
                        #[watch]
                        set_visible: model.sidebar_button_visible,
                        #[watch]
                        set_active: model.sidebar_visible,
                        set_icon_name: icon_name::DOCK_LEFT,
                        connect_clicked[sender] => move |val| {
                            sender.input(Msg::ShowSidebar(val.is_active()))
                        },
                        set_tooltip_text: Some("Show Text"),
                    },

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
                SimulationOutput::ShowSidebarButton(visible) => Msg::ShowSidebarButton(visible),
                SimulationOutput::ShowSidebar(visible) => Msg::ShowSidebar(visible),
                SimulationOutput::OpenFile(index) => Msg::OpenRequest(index),
                SimulationOutput::SaveFile(index, name, contents) => {
                    Msg::SaveRequest(index, name, contents)
                }
            });

        let preferences_menu =
            Preferences::builder()
                .launch(())
                .forward(sender.input_sender(), |msg| match msg {
                    UpdatePreferencesOutput::HistorySize(size) => Msg::ResizeHistory(size),
                    UpdatePreferencesOutput::Radix(radix) => Msg::ChangeRadix(radix),
                    UpdatePreferencesOutput::Theme => Msg::ChangeTheme,
                });

        let file_chooser = OpenDialog::builder()
            .transient_for_native(root)
            .launch(OpenDialogSettings::default())
            .forward(sender.input_sender(), |response| match response {
                OpenDialogResponse::Accept(path) => Msg::OpenResponse(path),
                OpenDialogResponse::Cancel => Msg::Ignore,
            });

        let file_saver = SaveDialog::builder()
            .transient_for_native(root)
            .launch(SaveDialogSettings::default())
            .forward(sender.input_sender(), |response| match response {
                SaveDialogResponse::Accept(path) => Msg::SaveResponse(path),
                SaveDialogResponse::Cancel => Msg::Ignore,
            });

        simulations.guard().push_back(1);
        let model = Self {
            simulations,
            preferences_menu,
            file_chooser,
            file_saver,
            file_tab: None,
            sidebar_button_visible: true,
            sidebar_visible: false,
            tab_count: 1,
            save_contents: String::new(),
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
            Msg::ShowSidebarButton(visible) => self.sidebar_button_visible = visible,
            Msg::ShowSidebar(visible) => {
                self.sidebar_visible = visible;
                self.simulations
                    .broadcast(SimulationMsg::ShowSidebar(visible));
            }
            Msg::ShowPreferences => self.preferences_menu.widget().present(),
            Msg::ChangeRadix(radix) => self
                .simulations
                .broadcast(SimulationMsg::ChangeRadix(radix)),
            Msg::ChangeTheme => self.simulations.broadcast(SimulationMsg::UpdateViews),
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
            Msg::OpenRequest(index) => {
                self.file_tab = Some(index);
                self.file_chooser.emit(OpenDialogMsg::Open);
            }
            Msg::OpenResponse(path) => {
                if let Some(index) = self.file_tab.clone() {
                    self.simulations
                        .send(index.current_index(), SimulationMsg::FileLoaded(path));
                }
            }
            Msg::SaveRequest(index, name, contents) => {
                self.file_tab = Some(index);
                self.save_contents = contents;
                self.file_saver.emit(SaveDialogMsg::SaveAs(name));
            }
            Msg::SaveResponse(path) => match std::fs::write(path, &self.save_contents) {
                Ok(_) => {
                    if let Some(index) = self.file_tab.clone() {
                        self.simulations
                            .send(index.current_index(), SimulationMsg::FileSaved);
                    }
                }
                Err(e) => sender.input(Msg::ShowMessage(e.to_string())),
            },
            Msg::Ignore => {}
        }
    }
}

fn main() {
    let app = RelmApp::new("org.simmips.gui");
    relm4_icons::initialize_icons();
    app.run::<App>(());
}
