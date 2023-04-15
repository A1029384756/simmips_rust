use relm4::gtk::traits::TextBufferExt;

use super::gtk::*;

pub fn highlight_line(text_buffer: &mut TextBuffer, line: u32) {
    let line_zero_indexed: i32 = line as i32 - 1;
    text_buffer.remove_all_tags(&text_buffer.start_iter(), &text_buffer.end_iter());

    match text_buffer.iter_at_line(line_zero_indexed) {
        Some(start) => {
            let mut end = start;
            end.forward_to_line_end();
            text_buffer.apply_tag_by_name("line_highlight", &start, &end);
        }
        None => {}
    }
}