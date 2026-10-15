use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum Distro {
    Arch,
    Debian,
    Fedora,
    NixOS,
    OpenSUSE,
    Alpine,
    Void,
    Gentoo,
    Unknown,
}

pub fn detect() -> Distro {
    if Path::new("/etc/NIXOS").exists() || Path::new("/nix/store").exists() {
        return Distro::NixOS;
    }
    if Path::new("/etc/arch-release").exists() {
        return Distro::Arch;
    }
    if Path::new("/etc/debian_version").exists() {
        return Distro::Debian;
    }
    if Path::new("/etc/fedora-release").exists() {
        return Distro::Fedora;
    }
    if Path::new("/etc/alpine-release").exists() {
        return Distro::Alpine;
    }
    if Path::new("/etc/void-release").exists() {
        return Distro::Void;
    }
    if Path::new("/etc/gentoo-release").exists() {
        return Distro::Gentoo;
    }
    if Path::new("/etc/os-release").exists() {
        if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
            if content.contains("openSUSE") {
                return Distro::OpenSUSE;
            }
        }
    }
    Distro::Unknown
}

pub fn name(d: &Distro) -> &'static str {
    match d {
        Distro::Arch => "Arch Linux",
        Distro::Debian => "Debian/Ubuntu",
        Distro::Fedora => "Fedora/RHEL",
        Distro::NixOS => "NixOS",
        Distro::OpenSUSE => "openSUSE",
        Distro::Alpine => "Alpine Linux",
        Distro::Void => "Void Linux",
        Distro::Gentoo => "Gentoo",
        Distro::Unknown => "Unknown",
    }
}
