use std::io;

use self::command::CommandMode;

mod command;
mod grid;
mod kube;
mod parser;
mod utils;

fn main() {
    let mut input_command = CommandMode::new_command();
    input_command.default_output_config_file_kube();
    let mut input_stream = String::new();
    loop {
        match io::stdin().read_line(&mut input_stream) {
            Ok(ok) => {
                println!("debug input: {}", input_stream);
                input_command.stream_command(input_stream.clone());
                // if input_stream == "\n" {
                //     utils::clear_screen();
                //     render_lists_kube_folders(data);
                // }
                // let input_index_int: usize = input_stream
                //     .trim()
                //     .parse()
                //     .expect(format!("Expect digit got : {:?}", input_stream).as_str());
                // if let Err(e) = parser::change_config_file_content(data, input_index_int) {
                //     eprintln!("Error change config file content, {}", e);
                // }
                // render_lists_kube_folders(data);
                input_stream.drain(..);
            }
            Err(e) => {
                eprintln!("Error read line input: {}", e);
                break;
            }
        }
    }
}

// fn render_lists_kube_folders(data: &kube::KubeStateData) {
//     let container_terminal = Window::new_window(150, 30);
//
//     let mut no_header = Window::new_header_column(String::from("No"), 3, data.files.len());
//     let mut path_header = Window::new_header_column(String::from("Path"), 100, data.files.len());
//
//     for (idx, val) in data.files.iter().enumerate() {
//         let number_list = idx + 1;
//         if let Some(path_str) = val.to_str() {
//             no_header.add_content_row(number_list.to_string());
//             path_header.add_content_row(path_str.to_string());
//         }
//     }
//
//     let render = Grid::new_render(container_terminal, vec![no_header, path_header]);
//     render.render_table_content();
//
//     let mut input_stream = String::new();
//     loop {
//         match io::stdin().read_line(&mut input_stream) {
//             Ok(ok) => {
//                 let input_command = CommandMode::new_command(&input_stream);
//                 if input_stream == "\n" {
//                     utils::clear_screen();
//                     render_lists_kube_folders(data);
//                 }
//                 let input_index_int: usize = input_stream
//                     .trim()
//                     .parse()
//                     .expect(format!("Expect digit got : {:?}", input_stream).as_str());
//                 if let Err(e) = parser::change_config_file_content(data, input_index_int) {
//                     eprintln!("Error change config file content, {}", e);
//                 }
//                 render_lists_kube_folders(data);
//             }
//             Err(e) => {
//                 eprintln!("Error read line input: {}", e);
//                 break;
//             }
//         }
//     }
// }
