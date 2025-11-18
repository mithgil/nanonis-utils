use anyhow::Result;
use nanonis_parse::read_nanonis_spectrum;
use nanonis_plot::{draw_curve, PlotStyle};
use std::path::Path;

//cargo run -p nanonis-plot --example semilogy_plot /home/tim/Documents/AS/data/nanonis2/20240920/Spectrum_cur001.dat
//home/tim/Documents/AS/data/nanonis2/20240920/Spectrum_cur001.dat
//
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: semilogy_plot <file.dat>");
        std::process::exit(1);
    }

    let input_path = Path::new(&args[1]);

    // === 1. 產出輸出圖檔路徑 ===
    let output_path = {
        let parent = input_path.parent().unwrap();
        let stem = input_path.file_stem().unwrap().to_string_lossy();
        parent.join(format!("{stem}.png"))
    };

    println!("Input : {}", input_path.display());
    println!("Output: {}", output_path.display());

    let (headers, data) = read_nanonis_spectrum(&input_path.to_str().unwrap())?;

    let style = PlotStyle {
        width: 1000,
        mesh: false,
        ..Default::default()
    };

    draw_curve(&data, &headers, output_path.to_str().unwrap(), &style)?;

    Ok(())
}
