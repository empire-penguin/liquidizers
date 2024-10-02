use liquidizers::{modem, Complex, Result};

const OUTPUT_FILENAME: &str = "agc_crcf_qpsk_example.m";

pub fn qpsk_main() -> Result<()> {
    let noise_floor = -40.0; // noise floor [dB]
    let snr_db = 20.0; // signal-to-noise ratio [dB]
    let bt = 0.05; // loop bandwidth
    let num_symbols = 100; // number of symbols
    let d = 5; // print every d iterations

    let k = 2; // iterpolation factor (samples/symbol)
    let m = 3; // filter delay (symbols)
    let beta = 0.3; // filter excess bandwidth factor
    let dt = 0.0; // filter fractional sample delay

    // derived values
    let num_samples = num_symbols * k;
    let gamma = 10.0f32.powf((snr_db + noise_floor) / 20.0); // channel gain
    let nstd = 10.0f32.powf(noise_floor / 20.0);

    // arrays for input/output
    let mut x = vec![Complex::new(0.0, 0.0); num_samples];
    let mut y = vec![Complex::new(0.0, 0.0); num_samples];
    let mut rssi = vec![0.0; num_samples];

    let modem = modem::ModemCF::new(modem::ModulationScheme::Qpsk);
    // let interp = filter::FirInterCrcf::new();

    let mut sample = Complex::default();
    for i in 0..num_symbols {
        let sym = modem.gen_rand_sym();
        sample = modem.modulate(sym);
        sample *= gamma;

        // interp.execute(sample, &mut x[i * k]);
    }

    // for i in 0..num_samples {
    //     write!(
    //         writer,
    //         "x({:4}) = {:12.4e} + j*{:12.4e};\n",
    //         i + 1,
    //         x[i].re,
    //         x[i].im
    //     )
    //     .unwrap();
    //     write!(
    //         writer,
    //         "y({:4}) = {:12.4e} + j*{:12.4e};\n",
    //         i + 1,
    //         y[i].re,
    //         y[i].im
    //     )
    //     .unwrap();
    //     write!(writer, "rssi({:4})  = {:12.4e};\n", i + 1, rssi[i]).unwrap();
    // }

    // write!(writer, "\n").unwrap();

    // // plot results
    // write!(writer, "figure;\n").unwrap();
    // write!(writer, "t = 0:(n-1);\n").unwrap();
    // write!(writer, "subplot(3,1,1);\n").unwrap();
    // write!(writer, "  plot(t, real(x), '-', 'Color',[0 0.2 0.5])\n").unwrap();
    // write!(writer, "  hold on;\n").unwrap();
    // write!(writer, "  plot(t, imag(x), '-', 'Color',[0 0.5 0.2]);\n").unwrap();
    // write!(writer, "  hold off;\n").unwrap();
    // write!(writer, "  grid on;\n").unwrap();
    // write!(writer, "  xlabel('sample index');\n").unwrap();
    // write!(writer, "  ylabel('input');\n").unwrap();
    // write!(writer, "subplot(3,1,2);\n").unwrap();
    // write!(writer, "  plot(t, real(y), '-', 'Color',[0 0.2 0.5]);\n").unwrap();
    // write!(writer, "  hold on;\n").unwrap();
    // write!(writer, "  plot(t, imag(y), '-', 'Color',[0 0.5 0.2]);\n").unwrap();
    // write!(writer, "  hold off;\n").unwrap();
    // write!(writer, "  grid on;\n").unwrap();
    // write!(writer, "  xlabel('sample index');\n").unwrap();
    // write!(writer, "  ylabel('output');\n").unwrap();
    // write!(writer, "subplot(3,1,3);\n").unwrap();
    // write!(
    //     writer,
    //     "  plot(t,rssi,'-','LineWidth',1.2,'Color',[0.5 0 0]);\n"
    // )
    // .unwrap();
    // write!(writer, "  grid on;\n").unwrap();
    // write!(writer, "  xlabel('sample index');\n").unwrap();
    // write!(writer, "  ylabel('rssi [dB]');\n").unwrap();

    // println!("results written to {}", OUTPUT_FILENAME);

    // println!("done.");
    return Ok(());
}
