use crate::backend;
use crate::detect::Distro;

pub fn run(names: &[String], distro: &Distro) {
    let be = backend::get(distro);

    if be.remove.is_empty() {
        eprintln!("pkg: неизвестная система");
        return;
    }

    let mut cmd = be.remove.clone();
    for name in names {
        cmd.push(name.clone());
    }

    println!("pkg: выполняю: {}", cmd.join(" "));

    let status = if be.needs_root {
        let mut full = vec!["sudo".to_string()];
        full.extend(cmd.clone());
        std::process::Command::new(&full[0])
            .args(&full[1..])
            .status()
    } else {
        std::process::Command::new(&cmd[0])
            .args(&cmd[1..])
            .status()
    };

    match status {
        Ok(s) if s.success() => println!("pkg: готово"),
        _ => eprintln!("pkg: ошибка"),
    }
}
