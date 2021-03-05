use std::{collections::BTreeMap, error::Error, io, path::PathBuf, time::Duration};

use plotters::prelude::*;
use structopt::StructOpt;

use crate::record::Record;

const FONT: &str = "Fira Code";

#[derive(Debug, StructOpt)]
pub struct Options {
    // <dir>/<name>.throughput.svg
    // <dir>/<name>.latency.svg
    dir: PathBuf,
    name: String,
    #[structopt(short, long, default_value = "640")]
    width: u32,
    #[structopt(short, long, default_value = "480")]
    height: u32,
    #[structopt(long, default_value = "2000")]
    latency_limit_ns: u64,
}

pub fn plot(options: &Options) {
    let data = read_data();
    let data = group_data(data);
    plot_throughput(options, &data).expect("failed to plot throughput");
    plot_latency(options, &data).expect("failed to plot latency");
}

fn read_data() -> Vec<Record> {
    let rd = io::stdin();
    let mut rd = csv::Reader::from_reader(rd);

    rd.deserialize()
        .map(|result| result.expect("invalid record"))
        .collect()
}

type Groups = BTreeMap<String, Vec<Record>>;

fn group_data(records: Vec<Record>) -> Groups {
    let mut groups = Groups::new();

    for record in records {
        let group = groups.entry(record.name.clone()).or_insert_with(Vec::new);
        group.push(record);
    }

    groups
}

static COLORS: &[RGBColor] = &[BLUE, RED, GREEN, MAGENTA, CYAN, BLACK, YELLOW];

fn plot_throughput(options: &Options, groups: &Groups) -> Result<(), Box<dyn Error>> {
    let path = format!("{}/{}.throughput.svg", options.dir.display(), options.name);
    let resolution = (options.width, options.height);
    let root = SVGBackend::new(&path, resolution).into_drawing_area();

    root.fill(&WHITE)?;

    let (x_max, y_max) = groups
        .values()
        .flatten()
        .map(|record| (record.threads, record.throughput))
        .fold((0, 0f64), |res, cur| (res.0.max(cur.0), res.1.max(cur.1)));

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .caption(&format!("{}: Throughput", options.name), (FONT, 20))
        .set_label_area_size(LabelAreaPosition::Left, 70)
        .set_label_area_size(LabelAreaPosition::Right, 70)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(1..x_max, 0.0..y_max)?;

    chart
        .configure_mesh()
        .disable_y_mesh()
        .x_label_formatter(&|v| format!("{}", v))
        .y_label_formatter(&|v| format!("{:.0} Mop/s", v / 1_000_000.))
        .x_labels(20)
        .y_desc("Throughput")
        .x_desc("Threads")
        .draw()?;

    let colors = COLORS.iter().cycle();

    for (records, color) in groups.values().zip(colors) {
        chart
            .draw_series(LineSeries::new(
                records
                    .iter()
                    .map(|record| (record.threads, record.throughput)),
                color,
            ))?
            .label(&records[0].name)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
    }

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .label_font((FONT, 13))
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

fn plot_latency(options: &Options, groups: &Groups) -> Result<(), Box<dyn Error>> {
    let path = format!("{}/{}.latency.svg", options.dir.display(), options.name);
    let resolution = (options.width, options.height);
    let root = SVGBackend::new(&path, resolution).into_drawing_area();

    root.fill(&WHITE)?;

    let (x_max, y_max) = groups
        .values()
        .flatten()
        .map(|record| (record.threads, record.latency))
        .fold((0, Duration::from_secs(0)), |res, cur| {
            (res.0.max(cur.0), res.1.max(cur.1))
        });

    let y_max = options.latency_limit_ns.min(y_max.as_nanos() as u64);

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .caption(&format!("{}: Latency", options.name), (FONT, 20))
        .set_label_area_size(LabelAreaPosition::Left, 70)
        .set_label_area_size(LabelAreaPosition::Right, 70)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(1..x_max, 0..y_max)?;

    chart
        .configure_mesh()
        .disable_y_mesh()
        .x_label_formatter(&|v| format!("{}", v))
        .y_label_formatter(&|v| format!("{:.0} ns", v))
        .x_labels(20)
        .y_labels(20)
        .y_desc("Latency")
        .x_desc("Threads")
        .draw()?;

    let colors = COLORS.iter().cycle();

    for (records, color) in groups.values().zip(colors) {
        chart
            .draw_series(LineSeries::new(
                records
                    .iter()
                    .map(|record| (record.threads, record.latency.as_nanos() as u64)),
                color,
            ))?
            .label(&records[0].name)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
    }

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .label_font((FONT, 13))
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
