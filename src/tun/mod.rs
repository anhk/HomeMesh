use std::io::{self, Read, Write};
mod linux;

pub trait Tun {
    fn add_route(&self, cidr: &str) -> Result<(), io::Error>;
    fn remove_route(&self, cidr: &str) -> Result<(), io::Error>;
}

pub fn alloc_tun() -> Result<impl Tun + Read + Write, io::Error> {
    #[cfg(target_os = "linux")]
    let tun = linux::alloc_tun()?;

    #[cfg(target_os = "macos")]
    let tun = macos::alloc_tun()?;

    Ok(tun)
}
