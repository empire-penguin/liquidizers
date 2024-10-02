use num_complex::Complex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModulationScheme {
    // Unknown Modulation Scheme
    Unknown,

    // Phase-shift keying (PSK)
    Psk2,
    Psk4,
    Psk8,
    Psk16,
    Psk32,
    Psk64,
    Psk128,
    Psk256,

    // Differential phase-shift keying (DPSK)
    Dpsk2,
    Dpsk4,
    Dpsk8,
    Dpsk16,
    Dpsk32,
    Dpsk64,
    Dpsk128,
    Dpsk256,

    // Amplitude-shift keying (ASK)
    Ask2,
    Ask4,
    Ask8,
    Ask16,
    Ask32,
    Ask64,
    Ask128,
    Ask256,

    // Rectangular quadrature amplitude modulation (QAM)
    Qam4,
    Qam8,
    Qam16,
    Qam32,
    Qam64,
    Qam128,
    Qam256,

    // Amplitude phase-shift keying (APSK)
    Apsk4,
    Apsk8,
    Apsk16,
    Apsk32,
    Apsk64,
    Apsk128,
    Apsk256,

    // Specific modem types
    Bpsk,
    Qpsk,
    Ook,
    Sqam32,
    Sqam128,
    V29,
    Arb16opt,
    Arb32opt,
    Arb64opt,
    Arb128opt,
    Arb64vt,
    Pi4Dqpsk,

    // arbitrary Qam
    Arb,
}

impl ModulationScheme {
    fn from_raw(raw: i32) -> Self {
        assert!(raw >= 0, "Negative return status: {}", raw);
        let status = raw as u32;
        match status {
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_UNKNOWN => ModulationScheme::Unknown,

            liquidizers_sys::modulation_scheme_LIQUID_MODEM_PSK2 => ModulationScheme::Psk2,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_PSK4 => ModulationScheme::Psk4,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_PSK8 => ModulationScheme::Psk8,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_PSK16 => ModulationScheme::Psk16,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_PSK32 => ModulationScheme::Psk32,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_PSK64 => ModulationScheme::Psk64,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_PSK128 => ModulationScheme::Psk128,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_PSK256 => ModulationScheme::Psk256,

            liquidizers_sys::modulation_scheme_LIQUID_MODEM_DPSK2 => ModulationScheme::Dpsk2,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_DPSK4 => ModulationScheme::Dpsk4,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_DPSK8 => ModulationScheme::Dpsk8,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_DPSK16 => ModulationScheme::Dpsk16,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_DPSK32 => ModulationScheme::Dpsk32,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_DPSK64 => ModulationScheme::Dpsk64,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_DPSK128 => ModulationScheme::Dpsk128,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_DPSK256 => ModulationScheme::Dpsk256,

            liquidizers_sys::modulation_scheme_LIQUID_MODEM_ASK2 => ModulationScheme::Ask2,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_ASK4 => ModulationScheme::Ask4,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_ASK8 => ModulationScheme::Ask8,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_ASK16 => ModulationScheme::Ask16,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_ASK32 => ModulationScheme::Ask32,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_ASK64 => ModulationScheme::Ask64,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_ASK128 => ModulationScheme::Ask128,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_ASK256 => ModulationScheme::Ask256,

            liquidizers_sys::modulation_scheme_LIQUID_MODEM_QAM4 => ModulationScheme::Qam4,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_QAM8 => ModulationScheme::Qam8,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_QAM16 => ModulationScheme::Qam16,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_QAM32 => ModulationScheme::Qam32,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_QAM64 => ModulationScheme::Qam64,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_QAM128 => ModulationScheme::Qam128,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_QAM256 => ModulationScheme::Qam256,

            liquidizers_sys::modulation_scheme_LIQUID_MODEM_APSK4 => ModulationScheme::Apsk4,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_APSK8 => ModulationScheme::Apsk8,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_APSK16 => ModulationScheme::Apsk16,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_APSK32 => ModulationScheme::Apsk32,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_APSK64 => ModulationScheme::Apsk64,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_APSK128 => ModulationScheme::Apsk128,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_APSK256 => ModulationScheme::Apsk256,

            liquidizers_sys::modulation_scheme_LIQUID_MODEM_BPSK => ModulationScheme::Bpsk,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_QPSK => ModulationScheme::Qpsk,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_OOK => ModulationScheme::Ook,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_SQAM32 => ModulationScheme::Sqam32,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_SQAM128 => ModulationScheme::Sqam128,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_V29 => ModulationScheme::V29,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_ARB16OPT => ModulationScheme::Arb16opt,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_ARB32OPT => ModulationScheme::Arb32opt,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_ARB64OPT => ModulationScheme::Arb64opt,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_ARB128OPT => {
                ModulationScheme::Arb128opt
            }
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_ARB64VT => ModulationScheme::Arb64vt,
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_PI4DQPSK => ModulationScheme::Pi4Dqpsk,

            // arbitrary Qam
            liquidizers_sys::modulation_scheme_LIQUID_MODEM_ARB => ModulationScheme::Arb,

            x => panic!("Invalid return status: {}", x),
        }
    }

