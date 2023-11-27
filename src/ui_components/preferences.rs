use relm4::{adw::prelude::*, prelude::*};
use relm4_icons::icon_name;

use super::column_views::Radices;

#[derive(Debug)]
pub enum ColorScheme {
    Light,
    Dark,
    Default,
}

#[derive(Debug)]
pub struct Preferences {
    color_scheme: ColorScheme,
    pub history_size: usize,
    pub radix: Radices,
}

#[derive(Debug)]
pub enum UpdatePreferencesInput {
    ColorScheme(ColorScheme),
    HistorySize(usize),
    Radix(Radices),
}

#[derive(Debug)]
pub enum UpdatePreferencesOutput {
    HistorySize(usize),
    Radix(Radices),
    Theme,
}

#[relm4::component(pub)]
impl Component for Preferences {
    type CommandOutput = ();
    type Input = UpdatePreferencesInput;
    type Output = UpdatePreferencesOutput;
    type Init = ();

    view! {
        adw::PreferencesWindow {
            set_title: Some("Preferences"),
            set_hide_on_close: true,
            #[wrap(Some)]
            #[name = "overlay"]
            set_content = &adw::ToastOverlay {
                #[wrap(Some)]
                set_child = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    append = &adw::HeaderBar {
                        set_show_end_title_buttons: true,
                    },
                    append = &adw::Clamp {
                        #[wrap(Some)]
                        set_child = &adw::PreferencesPage {
                            set_vexpand: true,
                            add = &adw::PreferencesGroup {
                                set_title: "Appearance",
                                adw::ComboRow {
                                    set_title: "Color Scheme",
                                    add_prefix = &gtk::Image {
                                        set_icon_name: Some(icon_name::DARK_MODE),
                                    },
                                    set_model: Some(&gtk::StringList::new(&[
                                        "Light",
                                        "Dark",
                                        "Default",
                                    ])),
                                    set_selected: match model.color_scheme {
                                        ColorScheme::Light => 0,
                                        ColorScheme::Dark => 1,
                                        ColorScheme::Default => 2,
                                    },
                                    connect_selected_notify[sender] => move |combo_row| {
                                        match combo_row.selected() {
                                        0 => sender.input_sender().send(UpdatePreferencesInput::ColorScheme(ColorScheme::Light)).unwrap(),
                                        1 => sender.input_sender().send(UpdatePreferencesInput::ColorScheme(ColorScheme::Dark)).unwrap(),
                                        _ => sender.input_sender().send(UpdatePreferencesInput::ColorScheme(ColorScheme::Default)).unwrap(),
                                        }
                                    }
                                },
                            },
                            add = &adw::PreferencesGroup {
                                set_title: "Undo History",
                                adw::EntryRow {
                                    set_title: "Undo Steps",
                                    set_text: &model.history_size.to_string(),
                                    set_input_purpose: gtk::InputPurpose::Digits,
                                    set_show_apply_button: true,
                                    connect_apply[sender] => move |entry_row| {
                                        sender.input_sender()
                                            .send(UpdatePreferencesInput::HistorySize(
                                                entry_row.text().to_string().parse::<usize>().unwrap_or(10)
                                            )).unwrap();
                                    },
                                },
                            },
                            add = &adw::PreferencesGroup {
                                set_title: "Display",
                                adw::ComboRow {
                                    set_title: "Radix",
                                    add_prefix = &gtk::Image {
                                        set_icon_name: Some(icon_name::EMERGENCY_NUMBER),
                                    },
                                    set_model: Some(&gtk::StringList::new(&[
                                        "Hex",
                                        "Decimal",
                                        "Binary",
                                    ])),
                                    set_selected: match model.radix {
                                        Radices::Hex => 0,
                                        Radices::Decimal => 1,
                                        Radices::Binary => 2,
                                    },
                                    connect_selected_notify[sender] => move |combo_row| {
                                        match combo_row.selected() {
                                        0 => sender.input_sender().send(UpdatePreferencesInput::Radix(Radices::Hex)).unwrap(),
                                        1 => sender.input_sender().send(UpdatePreferencesInput::Radix(Radices::Decimal)).unwrap(),
                                        _ => sender.input_sender().send(UpdatePreferencesInput::Radix(Radices::Binary)).unwrap(),
                                        }
                                    }
                                },
                            },
                        },
                    },
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            color_scheme: ColorScheme::Default,
            history_size: 10,
            radix: Radices::Hex,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            UpdatePreferencesInput::ColorScheme(color) => {
                match color {
                    ColorScheme::Light => {
                        adw::StyleManager::default().set_color_scheme(adw::ColorScheme::ForceLight);
                    }
                    ColorScheme::Dark => {
                        adw::StyleManager::default().set_color_scheme(adw::ColorScheme::ForceDark);
                    }
                    ColorScheme::Default => {
                        adw::StyleManager::default().set_color_scheme(adw::ColorScheme::Default);
                    }
                };

                sender.output(UpdatePreferencesOutput::Theme).unwrap();
            }
            UpdatePreferencesInput::HistorySize(size) => {
                if self.history_size != size {
                    self.history_size = size;
                    sender
                        .output(UpdatePreferencesOutput::HistorySize(size))
                        .unwrap();
                }
            }
            UpdatePreferencesInput::Radix(radix) => {
                if self.radix != radix {
                    self.radix = radix;
                    sender
                        .output(UpdatePreferencesOutput::Radix(radix))
                        .unwrap();
                }
            }
        }
    }
}
