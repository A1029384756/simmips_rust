use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::Sender;

use adw::prelude::*;
use mips_assembler::parse;
use relm4::prelude::*;
use relm4_components::open_dialog::{OpenDialog, OpenDialogMsg, OpenDialogResponse};
use relm4_icons::icon_name;

use super::{
    column_views::Radices, component_view::ComponentView, history::History,
    simple_view::SimpleView, CPUViewMessage,
};
use crate::cpu::{cpu_interface::CPUInterface, single_cycle_cpu::SingleCycleCPU};

#[derive(Debug, Clone)]
pub enum SimulationMsg {
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
    ChangeRadix(Radices),
    ResizeHistory(usize),
    ShowSidebar(bool),
}

#[derive(Debug)]
pub enum SimulationCmd {
    ThreadFinished(SingleCycleCPU),
}

#[derive(Debug)]
pub enum SimulationOutput {
    ShowMessage(String),
    ShowSidebarButton(bool),
    ShowSidebar(bool),
}

pub struct CPUSimulation {
    file_chooser: Controller<OpenDialog>,
    simple_view: Controller<SimpleView>,
    component_view: Controller<ComponentView>,
    history: History<SingleCycleCPU>,
    asm_view_buffer: gtk::TextBuffer,
    app_to_thread: Option<Sender<()>>,
    cpu_running: bool,
    sidebar_visible: bool,
    idx: usize,
}

