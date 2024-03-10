use std::{error::Error, path::PathBuf};

use vm_launcher::vm::Specifications;
fn main() -> Result<(), Box<dyn Error>> {
    let iso = PathBuf::from(r"/home/utkarsh/Downloads/alpine-virt-3.19.1-x86_64.iso");
    let spec = Specifications::get_specifications(1, 1, 20);
    spec.run_vm(iso)?;
    Ok(())
}
