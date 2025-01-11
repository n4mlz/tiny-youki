use std::io::Result;

use libc::{c_char, setdomainname};
use nix::unistd::sethostname;
use oci_spec::runtime::Spec;

pub fn setup_uts(config: &Spec) -> Result<()> {
    if let Some(hostname) = config.hostname() {
        set_hostname(hostname)?;
    }
    if let Some(domainname) = config.domainname() {
        set_domainname(domainname)?;
    }

    Ok(())
}

fn set_hostname(hostname: &str) -> Result<()> {
    sethostname(hostname)?;
    Ok(())
}

fn set_domainname(domainname: &str) -> Result<()> {
    let ptr = domainname.as_bytes().as_ptr() as *const c_char;
    let len = domainname.len();
    match unsafe { setdomainname(ptr, len) } {
        0 => Ok(()),
        -1 => Err(std::io::Error::last_os_error()),
        _ => unreachable!(),
    }?;

    Ok(())
}
