# Build Instructions
### Linux/WSL
1. Install [rustup](https://rustup.rs/)
2. Install **gtk4** and **libadwaita**
    - For Fedora users: `dnf install gkt4-devel libadwaita-devel`
3. Run `cargo build`

### MacOS
1. Install [rustup](https://rustup.rs/)
2. Install **gtk4** and **libadwaita** with `brew install gtk4 libadwaita`
3. Run `cargo build`

### Windows 
1. Install [rustup](https://rustup.rs/)
2. Install [MSYS2](https://msys2.org) using `winget install msys2`
3. From the MSYS2 MinGW 64-bit shell, execute `pacman -S mingw-w64-x86_64-gtk4 mingw-w64-x86_64-gettext mingw-w64-x86_64-libxml2 mingw-w64-x86_64-librsvg mingw-w64-x86_64-pkgconf mingw-w64-x86_64-gcc mingw-w64-x86_64-libadwaita mingw-w64-x86_64-librsvg`
4. Go to settings -> Search and open `Advanced system settings` -> Click on `Environment variables`
5. Select `Path` -> Click on `Edit` -> Add the following three entries:
    ```
    C:\msys64\mingw64\include
    C:\msys64\mingw64\bin
    C:\msys64\mingw64\lib
    ```
6. Run `cargo build`
