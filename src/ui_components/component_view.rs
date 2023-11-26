use crate::cpu::control_unit::{Mem, MemToReg, PCSrc, RegDst};
use crate::cpu::cpu_interface::CPUInterface;
use crate::cpu::single_cycle_cpu::SingleCycleCPU;

use super::CPUViewMessage;
use gdk_pixbuf::gio::MemoryInputStream;
use gdk_pixbuf::Pixbuf;
use gtk::prelude::*;
use rayon::prelude::*;
use relm4::drawing::DrawHandler;
use relm4::prelude::*;
use crate::main_separator;

pub struct ComponentView {
    handler: DrawHandler,
    imgs: Vec<Vec<u8>>,
    size: (i32, i32),
    cpu: SingleCycleCPU,
}

#[relm4::component(pub)]
impl SimpleComponent for ComponentView {
    type Input = CPUViewMessage;
    type Output = ();
    type Init = ();

    view! {
        #[root]
        gtk::Box {
            set_spacing: 5,
            set_margin_all: 5,
            set_baseline_position: gtk::BaselinePosition::Center,
            #[local_ref]
            area -> gtk::DrawingArea {
                set_vexpand: true,
                set_hexpand: true,
                connect_resize[sender] => move |_, x, y| {
                    sender.input(CPUViewMessage::Resize((x, y)))
                }
            },
        },
    }

    fn init(
        _: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let imgs = vec![
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "component_view.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "component_view.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "regdst_ra.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "regdst_rd.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "regdst_rt.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "alu_src.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "reg_write_memread.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "reg_write_pcinc.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "reg_write_aluresult.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "reg_write_imm.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "reg_write.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "mem_read_byte.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "mem_read_half.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "mem_read_word.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "mem_write_byte.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "mem_write_half.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "mem_write_word.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "pcsrc_branch.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "pcsrc_pc.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "pcsrc_jump.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "pcsrc_regjump.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "aluop_rtype.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "aluop_add.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "aluop_addu.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "aluop_sub.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "aluop_and.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "aluop_or.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "aluop_slt.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "aluop_sltu.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "beq.svg"
            ))),
            Vec::from(include_bytes!(concat!(
                "resources",
                main_separator!(),
                "bne.svg"
            ))),
        ];

        let model = ComponentView {
            handler: DrawHandler::new(),
            imgs,
            size: (0, 0),
            cpu: SingleCycleCPU::default(),
        };

        let area = model.handler.drawing_area();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            CPUViewMessage::Update(cpu) => {
                self.cpu = cpu;
                self.draw();
            }
            CPUViewMessage::Resize(size) => {
                self.size = size;
                self.draw();
            }
            CPUViewMessage::ChangeRadix(_) => {}
            CPUViewMessage::None => {}
        }
    }
}

impl ComponentView {
    fn draw(&mut self) {
        let cx = self.handler.get_context();
        cx.set_operator(gtk::cairo::Operator::Clear);
        cx.set_source_rgba(0.0, 0.0, 0.0, 0.0);
        cx.paint().expect("Couldn't fill context");
        cx.set_operator(gtk::cairo::Operator::Source);

        let mut drawn_images: Vec<usize> = vec![0];

        let signals = self.cpu.get_control_signals();
        match signals.reg_dst {
            RegDst::RA => drawn_images.push(1),
            RegDst::RD => drawn_images.push(2),
            RegDst::RT => drawn_images.push(3),
        }
        if signals.alu_src {
            drawn_images.push(4);
        }
        match signals.mem_to_reg {
            MemToReg::MemoryRead => drawn_images.push(5),
            MemToReg::PCInc => drawn_images.push(6),
            MemToReg::ALUResult => drawn_images.push(7),
            MemToReg::ImmLeftShift16 => drawn_images.push(8),
        }
        if signals.reg_write {
            drawn_images.push(9);
        }
        match signals.mem_read {
            Mem::None => {}
            Mem::Byte => drawn_images.push(10),
            Mem::Half => drawn_images.push(11),
            Mem::Word => drawn_images.push(12),
        }
        match signals.mem_write {
            Mem::None => {}
            Mem::Byte => drawn_images.push(13),
            Mem::Half => drawn_images.push(14),
            Mem::Word => drawn_images.push(15),
        }
        match signals.pc_src {
            PCSrc::PCBranch => drawn_images.push(16),
            PCSrc::PC => drawn_images.push(17),
            PCSrc::Jump => drawn_images.push(18),
            PCSrc::RegJump => drawn_images.push(19),
        }

        if self.size == (0, 0) {
            return;
        }

        let base_buf = Pixbuf::new(
            gdk_pixbuf::Colorspace::Rgb,
            true,
            8,
            self.size.0,
            self.size.1,
        )
        .unwrap();

        drawn_images.iter().for_each(|idx| {
            let stream =
                MemoryInputStream::from_bytes(&gdk_pixbuf::glib::Bytes::from(&self.imgs[*idx]));

            let buf = gdk_pixbuf::Pixbuf::from_stream_at_scale(
                &stream,
                self.size.0 - 10,
                self.size.1 - 10,
                true,
                None::<&gdk_pixbuf::gio::Cancellable>,
            )
            .unwrap();

            buf.composite(
                &base_buf,
                0,
                0,
                buf.width(),
                buf.height(),
                0.0,
                0.0,
                1.0,
                1.0,
                gdk_pixbuf::InterpType::Nearest,
                255,
            );
        });

        if !adw::StyleManager::default().is_dark() {
            let pixels = unsafe { base_buf.pixels() };
            pixels
                .par_chunks_mut(base_buf.n_channels() as usize)
                .for_each(|pixel| {
                    fn invert(c: u8) -> u8 {
                        (255 as i16 - c as i16) as u8
                    }

                    pixel[0] = invert(pixel[0]);
                    pixel[1] = invert(pixel[1]);
                    pixel[2] = invert(pixel[2]);
                });
        }

        cx.set_source_pixbuf(&base_buf, 0.0, 0.0);
        cx.paint().expect("Could not fill context");
    }
}
