# Module 10: Asynchronous Programming in Rust

## Tutorial 1: Timer.
# Experiment 1.1 — Timer Original dari Buku

Pada percobaan awal ini, program membuat **async executor** dan `TimerFuture` secara manual dari awal.

Program akan menjalankan sebuah async task yang melakukan tiga hal:

1. Menampilkan teks `howdy!`
2. Menunggu selama 2 detik menggunakan `TimerFuture`
3. Menampilkan teks `done!`

### Output

```bash
Jenisa's Komputer: howdy!
Jenisa's Komputer: done!
```

Terlihat ada jeda sekitar 2 detik antara kedua output tersebut.
Hal ini terjadi karena `TimerFuture` berjalan secara asynchronous di background thread tanpa menghentikan keseluruhan program.

---

# Experiment 1.2 — Memahami Cara Kerja Async Executor

## Perubahan yang Dilakukan

Ditambahkan satu `println!` setelah `spawner.spawn(...)`, tetapi sebelum `drop(spawner)` dan `executor.run()`.

```rust
fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    spawner.spawn(async {
        println!("Jenisa's Komputer: howdy!");
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("Jenisa's Komputer: done!");
    });

    println!("Jenisa's Komputer: hey hey"); // tambahan

    drop(spawner);
    executor.run();
}
```

---

## Output

```bash
Jenisa's Komputer: hey hey
Jenisa's Komputer: howdy!
Jenisa's Komputer: done!
```

---

## Kenapa `hey hey` muncul lebih dulu?

Hal ini menunjukkan konsep penting dalam asynchronous programming:

> `spawn()` tidak langsung menjalankan task.

Ketika `spawner.spawn(...)` dipanggil, task async hanya dimasukkan ke dalam antrean executor (*task queue*).
Task tersebut belum dijalankan saat itu juga.

Karena itu, program langsung melanjutkan eksekusi ke baris berikutnya:

```rust
println!("Jenisa's Komputer: hey hey");
```

Baru setelah `executor.run()` dipanggil, executor mulai mengambil task dari queue dan menjalankannya satu per satu.

---

## Urutan Eksekusi

| Kode      | Waktu Dieksekusi                               |
| --------- | ---------------------------------------------- |
| `hey hey` | Langsung dijalankan secara synchronous         |
| `howdy!`  | Saat `executor.run()` mulai menjalankan task   |
| `done!`   | Setelah `TimerFuture` selesai menunggu 2 detik |

---

## Kesimpulan

Dari percobaan ini bisa dipahami bahwa:

* `spawn()` hanya mendaftarkan task async ke executor
* Task belum berjalan sebelum executor dijalankan
* `executor.run()` bertugas menjalankan seluruh task yang ada di queue
* `await` memungkinkan program menunggu proses tertentu tanpa memblokir keseluruhan eksekusi

Singkatnya:

```text
spawn()  -> menjadwalkan task
run()    -> menjalankan task
await    -> menunggu proses async selesai
```

Konsep ini menjadi dasar penting dalam asynchronous programming di Rust karena memungkinkan banyak task berjalan secara efisien tanpa harus memblokir thread utama.

---

## Experiment 1.3 — Multiple Spawn dan Menghapus `drop(spawner)`

# Part 1 — Multiple Spawn

Pada percobaan ini, terdapat tiga async task yang di-*spawn* sebelum `executor.run()` dipanggil.

## Output

```bash id="g4z3ta"
Jenisa's Komputer: hey hey
Jenisa's Komputer: howdy!
Jenisa's Komputer: howdy2!
Jenisa's Komputer: howdy3!
Jenisa's Komputer: done2!
Jenisa's Komputer: done3!
Jenisa's Komputer: done!
```

---

## Kenapa urutan `done` tidak berurutan?

Semua task dijalankan secara asynchronous dan berjalan bersamaan (*concurrently*).

Ketika setiap task mencapai kode:

```rust id="1i8z2h"
TimerFuture::new(Duration::new(2, 0)).await
```

task tersebut akan:

* berhenti sementara (*yield*)
* mengembalikan kontrol ke executor
* menunggu timer selesai di background thread

Karena ketiga timer berjalan secara bersamaan, urutan munculnya pesan `done` bergantung pada thread mana yang selesai lebih dulu.

Akibatnya, urutan output seperti:

* `done2!`
* `done3!`
* `done!`

bisa berbeda setiap kali program dijalankan.

Hal ini disebut **non-deterministic behavior**, yaitu hasil eksekusi tidak selalu memiliki urutan yang sama.

---

## Peran Tiap Komponen

| Komponen        | Fungsi                                               |
| --------------- | ---------------------------------------------------- |
| `Spawner`       | Mengirim task ke dalam queue executor                |
| `Executor`      | Mengambil dan menjalankan task sampai selesai        |
| `drop(spawner)` | Memberi tahu executor bahwa tidak ada task baru lagi |

---

# Part 2 — Menghapus `drop(spawner)`

Pada percobaan ini, `drop(spawner)` di-comment atau dihapus.

Contoh:

```rust id="s3v2nq"
// drop(spawner);
executor.run();
```

---

## Apakah program tetap berjalan?

Ya, pada program sederhana ini program tetap selesai dengan normal.

Hal ini karena `spawner` otomatis di-*drop* ketika `main()` selesai dijalankan, sehingga executor tetap mengetahui bahwa tidak ada task tambahan lagi.

---

## Kenapa `drop(spawner)` tetap penting?

Dalam program yang lebih kompleks, `spawner` bisa saja:

* disimpan lebih lama
* dikirim ke thread lain
* atau tetap hidup selama program berjalan

Jika `spawner` masih hidup, executor akan terus menunggu task baru masuk ke queue.

Akibatnya:

```text id="8e7a2k"
executor.run()
```

bisa terus berjalan selamanya (*blocking forever*) karena executor mengira masih ada kemungkinan task baru dikirim.

Executor tidak memiliki cara untuk mengetahui bahwa pengiriman task sebenarnya sudah selesai.

---

## Kesimpulan

`drop(spawner)` penting untuk memberi sinyal bahwa:

* queue task sudah ditutup
* tidak akan ada task baru lagi
* executor boleh berhenti setelah semua task selesai

Tanpa sinyal tersebut, executor dapat terus menunggu tanpa akhir pada kasus tertentu.

Singkatnya:

```text id="k4v1jm"
drop(spawner) -> menutup pengiriman task
executor.run() -> berhenti ketika semua task selesai
```

Percobaan ini membantu memahami bagaimana async executor mengatur banyak task sekaligus dan bagaimana mekanisme penghentian executor bekerja di Rust asynchronous programming.
