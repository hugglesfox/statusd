# Statusd

Statusd is a xsetroot(1) based status bar daemon. Statusd is primarily designed
for dwm.

The status line is somewhat modular as each component is a struct which
implements
[`std::fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html)
in order to control how it's displayed. Although this means that source code
changes are required in order to customise the status bar.

