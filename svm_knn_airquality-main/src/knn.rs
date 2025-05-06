use ndarray::{Array1, Array2, Axis};
use crate::utils::euclidean_distance;
use std::collections::HashMap;

pub fn predict_knn(
    train: &Array2<f64>,
    train_labels: &Array1<usize>,
    test: &Array2<f64>,
    k: usize,
) -> Array1<usize> {
    let mut predictions = Array1::zeros(test.nrows());

    for (i, test_sample) in test.axis_iter(Axis(0)).enumerate() {
        let mut distances: Vec<_> = train
            .axis_iter(Axis(0))
            .enumerate()
            .map(|(j, train_sample)| {
                let dist = euclidean_distance(&train_sample.to_owned(), &test_sample.to_owned());
                (j, dist)
            })
            .collect();

        distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let mut counts = HashMap::new();
        for &(idx, _) in distances.iter().take(k) {
            let label = train_labels[idx];
            *counts.entry(label).or_insert(0) += 1;
        }

        let predicted_label = counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(label, _)| label)
            .unwrap_or(0);

        predictions[i] = predicted_label;
    }

    predictions
}
