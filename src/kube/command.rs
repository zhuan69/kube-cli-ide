use std::collections::HashMap;

use crate::utils;

#[derive(PartialEq, Hash, Eq, Debug)]
pub enum KubeCommandMode {
    ConfigMode,
    NamespaceMode,
    ListMode,
    LogKubeMode,
    DeleteMode,
    EditMode,
    ClearScreenMode,
    ExecMode,
    RestartMode,
}

#[derive(Debug)]
pub enum KubeKind {
    Empty,
    NamspaceKind,
    DeploymentKind,
    ServiceKind,
    PodKind,
    PvcKind,
    PvKind,
    IngressKind,
}

pub struct KubeCommandData {
    mode: KubeCommandMode,
    namespace: String,
    kind: String,
    command_mode: HashMap<KubeCommandMode, String>,
    instance_name: String,
}

impl KubeCommandData {
    pub fn new_kube_command(
        mode: KubeCommandMode,
        namespace: String,
        kind: KubeKind,
        instance_name: String,
    ) -> Self {
        let kind_value: String = match kind {
            KubeKind::DeploymentKind => String::from("deployment"),
            KubeKind::ServiceKind => String::from("service"),
            KubeKind::PodKind => String::from("pod"),
            KubeKind::PvcKind => String::from("pvc"),
            KubeKind::PvKind => String::from("pv"),
            KubeKind::IngressKind => String::from("ingress"),
            KubeKind::NamspaceKind => String::from("namespace"),
            KubeKind::Empty => String::new(),
        };
        KubeCommandData {
            mode,
            namespace,
            kind: kind_value,
            instance_name,
            command_mode: HashMap::from([
                (KubeCommandMode::ListMode, String::from("get")),
                (KubeCommandMode::EditMode, String::from("edit")),
                (
                    KubeCommandMode::RestartMode,
                    String::from("rollout restart"),
                ),
                (KubeCommandMode::DeleteMode, String::from("delete")),
                (KubeCommandMode::ExecMode, String::from("exec")),
                (KubeCommandMode::LogKubeMode, String::from("logs")),
            ]),
        }
    }

    pub fn exec_command(&self) -> String {
        let mut args: Vec<String> = vec![self.get_mode().to_string()];
        match self.mode {
            KubeCommandMode::LogKubeMode => {
                args.push("-f".to_string());
            }
            KubeCommandMode::ExecMode => {
                args.push("-it".to_string());
            }
            _ => {
                args.push(self.kind.clone());
                if !self.instance_name.is_empty() {
                    args.push(self.instance_name.clone());
                }
            }
        }
        if !self.namespace.is_empty() {
            args.push("-n".to_string());
            args.push(self.namespace.clone());
        }
        args.push("-o".to_string());
        args.push("json".to_string());
        println!("running command with args: {:?}", args);
        let exec_command = utils::run_command_child_process(String::from("kubectl"), args);
        let Ok(output_command) = exec_command.wait_with_output() else {
            panic!("cannot exec command child process");
        };
        match String::from_utf8(output_command.stdout) {
            Ok(val) => val,
            Err(err) => {
                eprintln!("error read from utf8: {}", err.to_string());
                String::new()
            }
        }
    }

    fn get_mode(&self) -> &String {
        self.command_mode.get(&self.mode).unwrap()
    }
}
