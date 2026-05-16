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
