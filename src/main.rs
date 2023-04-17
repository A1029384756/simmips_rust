mod column_views;
mod lex_parse;
mod utils;

use std::convert::identity;
use std::path::PathBuf;

use column_views::memory_view::{MemoryView, MemoryViewMsg};
use column_views::register_view::{RegViewMsg, RegisterView};
use gtk::prelude::*;
use lex_parse::lexer::tokenize;
use lex_parse::parser::parse_vm;
use lex_parse::virtual_machine_interface::VirtualMachineInterface;
use lex_parse::virtualmachine::VirtualMachine;
use num_traits::FromPrimitive;
use relm4::prelude::*;
use relm4_components::open_dialog::*;
use utils::highlight_line;

struct App {
    open_dialog: Controller<OpenDialog>,
    vm: VirtualMachine,
    asm_view_buffer: gtk::TextBuffer,
    register_view: Controller<RegisterView>,
    memory_view: Controller<MemoryView>,
    message: Option<String>,
}

#[derive(Debug)]
pub enum Msg {
    OpenRequest,
    OpenResponse(PathBuf),
    Ignore,
    Step,
    Run,
    ResetSimulation,
    ShowMessage(String),
    ResetMessage,
}

#[relm4::component]
impl SimpleComponent for App {
    type Input = Msg;
    type Output = ();
    type Init = ();

    fn post_view() {
        if let Some(text) = &model.message {
            let dialog = gtk::MessageDialog::builder()
                .text(text)
                .use_markup(true)
                .transient_for(&widgets.root)
                .modal(true)
                .buttons(gtk::ButtonsType::Ok)
                .build();
            dialog.connect_response(|dialog, _| dialog.destroy());
            dialog.show();
            sender.input(Msg::ResetMessage);
        }
    }

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

        let register_view: Controller<RegisterView> = RegisterView::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let memory_view: Controller<MemoryView> = MemoryView::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let highlight_tag = gtk::TextTag::new(Some("line_highlight"));
        highlight_tag.set_paragraph_background(Some("yellow"));
        highlight_tag.set_foreground(Some("black"));

        let tag_table = gtk::TextTagTable::new();
        tag_table.add(&highlight_tag);

        let model = App {
            open_dialog,
            vm: VirtualMachine::new(),
            asm_view_buffer: gtk::TextBuffer::new(Some(&tag_table)),
            register_view,
            memory_view,
            message: None,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            Msg::OpenRequest => self.open_dialog.emit(OpenDialogMsg::Open),
            Msg::OpenResponse(path) => match std::fs::read_to_string(&path) {
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
                            Err(e) => sender.input(Msg::ShowMessage(e)),
                        },
                        Err(e) => sender.input(Msg::ShowMessage(e)),
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
                for _ in 0..10000 {
                    if self.vm.is_error() {
                        break;
                    };
                    self.vm.step();
                }

                if self.vm.is_error() {
                    sender.input(Msg::ShowMessage(self.vm.get_error()));
                } else {
                    sender.input(Msg::ShowMessage(
                        "Ran 10000 iterations without error".to_string(),
                    ));
                }

                update_registers_and_mem(self);
                highlight_line(&mut self.asm_view_buffer, self.vm.get_current_source_line());
            }
            Msg::ShowMessage(message) => {
                self.message = Some(message);
            }
            Msg::ResetMessage => {
                self.message = None;
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
                        connect_clicked => Msg::OpenRequest,
                    },

                    gtk::Button {
                        set_label: "Step",
                        connect_clicked => Msg::Step,
                    },

                    gtk::Button {
                        set_label: "Run",
                        connect_clicked => Msg::Run,
                    },

                    gtk::Button {
                        set_label: "Reset",
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
    app.register_view.emit(RegViewMsg::UpdateRegisters(
        (0..35)
            .map(|idx| app.vm.get_register(FromPrimitive::from_i32(idx).unwrap()))
            .collect(),
    ));
    app.memory_view.emit(MemoryViewMsg::UpdateMemory(
        (0..app.vm.get_memory_size())
            .map(|idx| app.vm.get_memory_byte(idx).unwrap())
            .collect(),
    ));
}

fn main() {
    let app = RelmApp::new("org.simmips.gui");
    app.run::<App>(());
}
