# Parkir - Sistem Pengelolaan Parkir Digital

Ini adalah backend service untuk aplikasi Sistem Pengelolaan Parkir Digital yang dibangun menggunakan bahasa pemrograman Rust dan framework Actix-web.

## Teknologi Stack

- **Bahasa**: Rust (Edition 2024)
- **Framework Web**: [Actix-web](https://actix.rs/) (v4)
- **Database**: PostgreSQL
- **Driver Database**: `tokio-postgres` dengan connection pooling `deadpool-postgres`
- **Asynchronous Runtime**: Tokio
- **Serialisasi**: Serde

## Prasyarat

Sebelum menjalankan aplikasi ini, pastikan Anda telah menginstal:

1.  **Rust Toolchain**: [Instruksi Instalasi](https://www.rust-lang.org/tools/install)
2.  **PostgreSQL**: Pastikan server database berjalan.

## Instalasi dan Konfigurasi

1.  **Clone Repository**

```bash
git clone <repository_url>
cd parkir
```

2.  **Konfigurasi Database**

Saat ini, konfigurasi database masih bersifat hardcoded di dalam file `src/main.rs`. Pastikan kredensial database lokal Anda sesuai atau ubah file tersebut:

```rust
// src/main.rs
let conn: DbConn = DbConn {
    host: "localhost".to_string(),
    port: 5432,
    user: "postgres".to_string(), // Sesuaikan user
    pass: "nokiae90".to_string(), // Sesuaikan password
    dbname: "parkir".to_string(), // Sesuaikan nama database
    schema: "public".to_string(),
};
```

3.  **Setup Database**

Pastikan database `parkir` telah dibuat dan memiliki tabel-tabel berikut:

- `m_tarif` (Kolom: `id`, `tahun`, `awal`, `kelipatan`, `jenis`)
- `m_juru_parkir` (Kolom: `id`, `nama`, `no_ktp`, `alamat`, `no_hp`, `unique_key`)
- `m_parkir` (Kolom: `id_parkir`, `lokasi`, `lat`, `lon`, `unique_key`)

## Menjalankan Aplikasi

Untuk menjalankan aplikasi dalam mode development:

```bash
cargo run
```

Server akan berjalan secara default di `http://0.0.0.0:8080`.

## Dokumentasi API

Berikut adalah daftar endpoint yang tersedia:

### 1. Landing Page
- **URL**: `/`
- **Method**: `GET`
- **Response**: HTML (Sistim Pengelolaan Parkir Digital)

### 2. About Page
- **URL**: `/about`
- **Method**: `GET`
- **Response**: HTML (About Info)

### 3. Get Tarif (Tarif Parkir)
Mengambil data tarif parkir berdasarkan tahun.

- **URL**: `/tarif/{tahun}`
- **Method**: `GET`
- **Parameter URL**:
    - `tahun` (integer): Tahun tarif (contoh: 2024)
- **Contoh Request**: `GET /tarif/2024`
- **Response**: JSON `ResponseData`

```json
{
  "status": true,
  "msg": "Sukses",
  "data": [
    {
      "id": 1,
      "awal": 2000,
      "kelipatan": 1000,
      "jenis": "Roda 2"
    },
    ...
  ]
}
```

### 4. Get Jukir (Juru Parkir)
Mengambil daftar semua juru parkir.

- **URL**: `/jukir`
- **Method**: `GET`
- **Response**: JSON `ResponseData`

```json
{
  "status": true,
  "msg": "Sukses",
  "data": [
    {
      "id": 101,
      "nama": "Budi",
      "no_ktp": "1234567890",
      "alamat": "Jl. Contoh No. 1",
      "no_hp": "08123456789",
      "unique_key": "KEY123"
    },
    ...
  ]
}
```

### 5. Get Lokasi Parkir
Mengambil daftar semua lokasi parkir.

- **URL**: `/parkir`
- **Method**: `GET`
- **Response**: JSON `ResponseData`

```json
{
  "status": true,
  "msg": "Sukses",
  "data": [
    {
      "id_parkir": 1,
      "lokasi": "Pasar Besar",
      "lat": -6.1234,
      "lon": 106.1234,
      "unique_key": "LOC123"
    },
    ...
  ]
}
    ...
  ]
}

#### Buat Parkir
- **URL**: `/parkir/buat`
- **Method**: `POST`
- **Body**: JSON
```json
{
  "lokasi": "Pasar Baru",
  "lat": -6.1234,
  "lon": 106.1234
}
```
- **Response**: JSON `ResponseEmpty`
- **Catatan**: `unique_key` akan di-generate otomatis oleh server.

#### Ubah Parkir
- **URL**: `/parkir/ubah`
- **Method**: `POST`
- **Body**: JSON
```json
{
  "id_parkir": 1,
  "lokasi": "Pasar Baru Update",
  "lat": -6.1234,
  "lon": 106.1234
}
```
- **Response**: JSON `ResponseEmpty`

#### Hapus Parkir
- **URL**: `/parkir/hapus`
- **Method**: `POST`
- **Body**: JSON
```json
{
  "id_parkir": 1
}
```
- **Response**: JSON `ResponseEmpty`

### 6. Transaksi Data
Endpoint untuk mengelola data transaksi.

#### Get Transaksi
- **URL**: `/transaksi`
- **Method**: `GET`
- **Response**: JSON `ResponseData` (List Transaksi)

#### Buat Transaksi
- **URL**: `/transaksi/buat`
- **Method**: `POST`
- **Body**: JSON
```json
{
  "start": "2023-10-27T10:00:00",
  "end": null,
  "id_user": 1,
  "id_jukir": 2,
  "amount": 5000.0,
  "id_parkir": 1
}
```
- **Response**: JSON `ResponseEmpty`

#### Ubah Transaksi
- **URL**: `/transaksi/ubah`
- **Method**: `POST`
- **Body**: JSON (sama dengan buat, tambah field `id`)
```json
{
  "id": 101,
  "start": "2023-10-27T10:00:00",
  ...
}
```
- **Response**: JSON `ResponseEmpty`

#### Hapus Transaksi
- **URL**: `/transaksi/hapus`
- **Method**: `POST`
- **Body**: JSON
```json
{
  "id": 101
}
```
- **Response**: JSON `ResponseEmpty`

## Struktur Project

```
parkir/
├── Cargo.toml          # File manifest dependensi Rust
├── src/
│   ├── main.rs         # Entry point aplikasi & konfigurasi server
│   ├── route.rs        # Definisi rute/endpoint
│   ├── dbpool.rs       # Setup connection pool database
│   ├── controllers/    # Logika handler untuk setiap endpoint
│   │   ├── mod.rs
│   │   ├── landing.rs  # Handler home & about
│   │   ├── tarif.rs    # Handler data tarif
│   │   ├── jukir.rs    # Handler data jukir
│   │   ├── parkir.rs   # Handler data lokasi parkir
│   │   └── response.rs # Struktur response standar (JSON)
│   └── models/         # Struktur data (struct) yang merepresentasikan tabel DB
│       ├── mod.rs
│       ├── tarif.rs
│       ├── jukir.rs
│       └── parkir.rs
```
