# Buku Kas Pribadi (Tauri v2)

Aplikasi desktop offline untuk mencatat uang masuk & uang keluar pribadi.
Data disimpan sebagai file di folder pilihan Anda (bukan di localStorage browser), otomatis setiap ada perubahan.

## Struktur proyek

```
buku-kas-tauri/
├── package.json                  # hanya memuat Tauri CLI
├── app-icon.png                  # sumber ikon (ganti bila mau, lalu: npm run icons)
├── src/
│   └── index.html                # SELURUH aplikasi (HTML + CSS + JS, tanpa library luar)
├── src-tauri/
│   ├── Cargo.toml                # plugin: fs, dialog, persisted-scope
│   ├── tauri.conf.json           # nama app, ukuran jendela, withGlobalTauri
│   ├── capabilities/default.json # izin fs & dialog
│   ├── icons/                    # ikon hasil `tauri icon`
│   └── src/{main.rs,lib.rs}      # mendaftarkan plugin
└── .github/workflows/build.yml   # build installer Windows otomatis di GitHub
```

## Prasyarat (sekali saja)

1. **Node.js** 18+ — https://nodejs.org
2. **Rust** — https://rustup.rs
3. **Windows:** Microsoft C++ Build Tools (workload "Desktop development with C++"). WebView2 sudah ada di Windows 10/11 terbaru.
4. macOS: Xcode Command Line Tools. Linux: lihat https://v2.tauri.app/start/prerequisites/

## Menjalankan & membuild

```bash
npm install
npm run dev      # jalankan mode pengembangan
npm run build    # buat installer
```

Hasil installer ada di `src-tauri/target/release/bundle/` (`nsis/*.exe` dan `msi/*.msi` di Windows).
Build pertama memakan waktu beberapa menit karena Rust mengunduh dan mengompilasi dependensi.

## Cara kerja penyimpanan

- Pertama kali: klik **Pilih Folder** → pilih folder, mis. `D:\DataKeuangan`.
- Dialog folder (`plugin-dialog`) otomatis memberi izin akses ke folder itu, lalu `plugin-persisted-scope` menyimpannya, jadi **tidak perlu memilih ulang** setiap membuka aplikasi.
- Lokasi folder diingat lewat file kecil `config.json` di folder konfigurasi aplikasi (`%APPDATA%\com.bukukas.pribadi` di Windows). Isinya hanya path folder, bukan data keuangan.
- Data utama: `buku-kas-data.json` (ditulis ke file sementara dulu, lalu di-rename, supaya file tidak rusak bila komputer mati saat menyimpan).
- Cadangan harian: sub-folder `backup/` (60 hari terakhir).
- Ekspor CSV / cadangan JSON memakai dialog "Simpan sebagai" milik Tauri.

## Upload ke GitHub

`.gitignore` sudah mengecualikan `buku-kas-data.json`, `backup/`, dan file CSV supaya **data pribadi tidak ikut ter-commit**. Jangan memilih folder data di dalam folder repository.

Untuk membuat installer otomatis: push ke GitHub, lalu buat tag `v1.0.0`
(`git tag v1.0.0 && git push origin v1.0.0`) atau jalankan manual dari tab **Actions**.
Installer muncul sebagai *draft release*.

## Troubleshooting

- **"forbidden path" / gagal menyimpan setelah folder dipindah:** klik **Pilih Folder** lagi.
- **Windows 10 lama tanpa WebView2:** installer akan mengunduhnya (butuh internet satu kali). Untuk installer yang sepenuhnya offline, tambahkan di `tauri.conf.json` → `bundle.windows.webviewInstallMode` = `{ "type": "offlineInstaller" }`.
- **Cetak laporan:** tombol Cetak membuka dialog cetak sistem; pilih "Save as PDF / Microsoft Print to PDF" untuk membuat PDF.
- **Peringatan SmartScreen** saat menjalankan installer: normal untuk aplikasi tanpa code-signing certificate.
