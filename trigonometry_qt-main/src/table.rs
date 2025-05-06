// table.rs
use std::f64::consts::PI;

/// Lookup table untuk sin dan cos dengan presisi 1 derajat
pub const LOOKUP_DEGREE: usize = 361;
static mut SIN_TABLE: [f64; LOOKUP_DEGREE] = [0.0; LOOKUP_DEGREE];
static mut COS_TABLE: [f64; LOOKUP_DEGREE] = [0.0; LOOKUP_DEGREE];

/// Inisialisasi tabel sin dan cos
pub fn init_lookup_table() {
    for deg in 0..LOOKUP_DEGREE {
        let rad = (deg as f64) * PI / 180.0;
        unsafe {
            SIN_TABLE[deg] = rad.sin();
            COS_TABLE[deg] = rad.cos();
        }
    }
}

/// Mendapatkan sin dari derajat menggunakan lookup table
pub fn sine_lookup(deg: usize) -> f64 {
    unsafe { SIN_TABLE[deg % LOOKUP_DEGREE] }
}

/// Mendapatkan cos dari derajat menggunakan lookup table
pub fn cosine_lookup(deg: usize) -> f64 {
    unsafe { COS_TABLE[deg % LOOKUP_DEGREE] }
}