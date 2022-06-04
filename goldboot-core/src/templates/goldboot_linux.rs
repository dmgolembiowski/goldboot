use crate::{
	build::BuildWorker,
	cache::{MediaCache, MediaFormat},
	qemu::QemuArgs,
	templates::*,
};
use colored::*;
use log::info;
use serde::{Deserialize, Serialize};
use simple_error::bail;
use std::error::Error;
use validator::Validate;

#[derive(rust_embed::RustEmbed)]
#[folder = "res/GoldbootLinux/"]
struct Resources;

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct GoldbootLinuxTemplate {
	#[serde(flatten)]
	pub general: GeneralContainer,
}

impl Default for GoldbootLinuxTemplate {
	fn default() -> Self {
		Self {
			general: GeneralContainer {
				base: TemplateBase::GoldbootLinux,
				storage_size: String::from("4 GiB"),
				..Default::default()
			},
		}
	}
}

impl Template for GoldbootLinuxTemplate {
	fn build(&self, context: &BuildWorker) -> Result<(), Box<dyn Error>> {
		info!("Starting {} build", "Goldboot Linux".blue());

		let mut qemuargs = QemuArgs::new(&context);

		qemuargs.drive.push(format!(
			"file={},if=virtio,cache=writeback,discard=ignore,format=qcow2",
			context.image_path
		));
		qemuargs.drive.push(format!(
			"file={},media=cdrom",
			MediaCache::get(String::from("https://mirrors.edge.kernel.org/archlinux/iso/latest/archlinux-2022.03.01-x86_64.iso"), "none", MediaFormat::Iso)?
		));

		// Start VM
		let mut qemu = qemuargs.start_process()?;

		// Temporary root password for the run
		let temp_password = crate::random_password();

		// Send boot command
		#[rustfmt::skip]
		qemu.vnc.boot_command(vec![
			// Initial wait
			wait!(50),
			// Wait for login
			wait_screen_rect!("5b3ca88689e9d671903b3040889c7fa1cb5f244a", 100, 0, 1024, 400),
			// Configure root password
			enter!("passwd"), enter!(temp_password), enter!(temp_password),
			// Configure SSH
			enter!("echo 'AcceptEnv *' >>/etc/ssh/sshd_config"),
			enter!("echo 'PermitRootLogin yes' >>/etc/ssh/sshd_config"),
			// Start sshd
			enter!("systemctl restart sshd"),
		])?;

		// Wait for SSH
		let mut ssh = qemu.ssh_wait(context.ssh_port, "root", &temp_password)?;

		// Run install script
		if let Some(resource) = Resources::get("install.sh") {
			match ssh.upload_exec(
				resource.data.to_vec(),
				vec![("GB_ROOT_PASSWORD", &crate::random_password())],
			) {
				Ok(0) => debug!("Installation completed successfully"),
				_ => bail!("Installation failed"),
			}
		}

		// Shutdown
		ssh.shutdown("poweroff")?;
		qemu.shutdown_wait()?;
		Ok(())
	}

	fn general(&self) -> GeneralContainer {
		self.general.clone()
	}
}
