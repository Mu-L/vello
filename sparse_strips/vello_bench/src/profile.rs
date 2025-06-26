// Copyright 2025 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::env;
use std::time::Instant;
use vello_common::flatten;
use vello_common::kurbo::Stroke;
use vello_cpu::kurbo::StrokeCtx;

fn main() {
    let args: Vec<String> = env::args().collect();
    let iterations = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(1000)
    } else {
        1000
    };

    let data_items = vello_bench::data::get_data_items();
    
    // Use the first file by default (Ghostscript Tiger)
    let selected_file = if args.len() > 2 {
        let file_name = &args[2];
        data_items.iter().find(|item| item.name.contains(file_name))
    } else {
        data_items.first()
    };

    let item = selected_file.expect("No data files found");
    
    println!("Profiling stroke expansion for: {}", item.name);
    println!("Running {} iterations", iterations);
    println!("Number of stroked paths: {}", item.strokes.len());

    let mut total_duration = std::time::Duration::ZERO;

    for _ in 0..iterations {
        let start = Instant::now();
        let mut ctx = StrokeCtx::new();
        
        for path in &item.strokes {
            let stroke = Stroke {
                width: path.stroke_width as f64,
                ..Default::default()
            };
            let _expanded = flatten::expand_stroke(path.path.iter(), &stroke, &mut ctx, 0.25);
        }
        
        total_duration += start.elapsed();
    }

    let avg_duration = total_duration / iterations as u32;
    println!("\nAverage time per iteration: {:?}", avg_duration);
    println!("Total time: {:?}", total_duration);
}