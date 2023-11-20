mod cpu;
mod ui_components;

use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::Sender;

use adw::prelude::*;
use gtk::glib;
use relm4::prelude::*;
use relm4_components::open_dialog::*;
use relm4_icons::icon_name;

use cpu::cpu_interface::CPUInterface;
use cpu::single_cycle_cpu::SingleCycleCPU;
use mips_assembler::parse;

use ui_components::column_views::Radices;
use ui_components::component_view::ComponentView;
use ui_components::history::History;
use ui_components::simple_view::SimpleView;
use ui_components::CPUViewMessage;

struct App {
    open_dialog: Controller<OpenDialog>,
    simple_view: Controller<SimpleView>,
    component_view: Controller<ComponentView>,
    mode: AppMode,
    history: History<SingleCycleCPU>,
    asm_view_buffer: gtk::TextBuffer,
    app_to_thread: Option<Sender<()>>,
    cpu_running: bool,
    sidebar_visible: bool,
    sidebar_button_visible: bool,
}

#[derive(Debug)]
pub enum Msg {
    OpenRequest,
    OpenResponse(PathBuf),
    Ignore,
    Step,
    Run,
    Break,
    Undo,
    Redo,
    ResetSimulation,
    UpdateViews,
    ShowMessage(String),
    SetMode(AppMode),
    ChangeRadix(Radices),
    ShowSidebar,
    HideSidebar,
    ShowButton,
    HideButton,
}

#[derive(Debug)]
pub enum AppMode {
    SimpleView,
    ComponentView,
}

#[derive(Debug)]
enum CommandMsg {
    ThreadFinished(SingleCycleCPU),
}

#[relm4::component]
impl Component for App {
    type CommandOutput = CommandMsg;
    type Input = Msg;
    type Output = ();
    type Init = ();

