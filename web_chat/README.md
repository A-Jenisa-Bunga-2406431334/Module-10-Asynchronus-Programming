---

## Experiment 3.1: Kode Asli WebChat

### Cara Menjalankan

**Terminal 1 — Jalankan WebSocket Server:**
```bash
cd SimpleWebsocketServer
npm start
```

**Terminal 2 — Build dan Jalankan YewChat:**
```bash
cd YewChat
wasm-pack build --target bundler --out-name yewchat --out-dir pkg
npm start
```

Setelah kedua terminal berjalan, buka browser dan akses `http://localhost:8000`.

### Hasil

Aplikasi webchat berhasil dijalankan di browser dengan tampilan yang terdiri dari panel Users di sebelah kiri, area Chat di tengah, dan kolom input pesan di bagian bawah. Pesan yang dikirim ditampilkan secara real-time melalui koneksi WebSocket.

### Catatan

Karena ketidakcocokan dependency antara repo original dengan versi Rust terbaru, diperlukan beberapa langkah tambahan:
- Downgrade `futures` ke versi `0.3.28`
- Downgrade `bumpalo` ke versi `3.14.0`
- Menggunakan Rust versi `1.71.0` via `rustup override set 1.71.0`
- Build menggunakan `wasm-pack --target bundler`

Hal ini menunjukkan bahwa dependency management dalam ekosistem Rust dan WebAssembly sangat sensitif terhadap versi, dan kompatibilitas antar versi perlu diperhatikan dengan seksama.

---

## Experiment 3.2: Be Creative!

### Modifikasi yang dilakukan

Tampilan YewChat dimodifikasi dengan tema hijau dan beberapa perubahan UI:

1. **Tema warna hijau** — seluruh background, panel users, header, dan input box menggunakan palet warna hijau
2. **Bubble chat berbeda** — pesan sendiri (kanan) menggunakan bubble hijau tua, pesan orang lain (kiri) menggunakan bubble hijau muda
3. **Header diganti** — judul chat diubah menjadi "A-Jenisa-Bunga-2406431334's Chat!"
4. **Status online** — panel users menampilkan status "🟢 Online" untuk setiap user yang terhubung
