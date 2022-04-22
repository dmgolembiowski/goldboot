use goldboot_core::{cache::MediaCache, qemu::QemuArgs, *};
use serde::{Deserialize, Serialize};
use std::error::Error;
use validator::Validate;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum UbuntuServerVersion {
	Jammy,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UbuntuServerTemplate {
	pub root_password: String,

	/// The installation media URL
	pub iso_url: String,

	/// A hash of the installation media
	pub iso_checksum: String,

	pub version: UbuntuServerVersion,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub partitions: Option<Vec<Partition>>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub provisioners: Option<Vec<Provisioner>>,
}

impl Default for UbuntuServerTemplate {
	fn default() -> Self {
		Self {
			root_password: String::from("root"),
			iso_url: format!(""),
			iso_checksum: String::from("none"),
			version: UbuntuServerVersion::Jammy,
			partitions: None,
			provisioners: None,
		}
	}
}

impl Template for UbuntuServerTemplate {
	fn build(&self, context: &BuildContext) -> Result<(), Box<dyn Error>> {
		let mut qemuargs = QemuArgs::new(&context);

		qemuargs.drive.push(format!(
			"file={},if=virtio,cache=writeback,discard=ignore,format=qcow2",
			context.image_path
		));
		qemuargs.drive.push(format!(
			"file={},media=cdrom",
			MediaCache::get(self.iso_url.clone(), &self.iso_checksum)?
		));

		// Start VM
		let mut qemu = qemuargs.start_process()?;

		// Send boot command
		#[rustfmt::skip]
		qemu.vnc.boot_command(vec![
		])?;

		// Wait for SSH
		let ssh = qemu.ssh_wait(context.ssh_port, "root", &self.root_password)?;

		// Run provisioners
		for provisioner in &self.provisioners {
			// TODO
		}

		// Shutdown
		ssh.shutdown("poweroff")?;
		qemu.shutdown_wait()?;
		Ok(())
	}
}
