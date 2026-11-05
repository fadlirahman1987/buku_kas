#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // plugin dialog: pilih folder & "simpan sebagai"
        .plugin(tauri_plugin_dialog::init())
        // plugin fs: baca/tulis file data di folder pilihan pengguna
        .plugin(tauri_plugin_fs::init())
        // simpan izin folder yang dipilih agar berlaku lagi saat aplikasi dibuka ulang
        // (harus didaftarkan SETELAH plugin fs)
        .plugin(tauri_plugin_persisted_scope::init())
        .run(tauri::generate_context!())
        .expect("gagal menjalankan aplikasi Buku Kas Pribadi");
}
