use linfa::prelude::*;
use linfa_svm::Svm;
use ndarray::{Array1, Array2};

pub fn train_svm(train_x: Array2<f64>, train_y: Array1<f64>, test_x: Array2<f64>) -> Array1<f64> {
    // Gabungkan fitur dan label menjadi satu Dataset
    let dataset = Dataset::new(train_x, train_y);

    // Latih model SVM
    let model = Svm::params()
        .gaussian_kernel(100.0)
        .fit(&dataset)
        .expect("Gagal melatih SVM");

    // Prediksi label dari data uji
    model.predict(&test_x) // <-- Tidak perlu .expect
}
