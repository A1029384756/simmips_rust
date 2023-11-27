use adw::prelude::*;
use gtk::glib::clone;
use relm4::prelude::*;
use relm4_icons::icon_name;
use sourceview5::prelude::*;

#[derive(Debug)]
pub struct AsmView {
    asm_buffer: sourceview5::Buffer,
    assembled_buffer: sourceview5::Buffer,
    curr_line: u32,
    dirty: bool,
    can_save: bool,
}

#[derive(Debug)]
pub enum AsmViewMsg {
    LoadFile(String, Vec<u32>),
    SetLine(u32),
    UpdateTheme,
    SaveFile,
    SetDirty(bool),
    SetCanSave(bool),
}

#[derive(Debug)]
pub enum AsmViewOutput {
    SaveFile(String),
}

#[relm4::component(pub)]
impl SimpleComponent for AsmView {
    type Init = ();
    type Input = AsmViewMsg;
    type Output = AsmViewOutput;

    view! {
        #[root]
        gtk::Box {
            inline_css: "background: @window_bg_color",
            set_orientation: gtk::Orientation::Vertical,
            gtk::Overlay {
                gtk::ScrolledWindow {
                    set_width_request: 500,
                    sourceview5::View {
                        set_show_line_numbers: true,
                        set_margin_all: 5,
                        set_vexpand: true,
                        set_monospace: true,
                        set_buffer: Some(&model.asm_buffer),
                    },
                },
                add_overlay = &gtk::Box {
                    set_halign: gtk::Align::End,
                    set_valign: gtk::Align::End,
                    set_margin_all: 10,
                    gtk::Button {
                        set_icon_name: icon_name::FLOPPY,
                        set_tooltip_text: Some("Save"),
                        #[watch]
                        set_visible: model.dirty,
                        #[watch]
                        set_sensitive: model.can_save,
                        connect_clicked => AsmViewMsg::SaveFile,
                    },
                },
            },
            gtk::ScrolledWindow {
                set_width_request: 500,
                sourceview5::View {
                    set_show_line_numbers: true,
                    set_margin_all: 5,
                    set_vexpand: true,
                    set_editable: false,
                    set_monospace: true,
                    set_cursor_visible: false,
                    set_buffer: Some(&model.assembled_buffer),
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let tag_table = gtk::TextTagTable::new();
        tag_table.add(
            &gtk::TextTag::builder()
                .name("line_highlight")
                .paragraph_background("yellow")
                .foreground("black")
                .build(),
        );

        let asm_buffer = sourceview5::Buffer::new(None);
        let assembled_buffer = sourceview5::Buffer::new(Some(&tag_table));

        asm_buffer.set_highlight_syntax(true);

        asm_buffer.set_implicit_trailing_newline(false);
        assembled_buffer.set_implicit_trailing_newline(false);

        if let Some(ref language) = sourceview5::LanguageManager::new().language("ini") {
            asm_buffer.set_language(Some(language));
        }

        asm_buffer.connect_modified_changed(clone!(@strong sender => move |val| {
            sender.input(AsmViewMsg::SetDirty(val.is_modified()));
        }));

        let mut model = Self {
            asm_buffer,
            assembled_buffer,
            curr_line: 0,
            dirty: false,
            can_save: true,
        };

        model.set_theme_dark(adw::StyleManager::default().is_dark());

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AsmViewMsg::LoadFile(asm, binary) => {
                self.asm_buffer.set_text(&asm);
                self.asm_buffer.set_modified(false);
                if let Some(bin) = binary
                    .iter()
                    .map(|e| format!("{e:08x}\n"))
                    .reduce(|acc, e| acc + &e)
                {
                    self.assembled_buffer.set_text(bin.trim());
                }
            }
            AsmViewMsg::SetLine(pc) => {
                self.curr_line = (pc - 0x00400000) >> 2;
                self.highlight_assembly();
            }
            AsmViewMsg::UpdateTheme => self.set_theme_dark(adw::StyleManager::default().is_dark()),
            AsmViewMsg::SetDirty(dirty) => self.dirty = dirty,
            AsmViewMsg::SetCanSave(can_save) => self.can_save = can_save,
            AsmViewMsg::SaveFile => sender
                .output(AsmViewOutput::SaveFile(
                    self.asm_buffer
                        .text(
                            &self.asm_buffer.start_iter(),
                            &self.asm_buffer.end_iter(),
                            true,
                        )
                        .to_string(),
                ))
                .unwrap(),
        }
    }
}

impl AsmView {
    fn set_theme_dark(&mut self, is_dark: bool) {
        let theme = if is_dark { "Adwaita-dark" } else { "Adwaita" };
        if let Some(ref scheme) = sourceview5::StyleSchemeManager::new().scheme(theme) {
            self.asm_buffer.set_style_scheme(Some(scheme));
            self.assembled_buffer.set_style_scheme(Some(scheme));
        }
    }

    fn highlight_assembly(&mut self) {
        self.assembled_buffer.remove_all_tags(
            &self.assembled_buffer.start_iter(),
            &self.assembled_buffer.end_iter(),
        );

        if let Some(start) = self.assembled_buffer.iter_at_line(self.curr_line as i32) {
            let mut end = start;
            end.forward_to_line_end();
            self.assembled_buffer
                .apply_tag_by_name("line_highlight", &start, &end);
        }
    }
}
