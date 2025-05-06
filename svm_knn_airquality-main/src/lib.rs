// lib.rs
use linfa::prelude::*;
use linfa_logistic::MultiLogisticRegression;
use ndarray::{Array1, Array2, Axis, s};
use csv::ReaderBuilder;
use std::fs::File;
use rand::thread_rng;
use rand::seq::SliceRandom;
use plotters::prelude::*;

// Structure to hold the prediction results
#[derive(Debug)]
pub struct PredictionResult {
    pub svm_accuracy: f32,
    pub knn_accuracy: f32,
    pub svm_prediction: String,
    pub knn_prediction: String,
    pub plot_path: String,
    pub model_path: String,
}


// 1) Euclidean distance calculation
fn euclidean_distance(a: &Array1<f64>, b: &Array1<f64>) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

// 2) Create plot visualization
fn plot_svm_neighbors(
    train: &Array2<f64>,
    train_labels: &Array1<usize>,
    test: &Array2<f64>,
    test_label: usize,
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let caption = format!("SVM Decision Boundary (Predicted: {})", 
        match test_label {
            0 => "BAIK",
            1 => "SEDANG",
            2 => "TIDAK SEHAT",
            _ => "Unknown",
        }
    );

    let x_min = train.column(0).iter().chain(test.column(0).iter()).fold(f64::INFINITY, |a, &b| a.min(b));
    let x_max = train.column(0).iter().chain(test.column(0).iter()).fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let y_min = train.column(1).iter().chain(test.column(1).iter()).fold(f64::INFINITY, |a, &b| a.min(b));
    let y_max = train.column(1).iter().chain(test.column(1).iter()).fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    let root = BitMapBackend::new(file_name, (1000, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(caption.as_str(), ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart
        .configure_mesh()
        .x_desc("PM10 (Normalized)")
        .y_desc("SO2 (Normalized)")
        .draw()?;

    let grid_size = 100;
    let x_step = (x_max - x_min) / grid_size as f64;
    let y_step = (y_max - y_min) / grid_size as f64;

    let two_feature_train = train.slice(s![.., 0..2]).to_owned();
    let dataset = Dataset::new(two_feature_train, train_labels.clone())
        .with_feature_names(vec!["pm10", "so2"]);
    
    let svm_model = MultiLogisticRegression::default()
        .max_iterations(100)
        .fit(&dataset)?;

    for i in 0..grid_size {
        for j in 0..grid_size {
            let x = x_min + i as f64 * x_step;
            let y = y_min + j as f64 * y_step;
            
            let point = Array2::from_shape_vec((1, 2), vec![x, y])?;
            let pred = svm_model.predict(&point)[0];
            
            let color = match pred {
                0 => RGBColor(200, 255, 200),
                1 => RGBColor(255, 200, 200),
                2 => RGBColor(200, 200, 255),
                _ => WHITE,
            };
            
            chart.draw_series(std::iter::once(Rectangle::new(
                [(x, y), (x + x_step, y + y_step)],
                ShapeStyle::from(&color).filled(),
            )))?;
        }
    }

    for (i, label) in train_labels.iter().enumerate() {
        let color = match label {
            0 => GREEN.mix(0.7),
            1 => RED.mix(0.7),
            2 => BLUE.mix(0.7),
            _ => BLACK.into(),
        };
        
        chart.draw_series(PointSeries::of_element(
            vec![(train[[i, 0]], train[[i, 1]])],
            8,
            ShapeStyle::from(&color).filled(),
            &|coord, size, style| {
                EmptyElement::at(coord) 
                    + Circle::new((0, 0), size, style)
                    + Text::new(
                        match label {
                            0 => "B",
                            1 => "S",
                            2 => "T",
                            _ => "?",
                        },
                        (0, 10),
                        ("sans-serif", 15).into_font()
                    )
            },
        ))?;
    }

    let test_color = match test_label {
        0 => GREEN,
        1 => RED,
        2 => BLUE,
        _ => BLACK,
    };
    
    chart.draw_series(PointSeries::of_element(
        vec![(test[[0, 0]], test[[0, 1]])],
        15,
        ShapeStyle::from(&test_color).filled(),
        &|coord, size, style| {
            EmptyElement::at(coord) 
                + Circle::new((0, 0), size, style)
                + Text::new(
                    "X",
                    (0, 15),
                    ("sans-serif", 20).into_font()
                )
        },
    ))?;

    chart.draw_series(
        vec![
            ("BAIK", (x_max - (x_max-x_min)*0.3, y_max - (y_max-y_min)*0.1), GREEN),
            ("SEDANG", (x_max - (x_max-x_min)*0.3, y_max - (y_max-y_min)*0.15), RED),
            ("TIDAK SEHAT", (x_max - (x_max-x_min)*0.3, y_max - (y_max-y_min)*0.2), BLUE),
            ("Test Point", (x_max - (x_max-x_min)*0.3, y_max - (y_max-y_min)*0.25), BLACK),
        ]
        .into_iter()
        .map(|(label, pos, color)| {
            Text::new(
                label,
                pos,
                ("sans-serif", 15).into_font().color(&color),
            )
        }),
    )?;

    root.present()?;
    Ok(())
}

// 3) KNN prediction algorithm
fn knn_predict(
    train: &Array2<f64>,
    train_labels: &Array1<usize>,
    test: &Array2<f64>,
    k: usize,
) -> Array1<usize> {
    let mut predictions = Array1::zeros(test.nrows());

    for (i, test_sample) in test.axis_iter(Axis(0)).enumerate() {
        let distances = train
            .axis_iter(Axis(0))
            .map(|train_sample| euclidean_distance(&train_sample.to_owned(), &test_sample.to_owned()))
            .collect::<Vec<_>>();

        let mut nearest_indices = (0..distances.len()).collect::<Vec<_>>();
        nearest_indices.sort_by(|&a, &b| distances[a].partial_cmp(&distances[b]).unwrap());
        let nearest_indices = &nearest_indices[..k];

        let mut label_counts = std::collections::HashMap::new();
        for &idx in nearest_indices {
            let label = train_labels[idx];
            *label_counts.entry(label).or_insert(0) += 1;
        }

        let prediction = label_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(label, _)| label)
            .unwrap_or(0);

        predictions[i] = prediction;
    }

    predictions
}

// Main function that will be exposed to C/C++
#[unsafe(no_mangle)]
pub extern "C" fn analyze_air_quality(
    csv_path: *const libc::c_char,
    output_dir: *const libc::c_char,
    pm10: f64,
    so2: f64,
    co: f64,
    o3: f64,
    no2: f64,
) -> *mut PredictionResult {
    // Convert C strings to Rust strings
    let csv_path = unsafe { std::ffi::CStr::from_ptr(csv_path).to_string_lossy().into_owned() };
    let output_dir = unsafe { std::ffi::CStr::from_ptr(output_dir).to_string_lossy().into_owned() };

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&output_dir).unwrap();

    let plot_path = format!("{}/svm_neighbors.png", output_dir);
    let model_path = format!("{}/trained_model.bin", output_dir);

    let result = match analyze_air_quality_internal(&csv_path, &plot_path, &model_path, pm10, so2, co, o3, no2) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error: {}", e);
            return std::ptr::null_mut();
        }
    };

    Box::into_raw(Box::new(result))
}

