use liquidizers::{
    agc::{Agc, AgcCrcf, AgcSquelchType},
    Complex, Error, Result,
};
use std::{f32::consts::PI, io::Write};

const OUTPUT_FILENAME: &str = "agc_crcf_squelch_example.m";

pub fn squelch_main(bandwidth: f32) -> Result<()> {
    // validate input
    if bandwidth < 0.0 {
        eprintln!("error: bandwidth must be positive");
        return Err(Error::LiquidEirange);
    }

    let q = AgcCrcf::new();
    q.set_bandwidth(bandwidth).expect("error setting bandwidth");
    q.set_signal_level(1e-3)
        .expect("error setting signal level");

    q.squelch_enable().expect("error enabling squelch");
    q.squelch_set_threshold(-50.0)
        .expect("error setting squelch threshold");
    q.squelch_set_timeout(100)
        .expect("error setting squelch timeout");

    let num_samples = 2000;
    let mut x = vec![Complex::new(0.0, 0.0); num_samples];
    let mut y = vec![Complex::new(0.0, 0.0); num_samples];
    let mut rssi = vec![0.0; num_samples];
    let mut mode = vec![AgcSquelchType::AgcSquelchUnknown; num_samples];

    // print info
    println!(
        "automatic gain control // loop bandwidth: {:12.4e}",
        bandwidth
    );

    // generate signal, applyig tapering window appropriately
    for i in 0..num_samples {
        let mut gamma = 0.0;
        match i {
            i if i < 500 => {
                gamma = 1e-3;
            }
            i if i < 550 => {
                gamma = 1e-3 + (1e-2 - 1e-3) * (0.5 - 0.5 * (PI * (i - 500) as f32 / 50.0).cos());
            }
            i if i < 1450 => gamma = 1e-2,
            i if i < 1500 => {
                gamma = 1e-3 + (1e-2 - 1e-3) * (0.5 + 0.5 * (PI * (i - 500) as f32 / 50.0).cos());
            }
            _ => {
                gamma = 1e-3;
            }
        }
        x[i] = gamma * Complex::new(0.0, 2.0 * PI * 0.0193 * i as f32).exp();
    }

    // run AGC
    for i in 0..num_samples {
        y[i] = q.execute(x[i]);
        rssi[i] = q.get_rssi();
        mode[i] = q.squelch_get_status();

        // print status every so often
        if i % 100 == 0
            || i == num_samples - 1
            || mode[i] == AgcSquelchType::AgcSquelchRise
            || mode[i] == AgcSquelchType::AgcSquelchFall
        {
            println!(
                "{:4} : {:18} ({:1}), rssi = {:8.2} dB",
                i,
                mode[i].to_string(),
                mode[i].clone() as u32,
                rssi[i]
            );
        }
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
        write!(writer, "mode({:4})  = {};\n", i + 1, mode[i].clone() as u32).unwrap();
    }

    write!(writer, "\n").unwrap();

    // plot results
    write!(writer, "figure('position',[100 100 1200 900]);\n").unwrap();
    write!(writer, "t = 0:(n-1);\n").unwrap();
    write!(writer, "subplot(4,1,1);\n").unwrap();
    write!(
        writer,
        "  plot(t, real(x), '-', 'LineWidth',1.2, 'Color',[0 0.2 0.5])\n"
    )
    .unwrap();
    write!(writer, "  hold on;\n").unwrap();
    write!(
        writer,
        "  plot(t, imag(x), '-', 'LineWidth',1.2, 'Color',[0 0.5 0.2]);\n"
    )
    .unwrap();
    write!(writer, "  hold off;\n").unwrap();
    write!(writer, "  grid on;\n").unwrap();
    write!(writer, "  xlabel('sample index');\n").unwrap();
    write!(writer, "  ylabel('input');\n").unwrap();
    write!(writer, "axis([0 {} -0.011 0.011]);\n", num_samples).unwrap();
    write!(writer, "subplot(4,1,2);\n").unwrap();
    write!(
        writer,
        "  plot(t, real(y), '-', 'LineWidth',1.2, 'Color',[0 0.2 0.5]);\n"
    )
    .unwrap();
    write!(writer, "  hold on;\n").unwrap();
    write!(
        writer,
        "  plot(t, imag(y), '-', 'LineWidth',1.2, 'Color',[0 0.5 0.2]);\n"
    )
    .unwrap();
    write!(writer, "  hold off;\n").unwrap();
    write!(writer, "  grid on;\n").unwrap();
    write!(writer, "  xlabel('sample index');\n").unwrap();
    write!(writer, "  ylabel('output');\n").unwrap();
    write!(writer, "subplot(4,1,3);\n").unwrap();
    write!(
        writer,
        "  plot(t,rssi,'-','LineWidth',1.2,'Color',[0.5 0 0]);\n"
    )
    .unwrap();
    write!(writer, "  grid on;\n").unwrap();
    write!(writer, "  xlabel('sample index');\n").unwrap();
    write!(writer, "  ylabel('rssi [dB]');\n").unwrap();
    write!(writer, "subplot(4,1,4);\n").unwrap();
    write!(
        writer,
        "  plot(t,mode,'-s','LineWidth',1.2,'MarkerSize',4,'Color',[0.5 0 0]);\n"
    )
    .unwrap();
    write!(writer, "  grid on;\n").unwrap();
    write!(writer, "  xlabel('sample index');\n").unwrap();
    write!(writer, "  ylabel('squelch mode');\n").unwrap();
    write!(writer, "axis([0 {} 0 8]);\n", num_samples).unwrap();

    println!("results written to {}", OUTPUT_FILENAME);

    println!("done.");
    return Ok(());
}
