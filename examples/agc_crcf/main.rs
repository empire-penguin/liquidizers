use std::io::Write;

use liquidizers::{Complex, Result};

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const OUTPUT_FILENAME: &str = "agc_crcf_example.m";

fn main() -> Result<()> {
    let mut main_prog = clap::Command::new("agc_crcf")
        .version(VERSION.unwrap_or("unknown"))
        .about("Automatic gain control example demonstrating its transient response.")
        .args([
            clap::Arg::new("num_samples")
                .long("num_samples")
                .short('n')
                .help("number of samples, n >= 100\t")
                .default_value("2000"),
            clap::Arg::new("bandwidth")
                .long("bandwidth")
                .short('b')
                .help("AGC bandwidth, b >= 0\t")
                .default_value("0.01"),
        ]);

    let config = main_prog.clone().get_matches();
    if config.get_flag("help") {
        match main_prog.print_help() {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    let num_samples = config.get_one::<usize>("num_samples").unwrap().clone();
    let bandwidth = config.get_one::<f32>("bandwidth").unwrap().clone();
    let gamma = 0.001;

    // validate input
    if bandwidth < 0.0 {
        eprintln!("error: bandwidth must be positive");
        return Err(liquidizers::Error::LiquidEirange);
    } else if num_samples == 0 {
        eprintln!("error: number of samples must be greater than zero");
        return Err(liquidizers::Error::LiquidEirange);
    }

    let i: usize = 0;

    let q = liquidizers::agc_crcf_create();
    liquidizers::agc_crcf_set_bandwidth(q, bandwidth);
    // liquidizers::agc_crcf_set_scale(q, 0.5f32);

    let x = vec![Complex::new(0.0, 0.0); num_samples];
    let y = vec![Complex::new(0.0, 0.0); num_samples];

    let rssi = vec![0.0; num_samples];

    liquidizers::agc_crcf_print(q);

    for i in 0..num_samples {
        x[i] = gamma * Complex::new(0.0, 2.0 * std::f32::consts::PI * 0.0193 * i as f32).exp();
    }

    for i in 0..num_samples {
        let mut y_i = Complex::new(0.0, 0.0);
        liquidizers::agc_crcf_execute(q, x[i], &mut y_i);
        y[i] = y_i;

        rssi[i] = liquidizers::agc_crcf_get_rssi(q);
    }

    liquidizers::agc_crcf_destroy(q);

    let fid = std::fs::File::create(OUTPUT_FILENAME)
        .expect(format!("could not open file {} for writing", OUTPUT_FILENAME).as_str());
    let mut writer = std::io::BufWriter::new(fid);

    write!(writer, "%% {}: auto-generated file\n\n", OUTPUT_FILENAME).unwrap();
    write!(writer, "clear all;\nclose all;\n\n").unwrap();
    write!(writer, "n = {};\n", num_samples).unwrap();

    for i in 0..num_samples {
        write!(
            writer,
            "x({:4}) = {:12.4e} + j*{:12.4e};\n",
            i + 1,
            x[i].re,
            x[i].im
        )
        .unwrap();
        write!(
            writer,
            "y({:4}) = {:12.4e} + j*{:12.4e};\n",
            i + 1,
            y[i].re,
            y[i].im
        )
        .unwrap();
        write!(writer, "rssi({:4})  = {:12.4e};\n", i + 1, rssi[i])?;
    }

    write!(writer, "\n").unwrap();

    // plot results
    write!(writer, "figure;\n").unwrap();
    write!(writer, "t = 0:(n-1);\n").unwrap();
    write!(writer, "subplot(3,1,1);\n").unwrap();
    write!(writer, "  plot(t, real(x), '-', 'Color',[0 0.2 0.5],...\n").unwrap();
    write!(writer, "       t, imag(x), '-', 'Color',[0 0.5 0.2]);\n").unwrap();
    write!(writer, "  grid on;\n").unwrap();
    write!(writer, "  xlabel('sample index');\n").unwrap();
    write!(writer, "  ylabel('input');\n").unwrap();
    write!(writer, "subplot(3,1,2);\n").unwrap();
    write!(writer, "  plot(t, real(y), '-', 'Color',[0 0.2 0.5],...\n").unwrap();
    write!(writer, "       t, imag(y), '-', 'Color',[0 0.5 0.2]);\n").unwrap();
    write!(writer, "  grid on;\n").unwrap();
    write!(writer, "  xlabel('sample index');\n").unwrap();
    write!(writer, "  ylabel('output');\n").unwrap();
    write!(writer, "subplot(3,1,3);\n").unwrap();
    write!(
        writer,
        "  plot(t,rssi,'-','LineWidth',1.2,'Color',[0.5 0 0]);\n"
    )
    .unwrap();
    write!(writer, "  grid on;\n").unwrap();
    write!(writer, "  xlabel('sample index');\n").unwrap();
    write!(writer, "  ylabel('rssi [dB]');\n").unwrap();

    println!("results written to {}", OUTPUT_FILENAME);

    println!("done.");
    return Ok(());
}
