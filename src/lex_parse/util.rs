use crate::lex_parse::virtual_machine_interface::RegisterKind;

pub fn get_valid_register(reg: &str) -> Option<RegisterKind> {
    match reg {
        "$0" => Some(RegisterKind::REG00),
        "$1" => Some(RegisterKind::REG01),
        "$2" => Some(RegisterKind::REG02),
        "$3" => Some(RegisterKind::REG03),
        "$4" => Some(RegisterKind::REG04),
        "$5" => Some(RegisterKind::REG05),
        "$6" => Some(RegisterKind::REG06),
        "$7" => Some(RegisterKind::REG07),
        "$8" => Some(RegisterKind::REG08),
        "$9" => Some(RegisterKind::REG09),
        "$10" => Some(RegisterKind::REG10),
        "$11" => Some(RegisterKind::REG11),
        "$12" => Some(RegisterKind::REG12),
        "$13" => Some(RegisterKind::REG13),
        "$14" => Some(RegisterKind::REG14),
        "$15" => Some(RegisterKind::REG15),
        "$16" => Some(RegisterKind::REG16),
        "$17" => Some(RegisterKind::REG17),
        "$18" => Some(RegisterKind::REG18),
        "$19" => Some(RegisterKind::REG19),
        "$20" => Some(RegisterKind::REG20),
        "$21" => Some(RegisterKind::REG21),
        "$22" => Some(RegisterKind::REG22),
        "$23" => Some(RegisterKind::REG23),
        "$24" => Some(RegisterKind::REG24),
        "$25" => Some(RegisterKind::REG25),
        "$26" => Some(RegisterKind::REG26),
        "$27" => Some(RegisterKind::REG27),
        "$28" => Some(RegisterKind::REG28),
        "$29" => Some(RegisterKind::REG29),
        "$30" => Some(RegisterKind::REG30),
        "$31" => Some(RegisterKind::REG31),
        "$zero" => Some(RegisterKind::REG00),
        "$at" => Some(RegisterKind::REG01),
        "$v0" => Some(RegisterKind::REG02),
        "$v1" => Some(RegisterKind::REG03),
        "$a0" => Some(RegisterKind::REG04),
        "$a1" => Some(RegisterKind::REG05),
        "$a2" => Some(RegisterKind::REG06),
        "$a3" => Some(RegisterKind::REG07),
        "$t0" => Some(RegisterKind::REG08),
        "$t1" => Some(RegisterKind::REG09),
        "$t2" => Some(RegisterKind::REG10),
        "$t3" => Some(RegisterKind::REG11),
        "$t4" => Some(RegisterKind::REG12),
        "$t5" => Some(RegisterKind::REG13),
        "$t6" => Some(RegisterKind::REG14),
        "$t7" => Some(RegisterKind::REG15),
        "$s0" => Some(RegisterKind::REG16),
        "$s1" => Some(RegisterKind::REG17),
        "$s2" => Some(RegisterKind::REG18),
        "$s3" => Some(RegisterKind::REG19),
        "$s4" => Some(RegisterKind::REG20),
        "$s5" => Some(RegisterKind::REG21),
        "$s6" => Some(RegisterKind::REG22),
        "$s7" => Some(RegisterKind::REG23),
        "$t8" => Some(RegisterKind::REG24),
        "$t9" => Some(RegisterKind::REG25),
        "$k0" => Some(RegisterKind::REG26),
        "$k1" => Some(RegisterKind::REG27),
        "$gp" => Some(RegisterKind::REG28),
        "$sp" => Some(RegisterKind::REG29),
        "$fp" => Some(RegisterKind::REG30),
        "$ra" => Some(RegisterKind::REG31),
        "$pc" => Some(RegisterKind::REGPC),
        "$hi" => Some(RegisterKind::REGHI),
        "$lo" => Some(RegisterKind::REGLO),
        _ => None,
    }
}
