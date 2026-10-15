# pkg — универсальный пакетный менеджер для Linux

Одна команда для установки пакетов на любом Linux: Arch, Debian, Fedora, NixOS, openSUSE, Alpine, Void, Gentoo.

Вместо `pacman`, `apt`, `dnf`, `zypper` — просто `pkg -S firefox`. Он сам определит систему и вызовет нужный менеджер.

---

## Что это

`pkg` — тонкая обёртка над родными пакетными менеджерами. Он **не скачивает пакеты сам** и **не содержит своего репозитория**. Он просто определяет твой дистрибутив и вызывает ту команду, которую ты бы вызвал вручную.

Плюсы:
- Одна команда на всех системах.
- Не надо помнить `pacman -S` vs `apt install` vs `dnf install`.
- Работает поверх того, что уже есть. Ничего не ломает.

Минусы:
- Требует родной менеджер системы.
- Не заменяет `pacman`/`apt`/`dnf`, а дополняет.
- Флаги родного менеджера не пробрасываются (пока).

---

## Требования

Что должно быть **до установки**:

- **Rust** и **Cargo** — чтобы собрать проект.
- **Родной пакетный менеджер** системы:
  - Arch → `pacman` (есть по умолчанию)
  - Debian/Ubuntu → `apt` (есть по умолчанию)
  - Fedora → `dnf` (есть по умолчанию)
  - NixOS → `nix` (есть по умолчанию)
  - openSUSE → `zypper` (есть по умолчанию)
  - Alpine → `apk` (есть по умолчанию)
  - Void → `xbps` (есть по умолчанию)
  - Gentoo → `emerge` (есть по умолчанию)
- **`sudo`** — для систем, где нужен root (все, кроме NixOS).
- **`git`** — чтобы клонировать репозиторий.

### Как установить Rust

**Arch:**

```bash
sudo pacman -S rust
```

**Debian/Ubuntu:**

```bash
sudo apt install rustc cargo
```

**Fedora:**

```bash
sudo dnf install rust cargo
```

**NixOS:**

```bash
nix-shell -p cargo rustc
```

**Универсальный способ (rustup):**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

---

## Установка pkg

### Шаг 1. Клонируй репозиторий

```bash
git clone https://github.com/Strengthnaxy/pkg-Paket-Manadger-.git
cd pkg-Paket-Manadger-
```

### Шаг 2. Собери

```bash
cargo build --release
```

Если всё ок — увидишь:

```
Finished `release` profile [optimized] target(s)
```

Бинарник появится в `target/release/pkg`.

### Шаг 3. Установи

**Скопируй бинарник в `~/.local/bin`:**

```bash
mkdir -p ~/.local/bin
cp target/release/pkg ~/.local/bin/pkg
```

**Добавь `~/.local/bin` в PATH**, если его там нет:

**Bash:**

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**Zsh:**

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

**Fish:**

```fish
fish_add_path ~/.local/bin
```

### Шаг 4. Проверь

```bash
pkg -h
```

Должно вывести:

```
pkg — универсальный пакетный менеджер
Система: Arch Linux

Использование:
  pkg -S <имя>       установить
  pkg -R <имя>       удалить
  pkg -Qs <запрос>   поиск
  pkg -Q             список установленного
  pkg -h             справка
```

### Альтернатива: установка через cargo

```bash
cargo install --path .
```

Cargo сам положит бинарник в `~/.cargo/bin`, который обычно уже в PATH.

---

## Использование

### Установить пакет

```bash
pkg -S firefox
```

Что произойдёт в зависимости от системы:

| Система | Команда |
|---|---|
| Arch | `sudo pacman -S --noconfirm firefox` |
| Debian/Ubuntu | `sudo apt install -y firefox` |
| Fedora | `sudo dnf install -y firefox` |
| NixOS | `nix profile install nixpkgs#firefox` |
| openSUSE | `sudo zypper install -y firefox` |
| Alpine | `sudo apk add firefox` |
| Void | `sudo xbps-install -y firefox` |
| Gentoo | `sudo emerge --ask firefox` |

### Установить несколько пакетов сразу