    fn to_raw(&self) -> u32 {
        match self {
            ModulationScheme::Unknown => liquidizers_sys::modulation_scheme_LIQUID_MODEM_UNKNOWN,

            ModulationScheme::Psk2 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_PSK2,
            ModulationScheme::Psk4 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_PSK4,
            ModulationScheme::Psk8 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_PSK8,
            ModulationScheme::Psk16 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_PSK16,
            ModulationScheme::Psk32 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_PSK32,
            ModulationScheme::Psk64 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_PSK64,
            ModulationScheme::Psk128 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_PSK128,
            ModulationScheme::Psk256 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_PSK256,

            ModulationScheme::Dpsk2 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_DPSK2,
            ModulationScheme::Dpsk4 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_DPSK4,
            ModulationScheme::Dpsk8 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_DPSK8,
            ModulationScheme::Dpsk16 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_DPSK16,
            ModulationScheme::Dpsk32 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_DPSK32,
            ModulationScheme::Dpsk64 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_DPSK64,
            ModulationScheme::Dpsk128 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_DPSK128,
            ModulationScheme::Dpsk256 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_DPSK256,

            ModulationScheme::Ask2 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_ASK2,
            ModulationScheme::Ask4 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_ASK4,
            ModulationScheme::Ask8 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_ASK8,
            ModulationScheme::Ask16 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_ASK16,
            ModulationScheme::Ask32 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_ASK32,
            ModulationScheme::Ask64 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_ASK64,
            ModulationScheme::Ask128 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_ASK128,
            ModulationScheme::Ask256 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_ASK256,

            ModulationScheme::Qam4 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_QAM4,
            ModulationScheme::Qam8 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_QAM8,
            ModulationScheme::Qam16 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_QAM16,
            ModulationScheme::Qam32 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_QAM32,
            ModulationScheme::Qam64 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_QAM64,
            ModulationScheme::Qam128 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_QAM128,
            ModulationScheme::Qam256 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_QAM256,

            ModulationScheme::Apsk4 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_APSK4,
            ModulationScheme::Apsk8 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_APSK8,
            ModulationScheme::Apsk16 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_APSK16,
            ModulationScheme::Apsk32 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_APSK32,
            ModulationScheme::Apsk64 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_APSK64,
            ModulationScheme::Apsk128 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_APSK128,
            ModulationScheme::Apsk256 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_APSK256,

            ModulationScheme::Bpsk => liquidizers_sys::modulation_scheme_LIQUID_MODEM_BPSK,
            ModulationScheme::Qpsk => liquidizers_sys::modulation_scheme_LIQUID_MODEM_QPSK,
            ModulationScheme::Ook => liquidizers_sys::modulation_scheme_LIQUID_MODEM_OOK,
            ModulationScheme::Sqam32 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_SQAM32,
            ModulationScheme::Sqam128 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_SQAM128,
            ModulationScheme::V29 => liquidizers_sys::modulation_scheme_LIQUID_MODEM_V29,
            ModulationScheme::Arb16opt => liquidizers_sys::modulation_scheme_LIQUID_MODEM_ARB16OPT,
            ModulationScheme::Arb32opt => liquidizers_sys::modulation_scheme_LIQUID_MODEM_ARB32OPT,
            ModulationScheme::Arb64opt => liquidizers_sys::modulation_scheme_LIQUID_MODEM_ARB64OPT,
            ModulationScheme::Arb128opt => {
                liquidizers_sys::modulation_scheme_LIQUID_MODEM_ARB128OPT
            }
            ModulationScheme::Arb64vt => liquidizers_sys::modulation_scheme_LIQUID_MODEM_ARB64VT,
            ModulationScheme::Pi4Dqpsk => liquidizers_sys::modulation_scheme_LIQUID_MODEM_PI4DQPSK,

            ModulationScheme::Arb => liquidizers_sys::modulation_scheme_LIQUID_MODEM_ARB,
        }
    }
}

impl From<ModulationScheme> for liquidizers_sys::modulation_scheme {
    fn from(scheme: ModulationScheme) -> Self {
        scheme.to_raw() as liquidizers_sys::modulation_scheme
    }
}

pub struct ModemCF {
    pub(crate) raw: liquidizers_sys::modemcf,
}

impl ModemCF {
    pub fn new(scheme: ModulationScheme) -> Self {
        let raw = unsafe { liquidizers_sys::modemcf_create(scheme.into()) };
        Self { raw }
    }

    pub fn create_arbitrary(table: Vec<Complex<f32>>, m: u32) -> Self {
        let mut liquid_table = table
            .iter()
            .map(|c| liquidizers_sys::liquid_float_complex { re: c.re, im: c.im })
            .collect::<Vec<_>>();
        let raw =
            unsafe { liquidizers_sys::modemcf_create_arbitrary(liquid_table.as_mut_ptr(), m) };
        Self { raw }
    }

    pub fn recreate(&self, scheme: ModulationScheme) -> Self {
        let raw = unsafe { liquidizers_sys::modemcf_recreate(self.raw, scheme.into()) };
        Self { raw }
    }

    pub fn destroy(&self) {
        unsafe { liquidizers_sys::modemcf_destroy(self.raw) };
    }

    pub fn print(&self) {
        unsafe { liquidizers_sys::modemcf_print(self.raw) };
    }

    pub fn reset(&self) {
        unsafe { liquidizers_sys::modemcf_reset(self.raw) };
    }

    pub fn get_scheme(&self) -> ModulationScheme {
        ModulationScheme::from_raw(unsafe { liquidizers_sys::modemcf_get_scheme(self.raw) as i32 })
    }

    pub fn modulate(&self, symbol: u32) -> Complex<f32> {
        let mut y: liquidizers_sys::__BindgenComplex<f32> = Default::default();
        unsafe { liquidizers_sys::modemcf_modulate(self.raw, symbol, &mut y) };
        Complex::new(y.re, y.im)
    }

    pub fn gen_rand_sym(&self) -> u32 {
        unsafe { liquidizers_sys::modemcf_gen_rand_sym(self.raw) }
    }
}

impl Drop for ModemCF {
    fn drop(&mut self) {
        unsafe { liquidizers_sys::modemcf_destroy(self.raw) };
    }
}

impl Clone for ModemCF {
    fn clone(&self) -> Self {
        let raw = unsafe { liquidizers_sys::modemcf_copy(self.raw) };
        Self { raw }
    }
}
