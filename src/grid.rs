pub mod render;

pub struct Window {
    horizontal_width: usize,
    vertical_width: usize,
    row_content: Vec<String>,
    col_content: String,
}

pub struct Grid {
    main_window: Window,
    tab_windows: Vec<Window>,
}

impl Window {
    pub fn new_window(horizontal_width: usize, vertical_width: usize) -> Self {
        Window {
            horizontal_width,
            vertical_width,
            row_content: Vec::new(),
            col_content: String::new(),
        }
    }

    pub fn new_header_column(
        header: String,
        horizontal_width: usize,
        vertical_width: usize,
    ) -> Self {
        Window {
            horizontal_width,
            vertical_width,
            row_content: Vec::new(),
            col_content: header,
        }
    }

    pub fn add_content_row(&mut self, content: String) {
        self.row_content.push(content);
    }
}

impl Grid {
    pub fn new_render(main_window: Window, tab_windows: Vec<Window>) -> Self {
        Grid {
            main_window,
            tab_windows,
        }
    }

    pub fn render_table_content(&self) {
        let border_width = 1;
        let cols = self.tab_windows.len();

        let content_width = (self.main_window.horizontal_width - (cols + 1) * border_width) / cols;

        let header = self
            .tab_windows
            .iter()
            .map(|tab| {
                format!(
                    "{}{}",
                    "|".repeat(border_width),
                    format!("{:^width$}", tab.col_content, width = content_width)
                )
            })
            .collect::<String>();
        println!("{}{}", header, "|".repeat(border_width));

        let max_rows = self
            .tab_windows
            .iter()
            .map(|tab| tab.row_content.len())
            .max()
            .unwrap_or(0);

        for row_index in 0..max_rows {
            let row_content = self
                .tab_windows
                .iter()
                .map(|tab| {
                    let content = tab
                        .row_content
                        .get(row_index)
                        .unwrap_or(&"".to_string())
                        .to_string();
                    format!(
                        "{}{}",
                        "|".repeat(border_width),
                        format!("{:<width$}", content, width = content_width)
                    )
                })
                .collect::<String>();
            println!("{}{}", row_content, "|".repeat(border_width));
        }

        let horizontal_border = format!(
            "{}{}+",
            "+".repeat(border_width),
            "-".repeat(content_width * cols + (cols - 1) * border_width)
        );
        println!("{}", horizontal_border);
    }
}
