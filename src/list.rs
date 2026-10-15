use crate::backend;
use crate::detect::Distro;

pub fn installed(distro: &Distro) {
    let be = backend::get(distro);
    if be.list.is_empty() {
        eprintln!("pkg: неизвестная система");
        return;
    }
    std::process::Command::new(&be.list[0])
        .args(&be.list[1..])
        .status()
        .ok();
}

pub fn search(query: &str, distro: &Distro) {
    let be = backend::get(distro);
    if be.search.is_empty() {
        eprintln!("pkg: неизвестная система");
        return;
    }
    let mut cmd = be.search.clone();
    cmd.push(query.to_string());
    std::process::Command::new(&cmd[0])
        .args(&cmd[1..])
        .status()
        .ok();
}
