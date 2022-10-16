# COMPARE EXCEL WITH RUST (GUI).
simple application using Rust to recursive find changes on excel file

- [x] CLI -> output pretty print for the changes
- [x] GUI -> Using [egui/eframe](https://docs.rs/egui/latest/egui/) framework
- [ ] MORE OPTION AND FEATURES FOR COMPARING -> __Future Releases__


- Parsing Excel file with [calamine](https://docs.rs/calamine/latest/calamine/)
- Gui with [egui/eframe](https://docs.rs/egui/latest/egui/)
- Diff Algortm with [similar](https://docs.rs/similar/latest/similar/)

## HOW TO INSTALL

### DOWNLOAD INSTALLER
- Windows: [Download Installer](https://github.com/RizalAchp/CompareExcel/releases/download/release-0.1.0/dispendik_compare-0.1.0-x86_64.msi)
- Linux: TODO!

### COMPILE FROM SOURCE
- using cargo from rust
    - On Unix, run curl https://sh.rustup.rs -sSf | sh in your shell. This downloads and runs rustup-init.sh, which in turn downloads and runs the correct version of the rustup-init executable for your platform.
    - On Windows, download and run rustup-init.exe.


- run after in project directory
    ```bash
    cargo build --release && cargo install
    ```

