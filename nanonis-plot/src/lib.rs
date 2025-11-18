use anyhow::Result;
use csv::StringRecord;
use nanonis_parse::DataPoint;
use plotters::prelude::*;
use plotters::style::RGBColor;

/// Plot style configuration
pub struct PlotStyle {
    pub width: u32,
    pub height: u32,
    pub log_y: bool,
    pub mesh: bool,
    pub line_width: u32,
    pub line_color: RGBColor,
}

impl Default for PlotStyle {
    fn default() -> Self {
        Self {
            width: 1024,
            height: 768,
            log_y: true,
            mesh: true,
            line_width: 4,
            line_color: RGBColor(83, 134, 245),
        }
    }
}

pub fn draw_curve(
    data: &[DataPoint],
    headers: &StringRecord,
    output_path: &str,
    style: &PlotStyle,
) -> Result<()> {
    // 1. Find Min/Max for plotting axis limits
    let x_min = data
        .iter()
        .map(|p| p.x)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0.0);
    let x_max = data
        .iter()
        .map(|p| p.x)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(1.0);
    let x_max_rounded = (x_max / 10.0).ceil() * 10.0;

    let y_min = data
        .iter()
        .map(|p| p.y)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0.0);
    let y_max = data
        .iter()
        .map(|p| p.y)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(1.0);

    // Log scale adjustments
    let y_min_log = if y_min <= 0.0 { 1e-10 } else { y_min * 0.5 };
    let y_max_log = y_max * 2.0;

    // 2. Create drawing area
    let root = BitMapBackend::new(output_path, (style.width, style.height)).into_drawing_area();
    root.fill(&WHITE)?;

    // 3. Fonts
    let title_font = ("arial", 42).into_font();
    let axis_label_font = ("arial", 40).into_font();
    let tick_label_font = ("arial", 32).into_font();

    // 4. Build chart with optional log Y
    let mut chart = ChartBuilder::on(&root)
        .caption("", title_font.clone()) // 使用標題字體
        .margin(50)
        .x_label_area_size(85) // 增加 X 軸標籤區域以容納更大字體
        .y_label_area_size(120) // 增加 Y 軸標籤區域以容納更大字體
        .build_cartesian_2d(
            // X-Axis (Frequency) - 線性座標
            x_min..x_max_rounded,
            // Y-Axis (Current PSD) - 對數座標
            (y_min_log..y_max_log).log_scale(),
        )?;

    // 5. Configure mesh
    let global_exp = y_max.log10().floor() as i32;
    let pow10 = 10f64.powi(global_exp);

    if style.mesh {
        chart
            .configure_mesh()
            .x_desc(headers.get(0).unwrap_or("Frequency (Hz)"))
            .y_desc(headers.get(1).unwrap_or("Current PSD"))
            .set_all_tick_mark_size(5)
            .axis_style(BLACK.stroke_width(3))
            .axis_desc_style(axis_label_font.clone())
            .label_style(tick_label_font.clone())
            .y_label_formatter(&move |v: &f64| {
                let mantissa = *v / pow10;
                format!("{:.2}", mantissa)
            })
            .draw()?;
    } else {
        // hide mesh
        chart
            .configure_mesh()
            .disable_mesh()
            .x_desc(headers.get(0).unwrap_or("Frequency (Hz)"))
            .y_desc(headers.get(1).unwrap_or("Current PSD"))
            .label_style(tick_label_font.clone())
            .draw()?;
    }

    // 6. Draw exponent manually
    let offset_x: i32 = 35;
    let offset_y: i32 = -8;
    let base_text = (165, 60);
    root.draw(&Text::new("×10", base_text, ("arial", 26).into_font()))?;
    root.draw(&Text::new(
        global_exp.to_string(),
        (base_text.0 + offset_x, base_text.1 + offset_y),
        ("arial", 20).into_font(),
    ))?;

    // 7. Draw line series
    chart.draw_series(LineSeries::new(
        data.iter().map(|p| (p.x, p.y)),
        style.line_color.stroke_width(style.line_width),
    ))?;

    // 8. Draw border
    let drawing_area = chart.plotting_area();
    let (x_pixel_range, y_pixel_range) = drawing_area.get_pixel_range();
    for ((x0, y0), (x1, y1)) in [
        (
            (x_pixel_range.start, y_pixel_range.start),
            (x_pixel_range.end, y_pixel_range.start),
        ),
        (
            (x_pixel_range.end, y_pixel_range.start),
            (x_pixel_range.end, y_pixel_range.end),
        ),
        (
            (x_pixel_range.end, y_pixel_range.end),
            (x_pixel_range.start, y_pixel_range.end),
        ),
        (
            (x_pixel_range.start, y_pixel_range.end),
            (x_pixel_range.start, y_pixel_range.start),
        ),
    ] {
        root.draw(&PathElement::new(
            vec![(x0, y0), (x1, y1)],
            BLACK.stroke_width(3),
        ))?;
    }

    root.present()?;
    println!("✅ Plot saved to {}", output_path);
    Ok(())
}
