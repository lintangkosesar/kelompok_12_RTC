use std::f64::consts::PI;
use std::ffi::c_int;

// Tambahkan deklarasi mod ini!
mod taylor;
mod table;

// Fungsi internal (tidak diekspor)
use taylor::{sine_taylor, cosine_taylor};
use table::{init_lookup_table, sine_lookup, cosine_lookup};

/// Initialize library - harus diekspor untuk C
#[unsafe(no_mangle)]
pub extern "C" fn init() {
    init_lookup_table();
}

/// Fungsi utama untuk kalkulasi trigonometri
#[unsafe(no_mangle)]
pub extern "C" fn calculate_trigonometry(
    angle_deg: f64,
    terms: c_int,
    sin_t: *mut f64,
    cos_t: *mut f64,
    sin_lut: *mut f64,
    cos_lut: *mut f64,
    sin_std: *mut f64,
    cos_std: *mut f64
) {
    let angle_rad = angle_deg * PI / 180.0;

    // Taylor series
    let st = sine_taylor(angle_rad, terms);
    let ct = cosine_taylor(angle_rad, terms);

    // Lookup table
    let (sl, cl) = if angle_deg.fract() == 0.0 {
        (sine_lookup(angle_deg as usize), cosine_lookup(angle_deg as usize))
    } else {
        (f64::NAN, f64::NAN)
    };

    // Standard lib
    let ss = angle_rad.sin();
    let cs = angle_rad.cos();

    // Write ke pointer output
    unsafe {
        *sin_t = st;
        *cos_t = ct;
        *sin_lut = sl;
        *cos_lut = cl;
        *sin_std = ss;
        *cos_std = cs;
    }
}