#[relm4::factory(pub)]
impl FactoryComponent for CPUSimulation {
    type Init = usize;
    type Input = SimulationMsg;
    type Output = SimulationOutput;
    type CommandOutput = SimulationCmd;
    type ParentWidget = adw::TabView;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_vexpand: true,
            #[name = "flap"]
            adw::Flap {
                connect_reveal_flap_notify[sender] => move |val| {
                    match (val.reveals_flap(), val.is_folded()) {
                        (true, true) => sender.output(SimulationOutput::ShowSidebar(true)).unwrap(),
                        (false, true) => sender.output(SimulationOutput::ShowSidebar(false)).unwrap(),
                        _ => {},
                    }
                },
                connect_folded_notify[sender] => move |val| {
                    sender.output(SimulationOutput::ShowSidebarButton(val.is_folded())).unwrap()
                },
                #[watch]
                set_locked: self.sidebar_visible,
                #[watch]
                set_reveal_flap: self.sidebar_visible || !flap.is_folded(),
                #[wrap(Some)]
                set_flap = &gtk::ScrolledWindow {
                    set_width_request: 500,
                    gtk::TextView {
                        set_vexpand: true,
                        set_editable: false,
                        set_monospace: true,
                        set_cursor_visible: false,
                        set_buffer: Some(&self.asm_view_buffer),
                    },
                },
                #[wrap(Some)]
                set_content = &gtk::Box {
                    set_width_request: 600,
                    #[name = "stack"]
                    adw::ViewStack {
                        set_vexpand: true,
                        add_titled[Some("Simple"), "Simple"] = self.simple_view.widget() {} -> {
                            set_icon_name: Some(icon_name::TABLE),
                        },
                        add_titled[Some("Component"), "Component"] = self.component_view.widget() {} -> {
                            set_icon_name: Some(icon_name::PROCESSOR),
                        },
                   },
                },
            },
            adw::HeaderBar {
                set_show_end_title_buttons: false,

                #[wrap(Some)]
                set_title_widget = &adw::ViewSwitcher {
                    set_stack: Some(&stack),
                },

                pack_start = &gtk::Button {
                    #[watch]
                    set_sensitive: !self.cpu_running,
                    set_icon_name: icon_name::TEXT,
                    connect_clicked[sender] => move |_| { sender.input(SimulationMsg::OpenRequest) },
                    set_tooltip_text: Some("Load File"),
                },
                pack_start = &gtk::Button {
                    #[watch]
                    set_sensitive: !self.cpu_running,
                    set_icon_name: icon_name::REFRESH,
                    connect_clicked[sender] => move |_| { sender.input(SimulationMsg::ResetSimulation) },
                    set_tooltip_text: Some("Reset Simulation"),
                },

                pack_end = &gtk::Button {
                    #[watch]
                    set_sensitive: self.history.can_redo() && !self.cpu_running,
                    set_icon_name: icon_name::ARROW_REDO_FILLED,
                    connect_clicked[sender] => move |_| { sender.input(SimulationMsg::Redo) },
                    set_tooltip_text: Some("Redo"),
                },
                pack_end = &gtk::Button {
                    #[watch]
                    set_sensitive: !self.cpu_running,
                    set_icon_name: icon_name::ARROW_STEP_IN_RIGHT_FILLED,
                    connect_clicked[sender] => move |_| { sender.input(SimulationMsg::Step) },
                    set_tooltip_text: Some("Step Forward"),
                },
                pack_end = &gtk::ToggleButton {
                   #[watch]
                   set_active: self.cpu_running,
                   #[watch]
                   set_icon_name: if self.cpu_running { icon_name::PAUSE } else { icon_name::EXECUTE_FROM },
                   connect_clicked[sender] => move |val| {
                        match val.is_active() {
                            true => sender.input(SimulationMsg::Run),
                            false => sender.input(SimulationMsg::Break),
                        }
                   },
                   #[watch]
                   set_tooltip_text: if self.cpu_running { Some("Stop Simulation") } else { Some("Run Simulation") }
                },
                pack_end = &gtk::Button {
                    #[watch]
                    set_sensitive: self.history.can_undo() && !self.cpu_running,
                    set_icon_name: icon_name::ARROW_UNDO_FILLED,
                    connect_clicked[sender] => move |_| { sender.input(SimulationMsg::Undo) },
                    set_tooltip_text: Some("Undo"),
                },
            },
        },
        #[local_ref]
        returned_widget -> adw::TabPage {
            set_title: &format!("CPU {}", self.idx),
        }
    }

    fn init_model(count: Self::Init, _idx: &DynamicIndex, sender: FactorySender<Self>) -> Self {
        let file_chooser = OpenDialog::builder()
            .launch(relm4_components::open_dialog::OpenDialogSettings::default())
            .forward(sender.input_sender(), |response| match response {
                OpenDialogResponse::Accept(path) => SimulationMsg::OpenResponse(path),
                OpenDialogResponse::Cancel => SimulationMsg::Ignore,
            });

        let simple_view = SimpleView::builder()
            .launch(())
            .forward(sender.input_sender(), |_| SimulationMsg::Ignore);

        let component_view = ComponentView::builder()
            .launch(())
            .forward(sender.input_sender(), |_| SimulationMsg::Ignore);

        let tag_table = gtk::TextTagTable::new();
        tag_table.add(
            &gtk::TextTag::builder()
                .name("line_highlight")
                .paragraph_background("yellow")
                .foreground("black")
                .build(),
        );

        Self {
            file_chooser,
            simple_view,
            component_view,
            history: History::new(10),
            asm_view_buffer: gtk::TextBuffer::new(Some(&tag_table)),
            app_to_thread: None,
            cpu_running: false,
            sidebar_visible: false,
            idx: count,
        }
    }

    fn update(&mut self, msg: SimulationMsg, sender: FactorySender<Self>) {
        match msg {
            SimulationMsg::OpenRequest => self.file_chooser.emit(OpenDialogMsg::Open),
            SimulationMsg::OpenResponse(path) => match std::fs::read_to_string(path) {
                Ok(contents) => {
                    match parse(&contents) {
                        Ok((inst_mem, data_mem)) => {
                            self.history
                                .reset(SingleCycleCPU::new_from_memory(inst_mem, data_mem));
                            sender.input(SimulationMsg::UpdateViews);
                        }
                        Err(err) => sender.input(SimulationMsg::ShowMessage(err)),
                    };
                    self.asm_view_buffer.set_text(&contents);
                }
                Err(e) => sender.input(SimulationMsg::ShowMessage(e.to_string())),
            },
            SimulationMsg::Step => {
                let curr = self.history.get_curr().clone();
                self.history.append(curr);
                self.history.get_curr().step();
                if let Some(error) = self.history.get_curr().get_error() {
                    sender.input(SimulationMsg::ShowMessage(error));
                }
                sender.input(SimulationMsg::UpdateViews);
            }
            SimulationMsg::Run => {
                let (app_tx, thread_rx) = mpsc::channel::<()>();
                self.cpu_running = true;

                self.app_to_thread = Some(app_tx);

                let mut cpu_copy = self.history.get_curr().clone();
                sender.spawn_oneshot_command(move || {
                    while cpu_copy.get_error().is_none() {
                        cpu_copy.step();
                        if thread_rx.try_recv().is_ok() {
                            break;
                        }
                    }
                    SimulationCmd::ThreadFinished(cpu_copy)
                });
            }
            SimulationMsg::Break => match &self.app_to_thread {
                Some(tx) => if tx.send(()).is_ok() {},
                None => {}
            },
            SimulationMsg::ShowMessage(message) => sender
                .output(SimulationOutput::ShowMessage(message))
                .unwrap(),
            SimulationMsg::ResetSimulation => {
                match parse(&self.asm_view_buffer.text(
                    &self.asm_view_buffer.start_iter(),
                    &self.asm_view_buffer.end_iter(),
                    true,
                )) {
                    Ok((inst_mem, data_mem)) => {
                        self.history
                            .reset(SingleCycleCPU::new_from_memory(inst_mem, data_mem));
                        sender.input(SimulationMsg::UpdateViews);
                    }
                    Err(err) => sender.input(SimulationMsg::ShowMessage(err)),
                };
            }
            SimulationMsg::UpdateViews => {
                self.simple_view
                    .emit(CPUViewMessage::Update(self.history.get_curr().clone()));
                self.component_view
                    .emit(CPUViewMessage::Update(self.history.get_curr().clone()));
            }
            SimulationMsg::ResizeHistory(size) => self.history.resize(size),
            SimulationMsg::ShowSidebar(visible) => self.sidebar_visible = visible,
            SimulationMsg::ChangeRadix(radix) => {
                self.simple_view.emit(CPUViewMessage::ChangeRadix(radix));
                self.component_view.emit(CPUViewMessage::ChangeRadix(radix));
            }
            SimulationMsg::Undo => {
                self.history.undo();
                sender.input(SimulationMsg::UpdateViews);
            }
            SimulationMsg::Redo => {
                self.history.redo();
                sender.input(SimulationMsg::UpdateViews);
            }
            SimulationMsg::Ignore => {}
        }
    }

    fn update_cmd(&mut self, message: Self::CommandOutput, sender: FactorySender<Self>) {
        match message {
            SimulationCmd::ThreadFinished(cpu) => {
                self.cpu_running = false;
                self.history.append(cpu.clone());
                sender.input(SimulationMsg::UpdateViews);
                if let Some(error) = self.history.get_curr().get_error() {
                    sender.input(SimulationMsg::ShowMessage(error));
                }
            }
        }
    }
}
