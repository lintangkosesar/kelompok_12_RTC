use linfa::prelude::*;
use linfa_logistic::MultiLogisticRegression;
use ndarray::{Array1, Array2, s};
use plotters::prelude::*;

pub fn plot_svm_neighbors(
    train: &Array2<f64>,
    train_labels: &Array1<usize>,
    test: &Array2<f64>,
    test_label: usize,
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let caption = format!(
        "SVM Decision Boundary (Predicted: {})",
        match test_label {
            0 => "BAIK",
            1 => "SEDANG",
            2 => "TIDAK SEHAT",
            _ => "Unknown",
        }
    );

    // Gunakan dua fitur pertama untuk visualisasi
    let x_min = train.column(0).iter().chain(test.column(0).iter()).fold(f64::INFINITY, |a, &b| a.min(b));
    let x_max = train.column(0).iter().chain(test.column(0).iter()).fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let y_min = train.column(1).iter().chain(test.column(1).iter()).fold(f64::INFINITY, |a, &b| a.min(b));
    let y_max = train.column(1).iter().chain(test.column(1).iter()).fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    let root = BitMapBackend::new(file_name, (1000, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(&caption, ("sans-serif", 30).into_font())
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
    let dataset = Dataset::new(two_feature_train.clone(), train_labels.clone())
        .with_feature_names(vec!["pm10", "so2"]);

    let model = MultiLogisticRegression::default()
        .max_iterations(100)
        .fit(&dataset)?;

    for i in 0..grid_size {
        for j in 0..grid_size {
            let x = x_min + i as f64 * x_step;
            let y = y_min + j as f64 * y_step;

            let point = Array2::from_shape_vec((1, 2), vec![x, y])?;
            let prediction = model.predict(&point)[0];

            let color = match prediction {
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

    // Plot data training
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
                        ("sans-serif", 15).into_font(),
                    )
            },
        ))?;
    }

    // Plot titik uji
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
                + Text::new("X", (0, 15), ("sans-serif", 20).into_font())
        },
    ))?;

    // Tambahkan legenda
    chart.draw_series(vec![
        ("BAIK", GREEN),
        ("SEDANG", RED),
        ("TIDAK SEHAT", BLUE),
        ("Test Point", BLACK),
    ]
    .into_iter()
    .enumerate()
    .map(|(i, (label, color))| {
        let y_offset = 0.1 + i as f64 * 0.05;
        Text::new(
            label,
            (x_max - (x_max - x_min) * 0.3, y_max - (y_max - y_min) * y_offset),
            ("sans-serif", 15).into_font().color(&color),
        )
    }))?;

    root.present()?;
    println!("📊 Plot disimpan di '{}'", file_name);

    Ok(())
}