    fn init(
        _: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let open_dialog = OpenDialog::builder()
            .transient_for_native(root)
            .launch(OpenDialogSettings::default())
            .forward(sender.input_sender(), |response| match response {
                OpenDialogResponse::Accept(path) => Msg::OpenResponse(path),
                OpenDialogResponse::Cancel => Msg::Ignore,
            });

        let simple_view = SimpleView::builder()
            .launch(())
            .forward(sender.input_sender(), |_| Msg::Ignore);

        let component_view = ComponentView::builder()
            .launch(())
            .forward(sender.input_sender(), |_| Msg::Ignore);

        let tag_table = gtk::TextTagTable::new();
        tag_table.add(
            &gtk::TextTag::builder()
                .name("line_highlight")
                .paragraph_background("yellow")
                .foreground("black")
                .build(),
        );

        let model = App {
            open_dialog,
            simple_view,
            component_view,
            mode: AppMode::SimpleView,
            history: History::new(),
            asm_view_buffer: gtk::TextBuffer::new(Some(&tag_table)),
            app_to_thread: None,
            cpu_running: false,
            sidebar_visible: false,
            sidebar_button_visible: true,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Msg, sender: ComponentSender<Self>, root: &Self::Root) {
        match msg {
            Msg::OpenRequest => self.open_dialog.emit(OpenDialogMsg::Open),
            Msg::OpenResponse(path) => match std::fs::read_to_string(path) {
                Ok(contents) => {
                    match parse(&contents) {
                        Ok((inst_mem, data_mem)) => {
                            self.history
                                .reset(SingleCycleCPU::new_from_memory(inst_mem, data_mem));
                            sender.input(Msg::UpdateViews);
                        }
                        Err(err) => sender.input(Msg::ShowMessage(err)),
                    };
                    self.asm_view_buffer.set_text(&contents);
                }
                Err(e) => sender.input(Msg::ShowMessage(e.to_string())),
            },
            Msg::Step => {
                let curr = self.history.get_curr().clone();
                self.history.append(curr);
                self.history.get_curr().step();
                if let Some(error) = self.history.get_curr().get_error() {
                    sender.input(Msg::ShowMessage(error));
                }
                sender.input(Msg::UpdateViews);
            }
            Msg::Run => {
                let (app_tx, thread_rx) = mpsc::channel::<()>();
                self.cpu_running = true;

                self.app_to_thread = Some(app_tx);

                let mut cpu_copy = self.history.get_curr().clone();
                sender.oneshot_command(async move {
                    while cpu_copy.get_error().is_none() {
                        cpu_copy.step();
                        if thread_rx.try_recv().is_ok() {
                            break;
                        }
                    }
                    CommandMsg::ThreadFinished(cpu_copy)
                });
            }
            Msg::Break => match &self.app_to_thread {
                Some(tx) => if tx.send(()).is_ok() {},
                None => {}
            },
            Msg::ShowMessage(message) => {
                let dialog = adw::MessageDialog::builder()
                    .transient_for(root)
                    .body(message)
                    .build();
                dialog.add_response("Ok", "Ok");
                dialog.set_default_response(Some("Ok"));
                dialog.connect_response(
                    None,
                    glib::clone!(@strong sender => move |dialog, _| {
                        dialog.close();
                    }),
                );
                dialog.present();
            }
            Msg::SetMode(mode) => self.mode = mode,
            Msg::ResetSimulation => {
                match parse(&self.asm_view_buffer.text(
                    &self.asm_view_buffer.start_iter(),
                    &self.asm_view_buffer.end_iter(),
                    true,
                )) {
                    Ok((inst_mem, data_mem)) => {
                        self.history
                            .reset(SingleCycleCPU::new_from_memory(inst_mem, data_mem));
                        sender.input(Msg::UpdateViews);
                    }
                    Err(err) => sender.input(Msg::ShowMessage(err)),
                };
            }
            Msg::UpdateViews => {
                self.simple_view
                    .emit(CPUViewMessage::Update(self.history.get_curr().clone()));
                self.component_view
                    .emit(CPUViewMessage::Update(self.history.get_curr().clone()));
            }
            Msg::ShowSidebar => self.sidebar_visible = true,
            Msg::HideSidebar => self.sidebar_visible = false,
            Msg::ShowButton => self.sidebar_button_visible = true,
            Msg::HideButton => self.sidebar_button_visible = false,
            Msg::ChangeRadix(radix) => {
                self.simple_view.emit(CPUViewMessage::ChangeRadix(radix));
                self.component_view.emit(CPUViewMessage::ChangeRadix(radix));
            }
            Msg::Undo => {
                self.history.undo();
                sender.input(Msg::UpdateViews);
            }
            Msg::Redo => {
                self.history.redo();
                sender.input(Msg::UpdateViews);
            }
            Msg::Ignore => {}
        }
    }

    fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        sender: ComponentSender<Self>,
        _: &Self::Root,
    ) {
        match message {
            CommandMsg::ThreadFinished(cpu) => {
                self.cpu_running = false;
                self.history.append(cpu.clone());
                sender.input(Msg::UpdateViews);
                if let Some(error) = self.history.get_curr().get_error() {
                    sender.input(Msg::ShowMessage(error));
                }
            }
        }
    }