// Internal analysis function
fn analyze_air_quality_internal(
    csv_path: &str,
    plot_path: &str,
    model_path: &str,
    pm10: f64,
    so2: f64,
    co: f64,
    o3: f64,
    no2: f64,
) -> Result<PredictionResult, Box<dyn std::error::Error>> {
    // 4) Read dataset
    let file = File::open(csv_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut records: Vec<_> = rdr.records().collect::<Result<_, _>>()?;
    let mut rng = thread_rng();
    records.shuffle(&mut rng);

    // 5) Features and Labels
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

    // 6) Convert to Array2 and Array1
    let mut features = Array2::from_shape_vec((features.len(), 5), features.concat())?;
    let labels = Array1::from_vec(labels);

    // Normalize Data
    for mut col in features.axis_iter_mut(ndarray::Axis(1)) {
        let mean = col.mean().unwrap();
        let std = col.std(0.0);
        col.iter_mut().for_each(|x| *x = (*x - mean) / std);
    }

    // 7) Split dataset 80-20
    let (train, test) = linfa::dataset::Dataset::new(features, labels)
        .split_with_ratio(0.8);

    // 8) Train SVM model
    let svm_model = MultiLogisticRegression::default()
        .max_iterations(100)
        .fit(&train)?;

    // Save the trained model
    let model_file = File::create(model_path)?;
    bincode::serialize_into(model_file, &svm_model)?;

    // 9) Predict and evaluate SVM model
    let svm_pred = svm_model.predict(&test);
    let svm_accuracy = svm_pred.iter()
        .zip(test.targets().iter())
        .filter(|(p, t)| p == t)
        .count() as f32 / test.nsamples() as f32;

    // 10) Plot SVM results
    let test_sample = test.records().slice(s![0..1, ..]).to_owned();
    let test_label = svm_pred[0];
    plot_svm_neighbors(
        train.records(),
        train.targets(),
        &test_sample,
        test_label,
        plot_path,
    )?;

    // 11) Evaluate KNN model
    let knn_pred = knn_predict(train.records(), train.targets(), test.records(), 5);
    let knn_accuracy = knn_pred.iter()
        .zip(test.targets().iter())
        .filter(|(p, t)| p == t)
        .count() as f32 / test.nsamples() as f32;

    // 15) Manual input prediction
    let manual_features = vec![pm10, so2, co, o3, no2];
    let mut manual_array = Array2::from_shape_vec((1, 5), manual_features)?;

    // Normalize manual input
    for (i, mut col) in manual_array.axis_iter_mut(ndarray::Axis(1)).enumerate() {
        let train_col = train.records().column(i);
        let mean = train_col.mean().unwrap();
        let std = train_col.std(0.0);
        col.iter_mut().for_each(|x| *x = (*x - mean) / std);
    }

    // 16) Predict with SVM
    let svm_manual_prediction = svm_model.predict(&manual_array);
    let svm_predicted_category = match svm_manual_prediction[0] {
        0 => "BAIK".to_string(),
        1 => "SEDANG".to_string(),
        2 => "TIDAK SEHAT".to_string(),
        _ => "Unknown".to_string(),
    };

    // 17) Predict with KNN
    let knn_manual_prediction = knn_predict(
        train.records(),
        train.targets(),
        &manual_array,
        5,
    );
    let knn_predicted_category = match knn_manual_prediction[0] {
        0 => "BAIK".to_string(),
        1 => "SEDANG".to_string(),
        2 => "TIDAK SEHAT".to_string(),
        _ => "Unknown".to_string(),
    };

    Ok(PredictionResult {
        svm_accuracy,
        knn_accuracy,
        svm_prediction: svm_predicted_category,
        knn_prediction: knn_predicted_category,
        plot_path: plot_path.to_string(),
        model_path: model_path.to_string(),
    })
}

// Function to free the PredictionResult
#[unsafe(no_mangle)]
pub extern "C" fn free_prediction_result(result: *mut PredictionResult) {
    if !result.is_null() {
        unsafe { let _ = Box::from_raw(result); };
    }
}