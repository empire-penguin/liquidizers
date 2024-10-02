use liquidizers::Result;
mod crcf;
mod qpsk;
mod rrrf;
mod squelch;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

const CRCF_ABOUT_TEXT: &str =
    "Automatic gain control example demonstrating its transient response.";
const QPSK_ABOUT_TEXT: &str = "Automatic gain control test for data signals with fluctuating signal levels. 
QPSK modulation introduces periodic random zero-crossings which gives instantaneous amplitude levels near zero. 
This example tests the response of the AGC to these types of signals.";
const SQUELCH_ABOUT_TEXT: &str =
    "Automatic gain control example demonstrating its transient response with squelch enabled.";
const RRRF_ABOUT_TEXT: &str = "Automatic gain control example demonstrating its transient response using floating-point data types.";

fn main() -> Result<()> {
    let main_prog = clap::Command::new("agc")
        .version(VERSION.unwrap_or("unknown"))
        .subcommands([
            clap::Command::new("crcf").about(CRCF_ABOUT_TEXT).args([
                clap::Arg::new("num_samples")
                    .short('n')
                    .long("num_samples")
                    .default_value("2000"),
                clap::Arg::new("bandwidth")
                    .short('b')
                    .long("bandwidth")
                    .default_value("0.01"),
            ]),
            clap::Command::new("qpsk").about(QPSK_ABOUT_TEXT),
            clap::Command::new("squelch")
                .about(SQUELCH_ABOUT_TEXT)
                .args([clap::Arg::new("bandwidth")
                    .short('b')
                    .long("bandwidth")
                    .default_value("0.25")]),
            clap::Command::new("rrrf").about(RRRF_ABOUT_TEXT).args([
                clap::Arg::new("num_samples")
                    .short('n')
                    .long("num_samples")
                    .default_value("2000"),
                clap::Arg::new("bandwidth")
                    .short('b')
                    .long("bandwidth")
                    .default_value("0.01"),
            ]),
        ]);

    let config = main_prog.clone().get_matches();

    if let Some(crcf_config) = config.subcommand_matches("crcf") {
        let num_samples = crcf_config
            .get_one::<String>("num_samples")
            .unwrap()
            .parse::<usize>()
            .expect("error parsing number of samples");

        let bandwidth = crcf_config
            .get_one::<String>("bandwidth")
            .unwrap()
            .parse::<f32>()
            .expect("error parsing bandwidth");

        crcf::crcf_main(num_samples, bandwidth)?;
    }

    if let Some(_) = config.subcommand_matches("qpsk") {
        qpsk::qpsk_main()?;
    }

    if let Some(squelch_config) = config.subcommand_matches("squelch") {
        let bandwidth = squelch_config
            .get_one::<String>("bandwidth")
            .unwrap()
            .parse::<f32>()
            .expect("error parsing bandwidth");

        squelch::squelch_main(bandwidth)?;
    }

    if let Some(rrrf_config) = config.subcommand_matches("rrrf") {
        let num_samples = rrrf_config
            .get_one::<String>("num_samples")
            .unwrap()
            .parse::<usize>()
            .expect("error parsing number of samples");

        let bandwidth = rrrf_config
            .get_one::<String>("bandwidth")
            .unwrap()
            .parse::<f32>()
            .expect("error parsing bandwidth");

        rrrf::rrrf_main(num_samples, bandwidth)?;
    }
    return Ok(());
}
