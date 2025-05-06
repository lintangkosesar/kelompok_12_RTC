use csv::ReaderBuilder;
use ndarray::{Array1, Array2};
use std::fs::File;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn load_data(path: &str) -> Result<(Array2<f64>, Array1<usize>), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut records: Vec<_> = rdr.records().collect::<Result<_, _>>()?;
    records.shuffle(&mut thread_rng());

    let mut features = Vec::new();
    let mut labels = Vec::new();

    for record in &records {
        let feat: Vec<f64> = (0..5)
            .map(|i| record[i].trim().parse::<f64>().unwrap_or(0.0))
            .collect();
        features.push(feat);

        let label = match record[5].trim() {
            "BAIK" => 0,
            "SEDANG" => 1,
            "TIDAK SEHAT" => 2,
            _ => 0,
        };
        labels.push(label);
    }

    Ok((
        Array2::from_shape_vec((features.len(), 5), features.concat())?,
        Array1::from_vec(labels),
    ))
}
