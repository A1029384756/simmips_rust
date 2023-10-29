mod cpu;
mod ui_components;
mod utils;

use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};

use mips_assembler::parse;

use cpu::cpu_interface::CPUInterface;
use cpu::single_cycle_cpu::SingleCycleCPU;
use relm4::adw::traits::*;
use relm4::gtk::glib::clone;
use relm4::gtk::traits::*;
use relm4::prelude::*;
use relm4_components::open_dialog::*;

use ui_components::CPUViewMessage;
use ui_components::component_view::ComponentView;
use ui_components::header::{HeaderMsg, HeaderView};
use ui_components::simple_view::SimpleView;

struct App {
    open_dialog: Controller<OpenDialog>,
    header: Controller<HeaderView>,
    simple_view: Controller<SimpleView>,
    component_view: Controller<ComponentView>,

    mode: AppMode,
    cpu: Arc<Mutex<dyn CPUInterface>>,
    asm_view_buffer: gtk::TextBuffer,
    app_to_thread: Option<Sender<()>>,
    cpu_running: bool,
}

#[derive(Debug)]
pub enum Msg {
    OpenRequest,
    OpenResponse(PathBuf),
    Ignore,
    Step,
    Run,
    Break,
    ResetSimulation,
    UpdateViews,
    ShowMessage(String),
    SetMode(AppMode),
}

#[derive(Debug)]
pub enum AppMode {
    SimpleView,
    ComponentView,
}

#[derive(Debug)]
enum CommandMsg {
    ThreadFinished,
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

        let header =
            HeaderView::builder()
                .launch(())
                .forward(sender.input_sender(), |msg| match msg {
                    HeaderMsg::SimpleView => Msg::SetMode(AppMode::SimpleView),
                    HeaderMsg::ComponentView => Msg::SetMode(AppMode::ComponentView),
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
            header,
            mode: AppMode::SimpleView,
            cpu: Arc::new(Mutex::new(SingleCycleCPU::new())),
            asm_view_buffer: gtk::TextBuffer::new(Some(&tag_table)),
            app_to_thread: None,
            cpu_running: false,
        };
        let simple_widget = model.simple_view.widget();
        let component_widget = model.component_view.widget();

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
                            self.cpu = Arc::new(Mutex::new(SingleCycleCPU::new_from_memory(
                                inst_mem, data_mem,
                            )));
                            sender.input(Msg::UpdateViews);
                        }
                        Err(err) => sender.input(Msg::ShowMessage(err)),
                    };
                    self.asm_view_buffer.set_text(&contents);
                }
                Err(e) => sender.input(Msg::ShowMessage(e.to_string())),
            },
            Msg::Step => {
                if let Ok(mut cpu) = self.cpu.lock() {
                    cpu.step();
                    if let Some(error) = cpu.get_error() {
                        sender.input(Msg::ShowMessage(error));
                    }
                }
                sender.input(Msg::UpdateViews);
            }
            Msg::Run => {
                let (app_tx, thread_rx) = mpsc::channel::<()>();
                self.cpu_running = true;

                self.app_to_thread = Some(app_tx);
                let cpu_copy = self.cpu.clone();
                sender.oneshot_command(async move {
                    if let Ok(mut cpu) = cpu_copy.lock() {
                        while let None = cpu.get_error() {
                            cpu.step();
                            if thread_rx.try_recv().is_ok() {
                                break;
                            }
                        }
                    }
                    CommandMsg::ThreadFinished
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
                    clone!(@strong sender => move |dialog, _| {
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
                        self.cpu = Arc::new(Mutex::new(SingleCycleCPU::new_from_memory(
                            inst_mem, data_mem,
                        )));
                        sender.input(Msg::UpdateViews);
                    }
                    Err(err) => sender.input(Msg::ShowMessage(err)),
                };
            }
            Msg::UpdateViews => {
                self.simple_view.emit(CPUViewMessage::Update(self.cpu.clone()));
                self.component_view.emit(CPUViewMessage::Update(self.cpu.clone()));
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
            CommandMsg::ThreadFinished => {
                self.cpu_running = false;
                sender.input(Msg::UpdateViews);
                if let Ok(cpu) = self.cpu.lock() {
                    if let Some(error) = cpu.get_error() {
                        sender.input(Msg::ShowMessage(error));
                    }
                }
            }
        }
    }

    view! {
        gtk::Window {
            set_title: Some("MIPS Simulator"),
            set_default_size: (800, 600),
            set_titlebar: Some(model.header.widget()),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 5,
                    set_margin_all: 5,

                    gtk::ScrolledWindow {
                        set_min_content_height: 400,

                        #[wrap(Some)]
                        set_child = &gtk::TextView {
                            set_hexpand: true,
                            set_vexpand: true,
                            set_margin_all: 5,
                            set_editable: false,
                            set_monospace: true,
                            set_cursor_visible: false,
                            set_buffer: Some(&model.asm_view_buffer),
                        },
                    },

                    #[transition = "SlideLeftRight"]
                    match model.mode {
                        AppMode::SimpleView => gtk::Box {
                            #[local_ref]
                            simple_widget -> gtk::Box {},
                        }
                        AppMode::ComponentView => gtk::Box {
                            #[local_ref]
                            component_widget -> gtk::Box {},
                        }
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
                        set_label: "Run",
                        #[watch]
                        set_sensitive: !model.cpu_running,
                        connect_clicked => Msg::Run,
                    },

                    gtk::Button {
                        set_label: "Break",
                        #[watch]
                        set_sensitive: model.cpu_running,
                        connect_clicked => Msg::Break,
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
    app.run::<App>(());
}
