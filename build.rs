use std::env;
use std::path::PathBuf;

fn main() -> Result<(), ()> {
    // set by cargo, build scripts shuold use this directory for output files
    let out_dir: PathBuf = env::var_os("OUT_DIR").unwrap().into();

    // set by cargo artifact dependenvy feature
    let kernel: PathBuf = env::var_os("CARGO_BIN_FILE_KERNEL").unwrap().into();

    // create UEFI disk image
    let mut uefi_out_path = out_dir.clone();
    uefi_out_path.push("uefi.img");

    bootloader::UefiBoot::new(&*kernel)
        .create_disk_image(&*uefi_out_path)
        .unwrap();

    // create BIOS disk image
    // let mut bios_out_path = out_dir;
    // bios_out_path.push("bios.img");

    // bootloader::BiosBoot::new(&*kernel)
    // .create_disk_image(&*bios_out_path)
    // .unwrap();

    println!("cargo:rustc-env=UEFI_PATH={}", uefi_out_path.display());
    // println!("cargo:rustc-env=BIOS_PATH={}", bios_out_path.display());
    Ok(())
}
