# pkg

Универсальный пакетный менеджер для Linux. Работает на Arch, Debian, Fedora, NixOS, openSUSE, Alpine, Void, Gentoo.

Вызывает **родной пакетный менеджер** системы. Одна команда — везде.

---

## Установка

### Из исходников

```bash
git clone https://github.com/твой-ник/pkg.git
cd pkg
cargo build --release
cp target/release/pkg ~/.local/bin/
```

Убедись, что `~/.local/bin` в `PATH`:

```fish
fish_add_path ~/.local/bin
```

### Через cargo

```bash
cargo install --path .
```

---

## Быстрый старт

```bash
pkg -S firefox              # установить firefox
pkg -S kitty neovim git     # установить несколько пакетов
pkg -R firefox              # удалить
pkg -Qs python              # поиск
pkg -Q                      # список установленного
pkg -h                      # справка
```

---

## Команды

### `-S` / `--install` — установка

Устанавливает один или несколько пакетов. Вызывает родной пакетный менеджер системы.

```bash
pkg -S firefox
pkg -S kitty neovim git ripgrep
pkg --install firefox
```

**Что происходит на разных системах:**

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

---

### `-R` / `--remove` — удаление

Удаляет один или несколько пакетов.

```bash
pkg -R firefox
pkg -R kitty neovim
pkg --remove firefox
```

**Что происходит на разных системах:**

| Система | Команда |
|---|---|
| Arch | `sudo pacman -R --noconfirm firefox` |
| Debian/Ubuntu | `sudo apt remove -y firefox` |
| Fedora | `sudo dnf remove -y firefox` |
| NixOS | `nix profile remove firefox` |
| openSUSE | `sudo zypper remove -y firefox` |
| Alpine | `sudo apk del firefox` |
| Void | `sudo xbps-remove -y firefox` |
| Gentoo | `sudo emerge --unmerge firefox` |

---

### `-Qs` / `--search` — поиск

Ищет пакеты в репозиториях системы.

```bash
pkg -Qs python
pkg -Qs fire
pkg --search neovim
```

**Что происходит на разных системах:**

| Система | Команда |
|---|---|
| Arch | `pacman -Ss python` |
| Debian/Ubuntu | `apt search python` |
| Fedora | `dnf search python` |
| NixOS | `nix search nixpkgs python` |
| openSUSE | `zypper search python` |
| Alpine | `apk search python` |
| Void | `xbps-query -Rs python` |
| Gentoo | `emerge --search python` |

---

### `-Q` / `--list` — список установленного

Показывает все установленные пакеты.

```bash
pkg -Q
pkg --list
```

**Что происходит на разных системах:**

| Система | Команда |
|---|---|
| Arch | `pacman -Q` |
| Debian/Ubuntu | `dpkg -l` |
| Fedora | `dnf list installed` |
| NixOS | `nix profile list` |
| openSUSE | `zypper packages --installed-only` |
| Alpine | `apk info -v` |
| Void | `xbps-query -l` |
| Gentoo | `qlist -I` |

---

### `-h` / `--help` — справка

Показывает все доступные команды и определяет систему.

```bash
pkg -h
pkg --help
```

Пример вывода:

```
pkg — универсальный пакетный менеджер
Система: Arch Linux

Использование:
  pkg -S <имя>       установить
  pkg -R <имя>       удалить
  pkg -Qs <запрос>   поиск
  pkg -Q             список установленного
  pkg -h             справка

Примеры:
  pkg -S firefox
  pkg -S kitty neovim git
  pkg -R firefox
  pkg -Qs python
```

---

## Поддерживаемые системы

| Система | Менеджер | Root нужен? |
|---|---|---|
| Arch Linux | pacman | да |
| Debian / Ubuntu | apt | да |
| Fedora / RHEL | dnf | да |
| NixOS | nix profile | нет |
| openSUSE | zypper | да |
| Alpine Linux | apk | да |
| Void Linux | xbps | да |
| Gentoo | emerge | да |

**Определение системы** происходит автоматически по файлам:
- `/etc/NIXOS` или `/nix/store` → NixOS
- `/etc/arch-release` → Arch
- `/etc/debian_version` → Debian
- `/etc/fedora-release` → Fedora
- `/etc/alpine-release` → Alpine
- `/etc/void-release` → Void
- `/etc/gentoo-release` → Gentoo
- `/etc/SuSE-release` → openSUSE

---

## Как это работает

1. **Определение системы.** `pkg` читает `/etc/*-release` и понимает, какой у тебя дистрибутив.
2. **Выбор бэкенда.** Для каждой системы есть свой набор команд.
3. **Выполнение.** `pkg` запускает нужную команду через `sudo` (если требуется).
4. **Вывод.** Показывает, что именно выполняется, и результат.

**Пример работы на Arch:**

```bash
$ pkg -S firefox
pkg: выполняю: sudo pacman -S --noconfirm firefox
[sudo] password for user:
resolving dependencies...
looking for conflicting packages...
...
pkg: готово
```

**Пример работы на NixOS:**

```bash
$ pkg -S firefox
pkg: выполняю: nix profile install nixpkgs#firefox
...
pkg: готово
```

---

## Флаги (шпаргалка)

| Флаг | Длинная форма | Что делает |
|---|---|---|
| `-S` | `--install` | Установить пакет |
| `-R` | `--remove` | Удалить пакет |
| `-Qs` | `--search` | Поиск пакета |
| `-Q` | `--list` | Список установленного |
| `-h` | `--help` | Справка |

---

## Примеры

### Установить один пакет

```bash
pkg -S firefox
```

### Установить несколько пакетов

```bash
pkg -S kitty neovim git ripgrep fd bat fzf
```

### Удалить пакет

```bash
pkg -R firefox
```

### Удалить несколько

```bash
pkg -R firefox chromium
```

### Найти пакет

```bash
pkg -Qs python
```

### Посмотреть все установленное

```bash
pkg -Q
```

### Справка

```bash
pkg -h
```

---

## Требования

- **Rust** (для сборки): `sudo pacman -S rust` или [rustup](https://rustup.rs)
- **Родной пакетный менеджер** системы (pacman, apt, dnf и т.д.)
- **sudo** (для систем, где нужен root)
- **Nix** (только для NixOS)

---

## Сборка

### Debug (быстрая сборка)

```bash
cargo build
./target/debug/pkg -h
```

### Release (оптимизированная)

```bash
cargo build --release
./target/release/pkg -h
```

### Установка

```bash
cargo install --path .
```

Или вручную:

```bash
cp target/release/pkg ~/.local/bin/
```

---

## Разработка

### Структура

```
pkg/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs       # CLI, точка входа
    ├── detect.rs     # определение дистрибутива
    ├── backend.rs    # команды для каждой системы
    ├── install.rs    # установка
    ├── remove.rs     # удаление
    └── list.rs       # поиск и список
```

### Добавить новую систему

1. Открой `src/detect.rs`, добавь вариант в `enum Distro`.
2. В `detect()` добавь проверку файла системы.
3. В `name()` добавь название.
4. В `src/backend.rs` добавь блок `match` для новой системы.
5. Собери и проверь.

### Тесты

```bash
cargo test
```

---

## Лицензия

MIT

---

## Автор

strengthnaxy
