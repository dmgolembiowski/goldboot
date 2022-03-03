use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Serialize, Deserialize, Validate, Default)]
pub struct PackerTemplate {
    pub builders: Vec<QemuBuilder>,
    pub provisioners: Vec<PackerProvisioner>,
}

#[derive(Clone, Serialize, Deserialize, Validate, Default)]
pub struct QemuBuilder {
    pub boot_command: Vec<String>,
    pub boot_wait: String,
    pub communicator: String,
    pub disk_compression: bool,
    pub disk_size: String,
    pub format: String,
    pub headless: bool,
    pub iso_checksum: String,
    pub iso_url: String,
    pub memory: String,
    pub output_directory: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qemu_binary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qemuargs: Option<Vec<Vec<String>>>,
    pub r#type: String,
    pub shutdown_command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_wait_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winrm_insecure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winrm_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winrm_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winrm_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floppy_files: Option<Vec<String>>,
    pub disk_interface: String,
}

impl QemuBuilder {
    pub fn new() -> QemuBuilder {
        let mut builder = QemuBuilder::default();
        builder.format = "qcow2".into();
        builder.headless = true;
        builder.r#type = "qemu".into();
        builder.disk_compression = true;
        builder.disk_interface = "ide".into();

        return builder;
    }
}

#[derive(Clone, Serialize, Deserialize, Validate, Default)]
pub struct PackerProvisioner {
    pub extra_arguments: Vec<String>,
    pub playbook_file: Option<String>,
    pub r#type: String,
    pub scripts: Vec<String>,
    pub use_proxy: Option<bool>,
    pub user: Option<String>,
}
