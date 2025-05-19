use std::process::{Command, Stdio};
use std::path::PathBuf;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use crate::utils::config_loader::get_config;

pub fn start_jupyter_notebook() {
    let workspace_dir = get_config().jupyter.workspace_dir;
    let notebook_dir = PathBuf::from(workspace_dir);

    #[cfg(target_os = "windows")]
    {
        let mut cmd = Command::new("jupyter");
        cmd.arg("notebook")
            .arg("--NotebookApp.token=")
            .arg("--NotebookApp.password=")
            .arg("--NotebookApp.disable_check_xsrf=True")
            .arg("--NotebookApp.allow_origin=*")
            .arg("--NotebookApp.tornado_settings={\"headers\": {\"Content-Security-Policy\": \"frame-ancestors *\"}}")
            .arg("--no-browser")
            .current_dir(&notebook_dir)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .creation_flags(0x08000000); // CREATE_NO_WINDOW

        let _ = cmd.spawn().expect("Failed to launch Jupyter Notebook");
    }

    #[cfg(not(target_os = "windows"))]
    {
        let mut cmd = Command::new("nohup");
        cmd.arg("jupyter")
            .arg("notebook")
            .arg("--NotebookApp.token=")
            .arg("--NotebookApp.password=")
            .arg("--NotebookApp.disable_check_xsrf=True")
            .arg("--NotebookApp.allow_origin=*")
            .arg("--NotebookApp.tornado_settings={\"headers\": {\"Content-Security-Policy\": \"frame-ancestors *\"}}")
            .arg("--no-browser")
            .current_dir(&notebook_dir)
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        let _ = cmd.spawn().expect("Failed to launch Jupyter Notebook");
    }
}
