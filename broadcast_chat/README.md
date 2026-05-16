## Experiment 2.1 — Original Code of Broadcast Chat

---

# Cara Menjalankan Program

## Terminal 1 — Menjalankan Server

```bash id="v8k1sa"
cd broadcast_chat
cargo run --bin server
```

---

## Terminal 2, 3, 4 — Menjalankan Client

```bash id="f2m7qp"
cd broadcast_chat
cargo run --bin client
```

Jalankan beberapa client di terminal yang berbeda agar bisa saling mengirim pesan.

---

# Apa yang Terjadi?

Saat sebuah client mengetik pesan lalu menekan Enter:

1. Pesan dikirim ke server
2. Server menerima pesan tersebut
3. Server melakukan **broadcast** ke semua client lain yang sedang terhubung

Selain itu, server juga mencatat:

* alamat IP pengirim
* port client
* isi pesan yang diterima

---

# Hasil Program

```text id="x4b2mn"
Server:
- listening on port 2000
- New connection from 127.0.0.1:xxxxx
- Menampilkan semua pesan yang diterima

Client:
- Menerima pesan dari client lain
- Pesan ditampilkan dengan prefix "From server:"
```

---

# Penjelasan Output

## Pada Server

Server akan menampilkan:

* `listening on port 2000`
* notifikasi ketika ada client baru terhubung
* semua pesan yang masuk beserta alamat pengirimnya

Contoh:

```text id="q9s7fd"
New connection from 127.0.0.1:54321
127.0.0.1:54321: halo semua
```

---

## Pada Client

Setiap client akan menerima pesan broadcast dari server.

Contoh tampilan:

```text id="j3n6we"
From server: halo semua
```

Artinya pesan dari satu client berhasil dikirim ke semua client lainnya melalui server.

---

# Kenapa Menggunakan WebSocket dan Async?

## WebSocket

WebSocket memungkinkan koneksi dua arah (*two-way communication*) antara client dan server secara terus-menerus tanpa harus membuat koneksi baru setiap kali mengirim pesan.

Hal ini membuat chat menjadi:

* lebih cepat
* real-time
* efisien

---

## Async dengan Tokio

Program menggunakan async runtime dari Tokio agar server dapat menangani banyak client sekaligus tanpa blocking.

Setiap koneksi client dijalankan sebagai async task terpisah sehingga:

* banyak client bisa aktif bersamaan
* server tetap responsif
* proses pengiriman pesan menjadi lebih efisien

---

# Kesimpulan

Pada experiment ini:

* server berhasil menerima banyak koneksi client
* pesan dari satu client dapat di-broadcast ke client lain
* WebSocket digunakan untuk komunikasi real-time
* async Tokio memungkinkan server menangani banyak koneksi secara bersamaan tanpa menghambat proses lain

Konsep ini merupakan dasar dari aplikasi chat modern yang membutuhkan komunikasi cepat dan concurrent connection handling.

## Experiment 2.2 — Mengubah Port WebSocket

---

# Perubahan yang Dilakukan

Pada percobaan ini, port WebSocket diubah dari `2000` menjadi `8080`.

Terdapat dua file yang dimodifikasi:

## 1. `src/bin/server.rs`

Port binding pada server diubah dari:

```rust id="n5v8qk"
2000
```

menjadi:

```rust id="c2w7mz"
8080
```

Server sekarang akan mendengarkan koneksi pada port `8080`.

---

## 2. `src/bin/client.rs`

URL koneksi pada client juga diubah dari:

```rust id="f9x1pa"
ws://127.0.0.1:2000
```

menjadi:

```rust id="r4m8yt"
ws://127.0.0.1:8080
```

Client sekarang terhubung ke server menggunakan port baru.

---

# Kenapa Kedua File Harus Diubah?

Program chat ini menggunakan arsitektur client-server.

Artinya:

* server membuka dan mendengarkan koneksi pada suatu port
* client harus terhubung ke port yang sama

Jika hanya salah satu sisi yang diubah:

* server dan client akan menggunakan port berbeda
* koneksi gagal dilakukan

Contohnya:

* server mendengarkan di `8080`
* client masih mencoba connect ke `2000`

maka client tidak akan menemukan server.

---

# Hasil Program

Setelah kedua file diubah:

* server berhasil berjalan pada port `8080`
* semua client dapat terhubung dengan normal

Output server sekarang menampilkan:

```text id="y7d2kl"
listening on port 8080
```

Dan client tetap dapat mengirim serta menerima pesan seperti sebelumnya.

---

# Kesimpulan

Pada experiment ini dipahami bahwa:

* port adalah jalur komunikasi antara client dan server
* client dan server harus menggunakan port yang sama
* perubahan konfigurasi koneksi harus dilakukan di kedua sisi

Experiment ini juga menunjukkan bagaimana WebSocket connection bekerja dalam arsitektur jaringan berbasis client-server.
