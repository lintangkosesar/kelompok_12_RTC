use std::f64::consts::PI;
mod taylor;
mod table;

use taylor::{sine_taylor, cosine_taylor};
use table::{init_lookup_table, sine_lookup, cosine_lookup};
use std::io;

fn main() {
    // Inisialisasi lookup table
    init_lookup_table();

    // Membaca input dari pengguna
    println!("Masukkan sudut dalam derajat:");
    let mut angle_deg = String::new();
    io::stdin()
        .read_line(&mut angle_deg)
        .expect("Gagal membaca input");
    
    // Konversi ke f64
    let angle_deg: f64 = angle_deg.trim().parse().expect("Harap masukkan angka!");
    let angle_rad = angle_deg * PI / 180.0;
    
    // Membaca jumlah terms untuk Taylor series
    println!("Masukkan jumlah terms untuk deret Taylor (default 10):");
    let mut terms_input = String::new();
    io::stdin()
        .read_line(&mut terms_input)
        .expect("Gagal membaca input");
    
    // Menggunakan default 10 jika input kosong
    let terms: usize = terms_input.trim().parse().unwrap_or(10);

    // Menghitung dengan berbagai metode
    let sin_t = sine_taylor(angle_rad, terms.try_into().unwrap());
    let cos_t = cosine_taylor(angle_rad, terms.try_into().unwrap());

    // Lookup table hanya bekerja untuk sudut integer
    let sin_lut = if angle_deg.fract() == 0.0 {
        sine_lookup(angle_deg as usize)
    } else {
        f64::NAN
    };
    
    let cos_lut = if angle_deg.fract() == 0.0 {
        cosine_lookup(angle_deg as usize)
    } else {
        f64::NAN
    };

    let sin_std = angle_rad.sin();
    let cos_std = angle_rad.cos();

    // Menampilkan hasil
    println!("\nHasil untuk sudut: {}° ({} rad)", angle_deg, angle_rad);
    println!("--- Deret Taylor ({} terms) ---", terms);
    println!("sin({}°) ≈ {}", angle_deg, sin_t);
    println!("cos({}°) ≈ {}", angle_deg, cos_t);

    println!("--- Lookup Table ---");
    if angle_deg.fract() == 0.0 {
        println!("sin({}°) ≈ {}", angle_deg, sin_lut);
        println!("cos({}°) ≈ {}", angle_deg, cos_lut);
    } else {
        println!("Lookup table hanya tersedia untuk sudut integer");
    }

    println!("--- Rust Stdlib ---");
    println!("sin({}°) = {}", angle_deg, sin_std);
    println!("cos({}°) = {}", angle_deg, cos_std);
}