```bash
pkg -S firefox kitty neovim git
```

### Удалить пакет

```bash
pkg -R firefox
```

### Удалить несколько

```bash
pkg -R firefox chromium
```

### Поиск

```bash
pkg -Qs python
```

### Список установленного

```bash
pkg -Q
```

### Справка

```bash
pkg -h
```

---

## Флаги

| Флаг | Длинная форма | Что делает |
|---|---|---|
| `-S` | `--install` | Установить пакет |
| `-R` | `--remove` | Удалить пакет |
| `-Qs` | `--search` | Поиск пакета |
| `-Q` | `--list` | Список установленного |
| `-h` | `--help` | Справка |

---

## Пример работы

**Arch:**

```bash
$ pkg -S firefox
pkg: выполняю: sudo pacman -S --noconfirm firefox
[sudo] password for user:
resolving dependencies...
looking for conflicting packages...
Packages (1) firefox-121.0-1
...
pkg: готово
```

**NixOS:**

```bash
$ pkg -S firefox
pkg: выполняю: nix profile install nixpkgs#firefox
...
pkg: готово
```

---

## Как это работает

1. **Определение системы.** `pkg` читает файлы `/etc/*-release` и понимает, какой у тебя дистрибутив.
2. **Выбор бэкенда.** Для каждой системы свой набор команд (см. таблицы выше).
3. **Выполнение.** `pkg` вызывает нужную команду через `sudo` (если нужно).
4. **Вывод.** Показывает, что именно выполняется.

### Как определяется система

| Файл | Система |
|---|---|
| `/etc/NIXOS` или `/nix/store` | NixOS |
| `/etc/arch-release` | Arch Linux |
| `/etc/debian_version` | Debian / Ubuntu |
| `/etc/fedora-release` | Fedora / RHEL |
| `/etc/alpine-release` | Alpine Linux |
| `/etc/void-release` | Void Linux |
| `/etc/gentoo-release` | Gentoo |
| `/etc/os-release` (содержит «openSUSE») | openSUSE |

---

## Поддерживаемые системы

| Система | Менеджер | Нужен root? |
|---|---|---|
| Arch Linux | pacman | да |
| Debian / Ubuntu | apt | да |
| Fedora / RHEL | dnf | да |
| NixOS | nix profile | нет |
| openSUSE | zypper | да |
| Alpine Linux | apk | да |
| Void Linux | xbps | да |
| Gentoo | emerge | да |

---

## Разработка

### Структура проекта

```
pkg-Paket-Manadger-/
├── Cargo.toml
├── README.md
├── .gitignore
└── src/
    ├── main.rs       # CLI, точка входа
    ├── detect.rs     # определяет дистрибутив
    ├── backend.rs    # команды для каждой системы
    ├── install.rs    # установка
    ├── remove.rs     # удаление
    └── list.rs       # поиск и список
```

### Сборка для разработки

```bash
cargo build
./target/debug/pkg -h
```

### Сборка релиза

```bash
cargo build --release
./target/release/pkg -h
```

### Как добавить новую систему

1. Открой `src/detect.rs`.
2. Добавь вариант в `enum Distro`:

   ```rust
   pub enum Distro {
       Arch,
       Debian,
       // ...
       МояСистема,
   }
   ```

3. В функции `detect()` добавь проверку файла:

   ```rust
   if Path::new("/etc/моя-система-release").exists() {
       return Distro::МояСистема;
   }
   ```

4. В `name()` добавь название:

   ```rust
   Distro::МояСистема => "Моя Система",
   ```

5. Открой `src/backend.rs`, добавь блок в `match`:

   ```rust
   Distro::МояСистема => Backend {
       install: vec!["мой-менеджер".into(), "install".into()],
       remove: vec!["мой-менеджер".into(), "remove".into()],
       search: vec!["мой-менеджер".into(), "search".into()],
       list: vec!["мой-менеджер".into(), "list".into()],
       needs_root: true,
   },
   ```

6. Собери, проверь, отправь pull request.

---

## Лицензия

MIT

---

## Автор

[Strengthnaxy](https://github.com/Strengthnaxy)
