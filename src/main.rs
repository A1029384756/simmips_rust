mod column_views;
mod info_dialog;
mod virtual_machine;
mod utils;

use std::convert::identity;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::Sender;

use info_dialog::*;

use column_views::memory_view::{MemoryMsg, MemoryView};
use column_views::register_view::{RegMsg, RegisterView};
use gtk::prelude::*;
use virtual_machine::lexer::tokenize;
use virtual_machine::parser::parse_vm;
use virtual_machine::virtual_machine_interface::VirtualMachineInterface;
use virtual_machine::virtualmachine::VirtualMachine;
use num_traits::FromPrimitive;
use relm4::prelude::*;
use relm4_components::open_dialog::*;
use utils::highlight_line;

struct App {
    open_dialog: Controller<OpenDialog>,
    info_dialog: Controller<InfoDialog>,
    vm: VirtualMachine,
    asm_view_buffer: gtk::TextBuffer,
    register_view: Controller<RegisterView>,
    memory_view: Controller<MemoryView>,
    app_to_thread: Option<Sender<()>>,
    vm_running: bool,
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
}

#[derive(Debug)]
enum CommandMsg {
    ThreadFinished(VirtualMachine),
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
            vm: VirtualMachine::new(),
            asm_view_buffer: gtk::TextBuffer::new(Some(&tag_table)),
            register_view,
            memory_view,
            app_to_thread: None,
            vm_running: false,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Msg, sender: ComponentSender<Self>, root: &Self::Root) {
        match msg {
            Msg::OpenRequest => self.open_dialog.emit(OpenDialogMsg::Open),
            Msg::OpenResponse(path) => match std::fs::read_to_string(path) {
                Ok(contents) => {
                    self.asm_view_buffer.set_text(&contents);

                    match tokenize(&contents) {
                        Ok(tokens) => match parse_vm(tokens) {
                            Ok(vm) => {
                                self.vm = vm;
                                update_registers_and_mem(self);
                                highlight_line(
                                    &mut self.asm_view_buffer,
                                    self.vm.get_current_source_line(),
                                );
                            }
                            Err(e) => {
                                sender.input(Msg::ShowMessage(e));
                                self.vm = VirtualMachine::new();
                            }
                        },
                        Err(e) => {
                            sender.input(Msg::ShowMessage(e));
                            self.vm = VirtualMachine::new();
                        }
                    }
                }
                Err(e) => sender.input(Msg::ShowMessage(e.to_string())),
            },
            Msg::Step => {
                self.vm.step();
                update_registers_and_mem(self);
                highlight_line(&mut self.asm_view_buffer, self.vm.get_current_source_line());
                if self.vm.is_error() {
                    sender.input(Msg::ShowMessage(self.vm.get_error()));
                }
            }
            Msg::Run => {
                let (app_tx, thread_rx) = mpsc::channel::<()>();
                self.vm_running = true;

                self.app_to_thread = Some(app_tx);
                let mut thread_vm = self.vm.clone();
                sender.oneshot_command(async move {
                    while !thread_vm.is_error() {
                        thread_vm.step();
                        if thread_rx.try_recv().is_ok() {
                            break;
                        }
                    }
                    CommandMsg::ThreadFinished(thread_vm)
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
            Msg::ResetSimulation => {
                let contents = self
                    .asm_view_buffer
                    .text(
                        &self.asm_view_buffer.start_iter(),
                        &self.asm_view_buffer.end_iter(),
                        true,
                    )
                    .to_string();

                match tokenize(&contents) {
                    Ok(tokens) => match parse_vm(tokens) {
                        Ok(vm) => {
                            self.vm = vm;
                            update_registers_and_mem(self);
                            highlight_line(
                                &mut self.asm_view_buffer,
                                self.vm.get_current_source_line(),
                            );
                        }
                        Err(e) => sender.input(Msg::ShowMessage(e)),
                    },
                    Err(e) => sender.input(Msg::ShowMessage(e)),
                }
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
            CommandMsg::ThreadFinished(new_vm) => {
                self.vm_running = false;
                self.vm = new_vm;
                update_registers_and_mem(self);
                highlight_line(&mut self.asm_view_buffer, self.vm.get_current_source_line());
                if self.vm.is_error() {
                    sender.input(Msg::ShowMessage(self.vm.get_error()));
                }
            }
        }
    }

    view! {
        root = gtk::Window {
            set_title: Some("MIPS Simulator"),
            set_default_size: (800, 600),

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
                        set_sensitive: !model.vm_running,
                        connect_clicked => Msg::OpenRequest,
                    },

                    gtk::Button {
                        set_label: "Step",
                        #[watch]
                        set_sensitive: !model.vm_running,
                        connect_clicked => Msg::Step,
                    },

                    gtk::Button {
                        set_label: "Run",
                        #[watch]
                        set_sensitive: !model.vm_running,
                        connect_clicked => Msg::Run,
                    },

                    gtk::Button {
                        set_label: "Break",
                        #[watch]
                        set_sensitive: model.vm_running,
                        connect_clicked => Msg::Break,
                    },

                    gtk::Button {
                        set_label: "Reset",
                        #[watch]
                        set_sensitive: !model.vm_running,
                        connect_clicked => Msg::ResetSimulation,
                    },

                    gtk::Label {
                        #[watch]
                        set_label: &format!("Current Line: {}", model.vm.get_current_source_line()),
                        set_margin_all: 5,
                    },
                }
            }
        }
    }
}

fn update_registers_and_mem(app: &mut App) {
    app.register_view.emit(RegMsg::UpdateRegisters(
        (0..35)
            .map(|idx| app.vm.get_register(FromPrimitive::from_i32(idx).unwrap()))
            .collect(),
    ));
    app.memory_view.emit(MemoryMsg::UpdateMemory(
        (0..app.vm.get_memory_size())
            .map(|idx| app.vm.get_memory_byte(idx).unwrap())
            .collect(),
    ));
}

fn main() {
    let app = RelmApp::new("org.simmips.gui");
    app.run::<App>(());
}
