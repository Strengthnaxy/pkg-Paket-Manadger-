use crate::backend;
use crate::detect::Distro;

pub fn run(names: &[String], distro: &Distro) {
    let be = backend::get(distro);

    if be.install.is_empty() {
        eprintln!("pkg: неизвестная система, не знаю как ставить");
        return;
    }

    // Особый случай для NixOS
    if *distro == Distro::NixOS {
        for name in names {
            let mut cmd = be.install.clone();
            cmd.push(format!("nixpkgs#{}", name));
            run_cmd(&mut cmd, be.needs_root);
        }
        return;
    }

    // Для остальных: добавляем имена в конец
    let mut cmd = be.install.clone();
    for name in names {
        cmd.push(name.clone());
    }
    run_cmd(&mut cmd, be.needs_root);
}

fn run_cmd(cmd: &mut Vec<String>, needs_root: bool) {
    println!("pkg: выполняю: {}", cmd.join(" "));

    let status = if needs_root {
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
