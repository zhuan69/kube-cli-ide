use std::collections::HashMap;

use crate::grid::render::RenderContentData;
use crate::kube::command::{KubeCommandData, KubeCommandMode, KubeKind};
use crate::kube::spec::KubeListData;
use crate::kube::KubeStateData;
use crate::{kube, parser, utils};

#[derive(Debug)]
pub struct CommandMode {
    state_mode: KubeCommandMode,
    kind_state: KubeKind,
    namespace: String,
}

impl CommandMode {
    pub fn new_command() -> Self {
        CommandMode {
            state_mode: KubeCommandMode::ConfigMode,
            kind_state: KubeKind::Empty,
            namespace: String::new(),
        }
    }

    fn change_mode(&mut self, input_mode: &String) {
        let (state_mode, kind_state) = match input_mode.as_str().trim() {
            "n" => (KubeCommandMode::NamespaceMode, KubeKind::NamspaceKind),
            "" => (KubeCommandMode::ClearScreenMode, KubeKind::Empty),
            _ => (KubeCommandMode::ConfigMode, KubeKind::Empty),
        };
        self.state_mode = state_mode;
        self.kind_state = kind_state;
    }

    pub fn stream_command(&mut self, input_key: String) {
        self.change_mode(&input_key);
        println!("mode: {:?}", self.state_mode);
        match self.state_mode {
            KubeCommandMode::NamespaceMode => {
                let kube_command = KubeCommandData::new_kube_command(
                    KubeCommandMode::ListMode,
                    String::new(),
                    KubeKind::NamspaceKind,
                    String::new(),
                );
                let kube_output = kube_command.exec_command();
                if let Some(kube_list_data) =
                    KubeListData::new_list_api_data_from_string(kube_output.as_str())
                {
                    self.output_namespace_lists(&kube_list_data);
                }
            }
            KubeCommandMode::ListMode => {
                println!("get pod list");
            }
            KubeCommandMode::ClearScreenMode => {
                utils::clear_screen();
                self.default_output_config_file_kube();
            }
            _ => {
                let input_index: usize = input_key
                    .trim()
                    .parse()
                    .expect(format!("Expect digit got : {:?}", input_key).as_str());
                let data = self.default_output_config_file_kube();
                if let Err(e) = parser::change_config_file_content(&data, input_index) {
                    eprintln!("Error change config file content, {}", e);
                }
            }
        }
    }

    fn output_namespace_lists(&self, data: &KubeListData) {
        let mut no_contents: Vec<String> = Vec::with_capacity(data.items.len());
        let mut name_contents: Vec<String> = Vec::with_capacity(data.items.len());
        let mut status_contents: Vec<String> = Vec::with_capacity(data.items.len());

        for (idx, val) in data.items.iter().enumerate() {
            no_contents.push(idx.to_string());
            name_contents.push(val.metadata.name.clone());
            if let Some(status) = val.status.get(&"phase".to_string()) {
                status_contents.push(status.to_string());
            }
        }
        let contents: HashMap<String, Vec<String>> = HashMap::from([
            ("No".to_string(), no_contents),
            ("Name".to_string(), name_contents),
            ("Status".to_string(), status_contents),
        ]);
        RenderContentData::new_render_content(150, 10, contents).render_content();
    }

    pub fn default_output_config_file_kube(&self) -> KubeStateData {
        let mut data = kube::KubeStateData::new_kube_data();
        data.walking_dir_kube();
        let mut no_contents: Vec<String> = Vec::with_capacity(data.files.len());
        let mut config_contents: Vec<String> = Vec::with_capacity(data.files.len());
        for (idx, val) in data.files.iter().enumerate() {
            let number_list = idx + 1;
            no_contents.push(number_list.to_string());
            if let Some(path_str) = val.to_str() {
                config_contents.push(path_str.to_string());
            }
        }
        let contents: HashMap<String, Vec<String>> = HashMap::from([
            ("No".to_string(), no_contents),
            ("Config-File".to_string(), config_contents),
        ]);
        let mut render_content = RenderContentData::new_render_content(150, 30, contents);
        render_content.render_content();
        data
    }
}
