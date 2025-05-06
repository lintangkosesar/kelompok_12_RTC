mod data;
mod knn;
mod svm;
mod plot;
mod utils;

use data::load_data;
use knn::predict_knn;
use svm::train_svm;
use plot::plot_svm_neighbors;
use ndarray::{s, Axis};
use ndarray::Array1;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (features, labels_usize) = load_data("csv/airquality.csv")?;

    // Split 80% train / 20% test
    let split_idx = (features.nrows() as f64 * 0.8) as usize;
    let (train_x, test_x) = features.view().split_at(Axis(0), split_idx);
    let (train_y_usize, test_y_usize) = labels_usize.view().split_at(Axis(0), split_idx);
    
    // 🟢 Konversi train_y dari usize -> f64 untuk SVM
    let train_y_f64: Array1<f64> = train_y_usize.to_owned().mapv(|x| x as f64);
    // 🟢 Konversi test_y untuk evaluasi SVM
    let _test_y_f64: Array1<f64> = test_y_usize.to_owned().mapv(|x| x as f64);

    // SVM
    let svm_preds = train_svm(train_x.to_owned(), train_y_f64, test_x.to_owned());
    println!("SVM Prediksi Label Pertama: {}", svm_preds[0] as usize);
    
    // Hitung akurasi SVM
    let svm_accuracy = calculate_accuracy(&svm_preds.mapv(|x| x as usize).to_owned(), &test_y_usize.to_owned());
    println!("SVM Akurasi: {:.2}%", svm_accuracy * 100.0);

    // KNN
    let knn_pred = predict_knn(&train_x.to_owned(), &train_y_usize.to_owned(), &test_x.to_owned(), 3);
    println!("KNN Prediksi Label Pertama: {}", knn_pred[0]);
    
    // Hitung akurasi KNN
    let knn_accuracy = calculate_accuracy(&knn_pred, &test_y_usize.to_owned());
    println!("KNN Akurasi: {:.2}%", knn_accuracy * 100.0);
    
    // Plot (menggunakan 2 fitur pertama)
    plot_svm_neighbors(
        &train_x.slice(s![.., 0..2]).to_owned(),
        &train_y_usize.to_owned(),
        &test_x.slice(s![0..1, 0..2]).to_owned(),
        svm_preds[0] as usize,
        "output/svm_plot.png"
    )?;

    Ok(())
}

// Fungsi untuk menghitung akurasi
fn calculate_accuracy(predictions: &Array1<usize>, true_labels: &Array1<usize>) -> f64 {
    let correct = predictions.iter()
        .zip(true_labels.iter())
        .filter(|(pred, true_val)| *pred == *true_val)
        .count();
    correct as f64 / predictions.len() as f64
}
