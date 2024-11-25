use std::fmt::{Display, Formatter};
use std::io::{Cursor, Write as IoWrite};
use std::ops::Deref;

use ar::{Builder as ArBuilder, Header as ArHeader};
use flate2::write::GzEncoder;
use flate2::Compression;
use tar::{Builder as TarBuilder, Header as TarHeader};

pub struct ControlInfo {
    pub package: String,
    pub version: String,
    pub installed_size: u64,
    pub architecture: String,
}

pub trait AppendControl {
    fn append_control(&mut self, info: &ControlInfo, mtime: u64) -> std::io::Result<()>;
}

impl<W> AppendControl for ArBuilder<W>
where
    W: IoWrite,
{
    fn append_control(&mut self, info: &ControlInfo, mtime: u64) -> std::io::Result<()> {
        let control = info.to_string().into_bytes();

        let mut control_tar_gz = Vec::<u8>::new();
        let gz = GzEncoder::new(&mut control_tar_gz, Compression::default());
        let mut tar = TarBuilder::new(gz);

        let mut tar_header = TarHeader::new_gnu();
        tar_header.set_mode(0o100644);
        tar_header.set_size(control.len() as u64);
        tar_header.set_mtime(mtime);
        tar_header.set_cksum();
        tar.append_data(&mut tar_header, "control", control.deref())?;
        drop(tar);

        let mut ar_header = ArHeader::new(b"control.tar.gz".to_vec(), control_tar_gz.len() as u64);
        ar_header.set_mode(0o100644);
        ar_header.set_mtime(mtime);
        self.append(&ar_header, Cursor::new(control_tar_gz))
    }
}

impl Display for ControlInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Package: {}\n", self.package))?;
        f.write_fmt(format_args!("Version: {}\n", self.version))?;
        f.write_fmt(format_args!("Section: {}\n", "misc"))?;
        f.write_fmt(format_args!("Priority: {}\n", "optional"))?;
        f.write_fmt(format_args!("Architecture: {}\n", self.architecture))?;
        f.write_fmt(format_args!("Installed-Size: {}\n", self.installed_size))?;
        f.write_fmt(format_args!("Maintainer: {}\n", "N/A <nobody@example.com>"))?;
        f.write_fmt(format_args!(
            "Description: {}\n",
            "This is a webOS application."
        ))?;
        f.write_fmt(format_args!("webOS-Package-Format-Version: {}\n", 2))?;
        f.write_fmt(format_args!("webOS-Packager-Version: {}\n", "x.y.x"))?;
        Ok(())
    }
}
