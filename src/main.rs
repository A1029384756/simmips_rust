mod cpu;
mod ui_components;
mod utils;

use std::convert::identity;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};

use mips_assembler::parse;

use cpu::cpu_interface::CPUInterface;
use cpu::single_cycle_cpu::SingleCycleCPU;
use gtk::prelude::*;
use num_traits::FromPrimitive;
use relm4::prelude::*;
use relm4_components::open_dialog::*;

use ui_components::column_views::memory_view::{MemoryMsg, MemoryView};
use ui_components::column_views::register_view::{RegMsg, RegisterView};
use ui_components::header::{HeaderMsg, HeaderView};
use ui_components::info_dialog::{DialogMsg, InfoDialog};

struct App {
    open_dialog: Controller<OpenDialog>,
    info_dialog: Controller<InfoDialog>,
    header: Controller<HeaderView>,
    register_view: Controller<RegisterView>,
    memory_view: Controller<MemoryView>,

    mode: AppMode,
    cpu: Arc<Mutex<dyn CPUInterface>>,
    asm_view_buffer: gtk::TextBuffer,
    app_to_thread: Option<Sender<()>>,
    cpu_unning: bool,
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

        let info_dialog = InfoDialog::builder()
            .transient_for(root)
            .launch(())
            .forward(sender.input_sender(), |_| Msg::Ignore);

        let header =
            HeaderView::builder()
                .launch(())
                .forward(sender.input_sender(), |msg| match msg {
                    HeaderMsg::SimpleView => Msg::SetMode(AppMode::SimpleView),
                    HeaderMsg::ComponentView => Msg::SetMode(AppMode::ComponentView),
                });

        let register_view: Controller<RegisterView> = RegisterView::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let memory_view: Controller<MemoryView> = MemoryView::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

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
            info_dialog,
            header,
            mode: AppMode::SimpleView,
            cpu: Arc::new(Mutex::new(SingleCycleCPU::new())),
            asm_view_buffer: gtk::TextBuffer::new(Some(&tag_table)),
            register_view,
            memory_view,
            app_to_thread: None,
            cpu_unning: false,
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
                            self.cpu = Arc::new(Mutex::new(SingleCycleCPU::new_from_memory(
                                inst_mem, data_mem,
                            )));
                            self.update_registers_and_mem();
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
                self.update_registers_and_mem();
            }
            Msg::Run => {
                let (app_tx, thread_rx) = mpsc::channel::<()>();
                self.cpu_unning = true;

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
                self.info_dialog = InfoDialog::builder()
                    .transient_for(root)
                    .launch(())
                    .forward(sender.input_sender(), |_| Msg::Ignore);

                self.info_dialog.emit(DialogMsg::Show(message));
            }
            Msg::SetMode(mode) => self.mode = mode,
            Msg::ResetSimulation => {}
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
                self.cpu_unning = false;
                self.update_registers_and_mem();
                if let Ok(cpu) = self.cpu.lock() {
                    if let Some(error) = cpu.get_error() {
                        sender.input(Msg::ShowMessage(error));
                    }
                }
            }
        }
    }

    view! {
        root = gtk::Window {
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

                    append: model.register_view.widget(),
                    append: model.memory_view.widget(),
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 5,
                    set_margin_all: 5,
                    set_hexpand: true,

                    gtk::Button {
                        set_label: "Load File",
                        #[watch]
                        set_sensitive: !model.cpu_unning,
                        connect_clicked => Msg::OpenRequest,
                    },

                    gtk::Button {
                        set_label: "Step",
                        #[watch]
                        set_sensitive: !model.cpu_unning,
                        connect_clicked => Msg::Step,
                    },

                    gtk::Button {
                        set_label: "Run",
                        #[watch]
                        set_sensitive: !model.cpu_unning,
                        connect_clicked => Msg::Run,
                    },

                    gtk::Button {
                        set_label: "Break",
                        #[watch]
                        set_sensitive: model.cpu_unning,
                        connect_clicked => Msg::Break,
                    },

                    gtk::Button {
                        set_label: "Reset",
                        #[watch]
                        set_sensitive: !model.cpu_unning,
                        connect_clicked => Msg::ResetSimulation,
                    },
                }
            }
        }
    }
}

impl App {
    fn update_registers_and_mem(&mut self) {
        if let Ok(cpu) = self.cpu.lock() {
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

fn main() {
    let app = RelmApp::new("org.simmips.gui");
    app.run::<App>(());
}
