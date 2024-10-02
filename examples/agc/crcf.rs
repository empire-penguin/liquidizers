use liquidizers::{
    agc::{self, Agc},
    Complex, Result,
};
use std::io::Write;

const OUTPUT_FILENAME: &str = "agc_crcf_example.m";

pub fn crcf_main(num_samples: usize, bandwidth: f32) -> Result<()> {
    let gamma = 0.001;

    // validate input
    if bandwidth < 0.0 {
        eprintln!("error: bandwidth must be positive");
        return Err(liquidizers::Error::LiquidEirange);
    } else if num_samples == 0 {
        eprintln!("error: number of samples must be greater than zero");
        return Err(liquidizers::Error::LiquidEirange);
    }

    let q = agc::AgcCrcf::new();
    q.set_bandwidth(bandwidth).expect("error setting bandwidth");
    // q.set_scale(0.5f32);

    let mut x = vec![Complex::new(0.0, 0.0); num_samples];
    let mut y = vec![Complex::new(0.0, 0.0); num_samples];

    let mut rssi = vec![0.0; num_samples];

    q.print().expect("error printing AGC object");

    // generate input signal
    for i in 0..num_samples {
        x[i] = gamma * Complex::new(0.0, 2.0 * std::f32::consts::PI * 0.0193 * i as f32).exp();
    }

    for i in 0..num_samples {
        y[i] = q.execute(x[i]);
        rssi[i] = q.get_rssi();
    }

    drop(q);

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
        write!(writer, "rssi({:4})  = {:12.4e};\n", i + 1, rssi[i]).unwrap();
    }

    write!(writer, "\n").unwrap();

    // plot results
    write!(writer, "figure;\n").unwrap();
    write!(writer, "t = 0:(n-1);\n").unwrap();
    write!(writer, "subplot(3,1,1);\n").unwrap();
    write!(writer, "  plot(t, real(x), '-', 'Color',[0 0.2 0.5])\n").unwrap();
    write!(writer, "  hold on;\n").unwrap();
    write!(writer, "  plot(t, imag(x), '-', 'Color',[0 0.5 0.2]);\n").unwrap();
    write!(writer, "  hold off;\n").unwrap();
    write!(writer, "  grid on;\n").unwrap();
    write!(writer, "  xlabel('sample index');\n").unwrap();
    write!(writer, "  ylabel('input');\n").unwrap();
    write!(writer, "subplot(3,1,2);\n").unwrap();
    write!(writer, "  plot(t, real(y), '-', 'Color',[0 0.2 0.5]);\n").unwrap();
    write!(writer, "  hold on;\n").unwrap();
    write!(writer, "  plot(t, imag(y), '-', 'Color',[0 0.5 0.2]);\n").unwrap();
    write!(writer, "  hold off;\n").unwrap();
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
