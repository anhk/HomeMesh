use std::io::{self, Read, Write};
mod linux;

pub trait Tun {}

pub fn alloc_tun() -> Result<impl Tun + Read + Write, io::Error> {
    #[cfg(target_os = "linux")]
    let tun = linux::alloc_tun()?;

    #[cfg(target_os = "macos")]
    let tun = macos::alloc_tun()?;

    Ok(tun)
}
