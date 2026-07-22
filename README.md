# eGUI Demo

A demo GUI application using `egui` and `eframe`.

## References

1. Getting started:
    - <https://medium.com/@lyecre/building-cross-platform-apps-with-egui-and-rust-0a0fbb1779dc>

2. More background information:
    - <https://hackmd.io/@Hamze/BkvEAvFayx>
    - Unfortunately, the code in this resource is already a bit outdated

3. `eframe` template:
    - <https://github.com/emilk/eframe_template>
    - Official template, but already contains many extra features like web GUI and app persistence

## Remarks

Besides `egui`, there also is `gtk4-rs`.
While `egui` is purely Rust-based, `gtk4-rs` depends on GTK4, which is written in C.
The advantage of GTK is that it is a mature GUI framework, that has evolved over many years.

A very nice resource for `gtk4-rs` is the [gtk4-rs book](https://gtk-rs.org/gtk4-rs/git/book/).
It would be nice to have a similar reference for `egui`.
