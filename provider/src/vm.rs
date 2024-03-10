use std::{
    env,
    error::Error,
    path::PathBuf,
    process::{Command, Stdio},
};

#[derive(Clone)]
enum Arch {
    X86,
    Arm,
    Aarch64,
}

impl Arch {
    pub fn get_command(&self) -> String {
        match self {
            Arch::X86 => String::from("qemu-system-x86_64"),
            Arch::Arm => String::from("qemu-system-arm"),
            Arch::Aarch64 => String::from("qemu-system-aarch64"),
        }
    }
}

pub struct Specifications {
    cpu: u32,
    ram: u32,
    storage: u32,
    arch: Arch,
}

impl Specifications {
    pub fn get_specifications(cpu: u32, ram: u32, storage: u32) -> Self {
        let arch_string = env::consts::ARCH;
        let mut arch: Arch = Arch::X86;
        if arch_string == "x86_64" {
            arch = Arch::X86;
        } else if arch_string == "arm" {
            arch = Arch::Arm;
        } else if arch_string == "aarch64" {
            arch = Arch::Aarch64;
        }
        Self {
            cpu,
            ram,
            storage,
            arch,
        }
    }
    pub fn create_image(&self) -> Result<(), Box<dyn Error>> {
        Command::new("qemu-img")
            .arg("create")
            .arg("-f")
            .arg("qcow2")
            .arg("host.img")
            .arg(format!("{}G", self.storage))
            .status()
            .expect("Failed to create image");
        Ok(())
    }

    pub fn run_vm(&self, iso: PathBuf) -> Result<(), Box<dyn Error>> {
        let base_command = self.arch.get_command();
        self.create_image()?;

        let inner_command = format!(
            "{} -enable-kvm -boot menu=off -cpu host -drive file=host.img -cdrom {} -m {}G -smp {} -device e1000,netdev=net0 -netdev user,id=net0,hostfwd=tcp::5555-:22",
            base_command,
            iso.to_str().unwrap(),
            self.ram,
            self.cpu
        );
        println!("{}", inner_command);
        Command::new("sh")
            .arg("-c")
            .arg(inner_command)
            .status()
            .expect("Failed to run shell");

        Ok(())
    }
}

#[test]
pub fn test_1() -> Result<(), Box<dyn Error>> {
    let spec = Specifications::get_specifications(1, 1, 5);
    spec.run_vm(PathBuf::from(
        r"/home/utkarsh/Downloads/ubuntu-22.04.4-live-server-amd64\(1\).iso",
    ))?;
    Ok(())
}
