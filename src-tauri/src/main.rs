// Mencegah jendela konsol tambahan muncul di Windows (release build)
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    buku_kas_pribadi_lib::run()
}
