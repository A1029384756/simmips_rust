# Build Instructions
### Linux/WSL
1. Install [rustup](https://rustup.rs/)
2. Install **gtk4** and **libadwaita**
    - For Fedora users: `dnf install gtk4-devel glib2-devel libadwaita-devel gtksourceview5-devel`
3. Run `cargo build`

### MacOS
1. Install [rustup](https://rustup.rs/)
2. Install **gtk4** and **libadwaita** with `brew install gtk4 libadwaita pkg-config gtksourceview5`
3. Run `cargo build`

### Windows 
1. Install [rustup](https://rustup.rs/)
2. Run `rustup toolchain install stable-gnu`
3. Run `rustup default stable-gnu`
4. Install [MSYS2](https://msys2.org) using `winget install msys2`
5. From the MSYS2 MinGW 64-bit shell, execute `pacman -S mingw-w64-x86_64-gtk4 mingw-w64-x86_64-gettext mingw-w64-x86_64-libxml2 mingw-w64-x86_64-librsvg mingw-w64-x86_64-pkgconf mingw-w64-x86_64-gcc mingw-w64-x86_64-libadwaita mingw-w64-x86_64-librsvg mingw-w64-x86_64-gtksourceview5`
6. Go to settings -> Search and open `Advanced system settings` -> Click on `Environment variables`
7. Select `Path` -> Click on `Edit` -> Add the following three entries:
    ```
    C:\msys64\mingw64\include
    C:\msys64\mingw64\bin
    C:\msys64\mingw64\lib
    ```
8. Run `cargo build`

### TODO
Updated windows instructions
