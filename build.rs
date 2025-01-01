// build.rs
use bootloader::{BootImageConfig, BiosBoot, UefiBoot};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // 首先构建内核
    let kernel_target = "x86_64-unknown-none";
    let status = Command::new("cargo")
      .args(&["build", "--target", kernel_target])
      .status()
      .expect("Failed to build kernel");
    if!status.success() {
        panic!("Kernel build failed");
    }

    // 构建完成后，获取内核的二进制文件路径
    let mut kernel_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    kernel_path.push("target");
    kernel_path.push(kernel_target);
    kernel_path.push("debug");
    kernel_path.push("kernel");  // 这里的 "kernel" 是内核的二进制名称，根据实际情况修改
    if!kernel_path.exists() {
        panic!("Kernel binary not found");
    }

    // BIOS 引导配置
    let bios_boot_image_config = BootImageConfig {
        entry_point: 0x100000,  // 入口点地址，可根据需要调整
        kernel_file_path: kernel_path.clone(),
        bootloader_type: BiosBoot,
    };
    // 使用 bootloader 创建 BIOS 磁盘映像
    bootloader::create_boot_image(bios_boot_image_config).expect("Failed to create BIOS boot image");

    // UEFI 引导配置
    let uefi_boot_image_config = BootImageConfig {
        entry_point: 0x100000,  // 入口点地址，可根据需要调整
        kernel_file_path: kernel_path.clone(),
        bootloader_type: UefiBoot,
    };
    // 使用 bootloader 创建 UEFI 磁盘映像
    bootloader::create_boot_image(uefi_boot_image_config).expect("Failed to create UEFI boot image");
}