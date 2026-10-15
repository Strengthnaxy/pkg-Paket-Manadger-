use crate::detect::Distro;

pub struct Backend {
    pub install: Vec<String>,
    pub remove: Vec<String>,
    pub search: Vec<String>,
    pub list: Vec<String>,
    pub needs_root: bool,
}

pub fn get(distro: &Distro) -> Backend {
    match distro {
        Distro::Arch => Backend {
            install: vec!["pacman".into(), "-S".into(), "--noconfirm".into()],
            remove: vec!["pacman".into(), "-R".into(), "--noconfirm".into()],
            search: vec!["pacman".into(), "-Ss".into()],
            list: vec!["pacman".into(), "-Q".into()],
            needs_root: true,
        },
        Distro::Debian => Backend {
            install: vec!["apt".into(), "install".into(), "-y".into()],
            remove: vec!["apt".into(), "remove".into(), "-y".into()],
            search: vec!["apt".into(), "search".into()],
            list: vec!["dpkg".into(), "-l".into()],
            needs_root: true,
        },
        Distro::Fedora => Backend {
            install: vec!["dnf".into(), "install".into(), "-y".into()],
            remove: vec!["dnf".into(), "remove".into(), "-y".into()],
            search: vec!["dnf".into(), "search".into()],
            list: vec!["dnf".into(), "list".into(), "installed".into()],
            needs_root: true,
        },
        Distro::NixOS => Backend {
            install: vec!["nix".into(), "profile".into(), "install".into()],
            remove: vec!["nix".into(), "profile".into(), "remove".into()],
            search: vec!["nix".into(), "search".into(), "nixpkgs".into()],
            list: vec!["nix".into(), "profile".into(), "list".into()],
            needs_root: false,
        },
        Distro::OpenSUSE => Backend {
            install: vec!["zypper".into(), "install".into(), "-y".into()],
            remove: vec!["zypper".into(), "remove".into(), "-y".into()],
            search: vec!["zypper".into(), "search".into()],
            list: vec!["zypper".into(), "packages".into(), "--installed-only".into()],
            needs_root: true,
        },
        Distro::Alpine => Backend {
            install: vec!["apk".into(), "add".into()],
            remove: vec!["apk".into(), "del".into()],
            search: vec!["apk".into(), "search".into()],
            list: vec!["apk".into(), "info".into(), "-v".into()],
            needs_root: true,
        },
        Distro::Void => Backend {
            install: vec!["xbps-install".into(), "-y".into()],
            remove: vec!["xbps-remove".into(), "-y".into()],
            search: vec!["xbps-query".into(), "-Rs".into()],
            list: vec!["xbps-query".into(), "-l".into()],
            needs_root: true,
        },
        Distro::Gentoo => Backend {
            install: vec!["emerge".into(), "--ask".into()],
            remove: vec!["emerge".into(), "--unmerge".into()],
            search: vec!["emerge".into(), "--search".into()],
            list: vec!["qlist".into(), "-I".into()],
            needs_root: true,
        },
        Distro::Unknown => Backend {
            install: vec![],
            remove: vec![],
            search: vec![],
            list: vec![],
            needs_root: true,
        },
    }
}
