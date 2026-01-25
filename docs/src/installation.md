# Installation and Setup

**Kanji Master** is written in Rust. To build and run the application, ensure you have Rust and the necessary system libraries installed.

---

## Prerequisites

1.  Install [Rust and Cargo](https://rustup.rs/).
2.  Ensure your system has the required libraries for compiling GUI applications:

    *   **Linux:** `fontconfig`, `xcb`, and other dependencies specific to your distribution.
    *   **macOS / Windows:** Usually, no additional libraries beyond Rust are required.

> ⚠️ The list of system libraries may vary depending on your OS.

---

## Building and Installation

### 1. Cloning the Repository

```bash
git clone https://github.com/atxxxm/KANJI-MASTER
cd KANJI-MASTER
```

---

### 2. Installation via Cargo

There are several ways to install Kanji Master:

#### a) Installing directly from GitHub:

```bash
cargo install --git https://github.com/atxxxm/KANJI-MASTER
```

#### b) Installing from a local copy of the repository:

```bash
cargo install --path .
```

> 🔹 Using `--path` is convenient for testing or if you plan to modify the project locally.

---

## 3. Configuring the PATH

To make Cargo binaries and installed applications runnable from anywhere, ensure that the path `$HOME/.cargo/bin` (Linux/macOS) or the corresponding Windows path is added to your system's PATH variable.

### Linux / macOS (bash / zsh)

Add to `~/.bashrc` or `~/.zshrc`:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Apply the changes:

```bash
source ~/.bashrc   # or source ~/.zshrc
```

Verification:

```bash
cargo --version
kanji-master --help
```

### Fish shell

```fish
set -Ux PATH $HOME/.cargo/bin $PATH
```

### Windows (PowerShell / CMD)

When installing Rust via `rustup`, the PATH variable is usually configured automatically.
If you need to add it manually, the typical path to Cargo is:

```
C:\Users\<Your_Username>\.cargo\bin
```

*   In PowerShell, you can add it temporarily like this:

```powershell
$env:PATH += ";C:\Users\<Your_Username>\.cargo\bin"
```

*   To add it permanently, use **System Properties → Advanced → Environment Variables → PATH**.

> After this, all Cargo commands and installed applications (including `kanji-master`) will be accessible from anywhere.

---

## 4. Running the Application

To run from source:

```bash
cargo run --release
```

If the application was installed via Cargo:

```bash
kanji-master
```

> 🔹 The `--release` flag is needed for optimized animations and interface performance.

---

## 5. First Launch and Data Initialization

On the first launch of Kanji Master:

*   The application will automatically create all necessary data and configuration files.
*   Wait for the process to complete before starting to work with the program.