    view! {
        adw::ApplicationWindow {
            set_size_request: (500, 500),
            set_default_size: (1000, 650),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_hexpand: true,

                adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::ViewSwitcher {
                            set_stack: Some(&stack),
                    },
                    #[name = "toggle_sidebar"]
                    pack_start = &gtk::ToggleButton {
                        #[watch]
                        set_visible: model.sidebar_button_visible,
                        #[watch]
                        set_active: model.sidebar_visible,
                        set_icon_name: icon_name::DOCK_LEFT,
                        connect_clicked[sender] => move |val| {
                            match val.is_active() {
                                true => sender.input(Msg::ShowSidebar),
                                false => sender.input(Msg::HideSidebar),
                            }
                        },
                    },
                    pack_end = &gtk::DropDown::from_strings(&["Hex", "Binary", "Decimal"]) {
                        connect_selected_item_notify[sender] => move |val| {
                            match val.selected() {
                                0 => sender.input(Msg::ChangeRadix(Radices::Hex)),
                                1 => sender.input(Msg::ChangeRadix(Radices::Binary)),
                                2 => sender.input(Msg::ChangeRadix(Radices::Decimal)),
                                _ => panic!("Invalid radix"),
                            }
                        },
                    },
                    pack_end = &gtk::Button {
                        #[watch]
                        set_sensitive: model.history.can_redo(),
                        set_icon_name: icon_name::STEP_OVER,
                        connect_clicked[sender] => move |_| { sender.input(Msg::Redo) },
                    },
                    pack_end = &gtk::ToggleButton {
                       #[watch]
                       set_active: model.cpu_running,
                       #[watch]
                       set_icon_name: if model.cpu_running { icon_name::PAUSE } else { icon_name::EXECUTE_FROM },
                       connect_clicked[sender] => move |val| {
                            match val.is_active() {
                                true => sender.input(Msg::Run),
                                false => sender.input(Msg::Break),
                            }
                       }
                    },
                    pack_end = &gtk::Button {
                        #[watch]
                        set_sensitive: model.history.can_undo(),
                        set_icon_name: icon_name::STEP_BACK,
                        connect_clicked[sender] => move |_| { sender.input(Msg::Undo) },
                    },
                },

                #[name = "split_view"]
                gtk::Box {
                    set_vexpand: true,
                    #[name = "flap"]
                    adw::Flap {
                        connect_reveal_flap_notify[sender] => move |val| {
                            match (val.reveals_flap(), val.is_folded()) {
                                (true, true) => sender.input(Msg::ShowSidebar),
                                (false, true) => sender.input(Msg::HideSidebar),
                                _ => {},
                            }
                        },
                        connect_folded_notify[sender] => move |val| {
                            match val.is_folded() {
                                true => sender.input(Msg::ShowButton),
                                false => sender.input(Msg::HideButton),
                            }
                        },
                        #[watch]
                        set_locked: model.sidebar_visible,
                        #[watch]
                        set_reveal_flap: model.sidebar_visible || !flap.is_folded(),
                        #[wrap(Some)]
                        set_flap = &gtk::ScrolledWindow {
                            set_width_request: 500,
                            gtk::TextView {
                                set_vexpand: true,
                                set_editable: false,
                                set_monospace: true,
                                set_cursor_visible: false,
                                set_buffer: Some(&model.asm_view_buffer),
                            },
                        },
                        #[wrap(Some)]
                        set_content = &gtk::Box {
                            set_width_request: 600,
                            #[name = "stack"]
                            adw::ViewStack {
                                set_vexpand: true,
                                add_titled[Some("Simple"), "Simple"] = model.simple_view.widget() {} -> {
                                    set_icon_name: Some(icon_name::TABLE),
                                },
                                add_titled[Some("Component"), "Component"] = model.component_view.widget() {} -> {
                                    set_icon_name: Some(icon_name::OBJECT_PACKING),
                                },
                            },
                        },
                    },
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 5,
                    set_margin_all: 5,
                    set_hexpand: true,

                    gtk::Button {
                        set_label: "Load File",
                        #[watch]
                        set_sensitive: !model.cpu_running,
                        connect_clicked => Msg::OpenRequest,
                    },

                    gtk::Button {
                        set_label: "Step",
                        #[watch]
                        set_sensitive: !model.cpu_running,
                        connect_clicked => Msg::Step,
                    },

                    gtk::Button {
                        set_label: "Reset",
                        #[watch]
                        set_sensitive: !model.cpu_running,
                        connect_clicked => Msg::ResetSimulation,
                    },
                }
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("org.simmips.gui");
    relm4_icons::initialize_icons();
    app.run::<App>(());
}
