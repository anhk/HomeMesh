use libc::*;
use std::{
    fs,
    io::{self, Error, Read, Write},
    os::fd::AsRawFd,
    process::Command,
};

const IFNAMSIZ: usize = 16;
const IFF_TUN: i16 = 0x0001;
const IFF_NO_PI: i16 = 0x1000;
const TUNSETIFF: u64 = 0x400454ca; // TODO: use _IOW('T', 202, int)

pub struct IoctlFlagsData {
    pub ifr_name: [u8; IFNAMSIZ],
    pub ifr_flags: i16,
}

pub struct Tun {
    pub handle: fs::File,
    pub ifname: String,
}

pub fn alloc_tun() -> Result<Tun, io::Error> {
    use std::fs::OpenOptions;

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/net/tun")?;

    let mut req = IoctlFlagsData {
        ifr_name: [0u8; IFNAMSIZ],
        ifr_flags: IFF_TUN | IFF_NO_PI,
    };

    if unsafe { ioctl(file.as_raw_fd(), TUNSETIFF, &mut req) } < 0 {
        return Err(io::Error::last_os_error());
    }

    let size = req.ifr_name.iter().position(|&r| r == 0).unwrap();

    let tun = Tun {
        handle: file,
        ifname: String::from_utf8(req.ifr_name[..size].to_vec()).unwrap(),
    };
    tun.set_address("33.33.33.253/24")?.up()?;
    Ok(tun)
}

impl Tun {
    pub fn set_address(&self, cidr: &str) -> Result<&Self, io::Error> {
        Command::new("ifconfig")
            .arg(&self.ifname)
            .arg(cidr)
            .output()?;
        Ok(self)
    }

    pub fn up(&self) -> Result<&Self, io::Error> {
        Command::new("ifconfig")
            .arg(&self.ifname)
            .arg("up")
            .output()?;
        Ok(self)
    }
}

impl super::Tun for Tun {}

impl Read for Tun {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        self.handle.read(buf)
    }
}

impl Write for Tun {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.handle.write(buf)
    }
    fn flush(&mut self) -> Result<(), Error> {
        self.handle.flush()
    }
}
