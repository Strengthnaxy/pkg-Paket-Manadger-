mod detect;
mod backend;
mod install;
mod remove;
mod list;

use std::env;

fn main() {
    let distro = detect::detect();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help(&distro);
        return;
    }

    match args[1].as_str() {
        "-S" | "--install" => {
            if args.len() < 3 {
                eprintln!("pkg: укажи пакет: pkg -S firefox");
                return;
            }
            install::run(&args[2..], &distro);
        }
        "-R" | "--remove" => {
            if args.len() < 3 {
                eprintln!("pkg: укажи пакет: pkg -R firefox");
                return;
            }
            remove::run(&args[2..], &distro);
        }
        "-Qs" | "--search" => {
            if args.len() < 3 {
                eprintln!("pkg: укажи запрос: pkg -Qs fire");
                return;
            }
            list::search(&args[2], &distro);
        }
        "-Q" | "--list" => list::installed(&distro),
        "-h" | "--help" => print_help(&distro),
        _ => {
            eprintln!("pkg: неизвестная команда: {}", args[1]);
            print_help(&distro);
        }
    }
}

fn print_help(distro: &detect::Distro) {
    println!("pkg — универсальный пакетный менеджер");
    println!("Система: {}", detect::name(distro));
    println!();
    println!("Использование:");
    println!("  pkg -S <имя>       установить");
    println!("  pkg -R <имя>       удалить");
    println!("  pkg -Qs <запрос>   поиск");
    println!("  pkg -Q             список установленного");
    println!("  pkg -h             справка");
    println!();
    println!("Примеры:");
    println!("  pkg -S firefox");
    println!("  pkg -S kitty neovim git");
    println!("  pkg -R firefox");
    println!("  pkg -Qs python");
}
