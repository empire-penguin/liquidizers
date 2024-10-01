use crate::{Error, Result};

pub enum AgcSquelchType {
    AgcSquelchUnknown,
    AgcSquelchEnabled,
    AgcSquelchRise,
    AgcSquelchSignalHi,
    AgcSquelchFall,
    AgcSquelchSignalLo,
    AgcSquelchTimeout,
    AgcSquelchDisabled,
}

pub trait Agc {
    fn new() -> Self;
    fn print(&self) -> Result<()>;
    fn reset(&self) -> Result<()>;
    fn execute(&self, x: num_complex::Complex<f32>) -> num_complex::Complex<f32>;
    fn execute_block(&self, x: &[num_complex::Complex<f32>]) -> Vec<num_complex::Complex<f32>>;
    fn lock(&self) -> Result<()>;
    fn unlock(&self) -> Result<()>;
    fn is_locked(&self) -> bool;
    fn get_bandwidth(&self) -> f32;
    fn set_bandwidth(&self, bandwidth: f32) -> Result<()>;
    fn get_signal_level(&self) -> f32;
    fn set_signal_level(&self, signal_level: f32) -> Result<()>;
    fn get_rssi(&self) -> f32;
    fn set_rssi(&self, rssi: f32) -> Result<()>;
    fn get_gain(&self) -> f32;
    fn set_gain(&self, gain: f32) -> Result<()>;
    fn get_scale(&self) -> f32;
    fn set_scale(&self, scale: f32) -> Result<()>;
    fn init(&self) -> Result<()>;
    fn squelch_enable() -> Result<()>;
    fn squelch_disable() -> Result<()>;
    fn squelch_is_enabled() -> bool;
    fn squelch_set_threshold(&self, threshold: f32) -> Result<()>;
    fn squelch_get_threshold(&self) -> f32;
    fn squelch_set_timeout(&self, timeout: usize) -> Result<()>;
    fn squelch_get_timeout(&self) -> usize;
    fn squelch_get_status(&self) -> AgcSquelchType;
    fn squelch_update_mode(&self) -> Result<()>;
}

#[derive(Debug)]
pub struct AgcCrcf {
    pub(crate) raw: liquidizers_sys::agc_crcf,
}

// pub fn set_bandwidth(&self, bandwidth: f32) -> Result<()> {
//     match Error::from_raw(unsafe {
//         liquidizers_sys::agc_crcf_set_bandwidth(self.raw, bandwidth)
//     }) {
//         Error::LiquidOk => Ok(()),
//         e => Err(e),
//     }
// }

impl Agc for AgcCrcf {
    fn new() -> Self {
        let raw = unsafe { liquidizers_sys::agc_crcf_create() };
        Self { raw }
    }

    fn print(&self) -> Result<()> {
        match Error::from_raw(unsafe { liquidizers_sys::agc_crcf_print(self.raw) }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn reset(&self) -> Result<()> {
        match Error::from_raw(unsafe { liquidizers_sys::agc_crcf_reset(self.raw) }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn execute(&self, x: num_complex::Complex<f32>) -> num_complex::Complex<f32> {
        let mut y: liquidizers_sys::__BindgenComplex<f32> = Default::default();
        let x_bg = liquidizers_sys::__BindgenComplex::<f32> { re: x.re, im: x.im };
        unsafe { liquidizers_sys::agc_crcf_execute(self.raw, x_bg, &mut y) };
        num_complex::Complex::new(y.re, y.im)
    }

    fn execute_block(&self, x: &[num_complex::Complex<f32>]) -> Vec<num_complex::Complex<f32>> {
        let mut x_bg: Vec<liquidizers_sys::__BindgenComplex<f32>> = x
            .iter()
            .map(|c| liquidizers_sys::__BindgenComplex::<f32> { re: c.re, im: c.im })
            .collect();

        let mut y_bg: Vec<liquidizers_sys::__BindgenComplex<f32>> =
            vec![Default::default(); x.len()];

        unsafe {
            liquidizers_sys::agc_crcf_execute_block(
                self.raw,
                x_bg.as_mut_ptr(),
                x.len() as u32,
                y_bg.as_mut_ptr(),
            )
        };
        y_bg.iter()
            .map(|c| num_complex::Complex::new(c.re, c.im))
            .collect()
    }

    fn lock(&self) -> Result<()> {
        match Error::from_raw(unsafe { liquidizers_sys::agc_crcf_lock(self.raw) }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }
}

impl Clone for AgcCrcf {
    fn clone(&self) -> Self {
        Self {
            raw: unsafe { liquidizers_sys::agc_crcf_copy(self.raw) },
        }
    }
}

impl Drop for AgcCrcf {
    fn drop(&mut self) {
        match Error::from_raw(unsafe { liquidizers_sys::agc_crcf_destroy(self.raw) }) {
            Error::LiquidOk => (),
            _ => panic!("agc_crcf_destroy failed"),
        }
    }
}

pub struct AgcRrrf {
    pub(crate) raw: liquidizers_sys::agc_rrrf,
}

impl AgcRrrf {
    pub fn new() -> Self {
        let raw = unsafe { liquidizers_sys::agc_rrrf_create() };
        Self { raw }
    }
}

impl Drop for AgcRrrf {
    fn drop(&mut self) {
        match Error::from_raw(unsafe { liquidizers_sys::agc_rrrf_destroy(self.raw) }) {
            Error::LiquidOk => (),
            _ => panic!("agc_rrrf_destroy failed"),
        }
    }
}
