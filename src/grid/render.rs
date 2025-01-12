use std::collections::HashMap;

use super::{Grid, Window};

pub struct RenderContentData {
    horizontal_width: usize,
    vertical_width: usize,
    contents: HashMap<String, Vec<String>>,
}

impl RenderContentData {
    pub fn new_render_content(
        horizontal_width: usize,
        vertical_width: usize,
        contents: HashMap<String, Vec<String>>,
    ) -> Self {
        RenderContentData {
            horizontal_width,
            vertical_width,
            contents,
        }
    }

    pub fn render_content(&mut self) {
        let main_container = Window::new_window(self.horizontal_width, self.vertical_width);
        let evenly_horizontal_devide = self.horizontal_width / self.contents.len();
        let mut headers: Vec<Window> = Vec::new();
        for (k, v) in self.contents.drain() {
            let mut header =
                Window::new_header_column(k.to_string(), evenly_horizontal_devide, v.len());
            for val in v {
                header.add_content_row(val.to_string());
            }
            headers.push(header);
        }
        let render_grid = Grid::new_render(main_container, headers);
        render_grid.render_table_content();
    }
}
