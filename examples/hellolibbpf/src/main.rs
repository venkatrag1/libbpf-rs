mod hellolibbpf {
    include!(concat!(env!("OUT_DIR"), "/hellolibbpf.skel.rs"));
}
use hellolibbpf::*;
use anyhow::{bail, Result};
use std::{thread, time};

fn bump_memlock_rlimit() -> Result<()> {
    let rlimit = libc::rlimit {
        rlim_cur: 128 << 20,
        rlim_max: 128 << 20,
    };

    if unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlimit) } != 0 {
        bail!("Failed to increase rlimit");
    }

    Ok(())
}

fn main() -> Result<()> {

    let mut skel_builder = HellolibbpfSkelBuilder::default();
    skel_builder.obj_builder.debug(true);
    bump_memlock_rlimit()?;
    let mut open_skel = skel_builder.open()?;
    let mut skel = open_skel.load()?;
    skel.attach();
    loop {
        thread::sleep(time::Duration::from_millis(10));
    }
}



