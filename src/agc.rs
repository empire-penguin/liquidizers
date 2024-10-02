use crate::{Error, Result};
use num_complex::Complex;
use std::fmt::{self, Display, Formatter};

pub trait Agc<T>: Clone {
    fn new() -> Self;
    fn print(&self) -> Result<()>;
    fn reset(&self) -> Result<()>;
    fn execute(&self, input: T) -> T;
    fn execute_block(&self, input: &[T]) -> Vec<T>;
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
    fn init(&self, input: &[T], size: u32) -> Result<()>;
    fn squelch_enable(&self) -> Result<()>;
    fn squelch_disable(&self) -> Result<()>;
    fn squelch_is_enabled(&self) -> bool;
    fn squelch_set_threshold(&self, threshold: f32) -> Result<()>;
    fn squelch_get_threshold(&self) -> f32;
    fn squelch_set_timeout(&self, timeout: u32) -> Result<()>;
    fn squelch_get_timeout(&self) -> u32;
    fn squelch_get_status(&self) -> AgcSquelchType;
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum AgcSquelchType {
    #[default]
    AgcSquelchUnknown,
    AgcSquelchEnabled,
    AgcSquelchRise,
    AgcSquelchSignalHi,
    AgcSquelchFall,
    AgcSquelchSignalLo,
    AgcSquelchTimeout,
    AgcSquelchDisabled,
}

impl AgcSquelchType {
    fn from_raw(raw: i32) -> Self {
        assert!(raw >= 0, "Negative return status: {}", raw);
        let status = raw as u32;
        match status {
            liquidizers_sys::agc_squelch_mode_LIQUID_AGC_SQUELCH_UNKNOWN => {
                AgcSquelchType::AgcSquelchUnknown
            }
            liquidizers_sys::agc_squelch_mode_LIQUID_AGC_SQUELCH_ENABLED => {
                AgcSquelchType::AgcSquelchEnabled
            }
            liquidizers_sys::agc_squelch_mode_LIQUID_AGC_SQUELCH_RISE => {
                AgcSquelchType::AgcSquelchRise
            }
            liquidizers_sys::agc_squelch_mode_LIQUID_AGC_SQUELCH_SIGNALHI => {
                AgcSquelchType::AgcSquelchSignalHi
            }
            liquidizers_sys::agc_squelch_mode_LIQUID_AGC_SQUELCH_FALL => {
                AgcSquelchType::AgcSquelchFall
            }
            liquidizers_sys::agc_squelch_mode_LIQUID_AGC_SQUELCH_SIGNALLO => {
                AgcSquelchType::AgcSquelchSignalLo
            }
            liquidizers_sys::agc_squelch_mode_LIQUID_AGC_SQUELCH_TIMEOUT => {
                AgcSquelchType::AgcSquelchTimeout
            }
            liquidizers_sys::agc_squelch_mode_LIQUID_AGC_SQUELCH_DISABLED => {
                AgcSquelchType::AgcSquelchDisabled
            }
            x => panic!("Invalid return status: {}", x),
        }
    }
    fn to_raw(&self) -> u32 {
        match self {
            AgcSquelchType::AgcSquelchUnknown => {
                liquidizers_sys::agc_squelch_mode_LIQUID_AGC_SQUELCH_UNKNOWN
            }
            AgcSquelchType::AgcSquelchEnabled => {
                liquidizers_sys::agc_squelch_mode_LIQUID_AGC_SQUELCH_ENABLED
            }
            AgcSquelchType::AgcSquelchRise => {
                liquidizers_sys::agc_squelch_mode_LIQUID_AGC_SQUELCH_RISE
            }
            AgcSquelchType::AgcSquelchSignalHi => {
                liquidizers_sys::agc_squelch_mode_LIQUID_AGC_SQUELCH_SIGNALHI
            }
            AgcSquelchType::AgcSquelchFall => {
                liquidizers_sys::agc_squelch_mode_LIQUID_AGC_SQUELCH_FALL
            }
            AgcSquelchType::AgcSquelchSignalLo => {
                liquidizers_sys::agc_squelch_mode_LIQUID_AGC_SQUELCH_SIGNALLO
            }
            AgcSquelchType::AgcSquelchTimeout => {
                liquidizers_sys::agc_squelch_mode_LIQUID_AGC_SQUELCH_TIMEOUT
            }
            AgcSquelchType::AgcSquelchDisabled => {
                liquidizers_sys::agc_squelch_mode_LIQUID_AGC_SQUELCH_DISABLED
            }
        }
    }
}

impl Display for AgcSquelchType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct AgcCrcf {
    pub(crate) raw: liquidizers_sys::agc_crcf,
}

impl Agc<Complex<f32>> for AgcCrcf {
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

    fn execute(&self, input: Complex<f32>) -> Complex<f32> {
        let mut y: liquidizers_sys::__BindgenComplex<f32> = Default::default();
        let x_bg = liquidizers_sys::__BindgenComplex::<f32> {
            re: input.re,
            im: input.im,
        };
        unsafe { liquidizers_sys::agc_crcf_execute(self.raw, x_bg, &mut y) };
        num_complex::Complex::new(y.re, y.im)
    }

    fn execute_block(&self, input: &[Complex<f32>]) -> Vec<Complex<f32>> {
        let mut x_bg: Vec<liquidizers_sys::__BindgenComplex<f32>> = input
            .iter()
            .map(|c| liquidizers_sys::__BindgenComplex::<f32> { re: c.re, im: c.im })
            .collect();

        let mut y_bg: Vec<liquidizers_sys::__BindgenComplex<f32>> =
            vec![Default::default(); input.len()];

        unsafe {
            liquidizers_sys::agc_crcf_execute_block(
                self.raw,
                x_bg.as_mut_ptr(),
                input.len() as u32,
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

    fn unlock(&self) -> Result<()> {
        match Error::from_raw(unsafe { liquidizers_sys::agc_crcf_unlock(self.raw) }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn get_bandwidth(&self) -> f32 {
        unsafe { liquidizers_sys::agc_crcf_get_bandwidth(self.raw) }
    }

    fn get_gain(&self) -> f32 {
        unsafe { liquidizers_sys::agc_crcf_get_gain(self.raw) }
    }

    fn get_rssi(&self) -> f32 {
        unsafe { liquidizers_sys::agc_crcf_get_rssi(self.raw) }
    }

    fn get_scale(&self) -> f32 {
        unsafe { liquidizers_sys::agc_crcf_get_scale(self.raw) }
    }

    fn get_signal_level(&self) -> f32 {
        unsafe { liquidizers_sys::agc_crcf_get_signal_level(self.raw) }
    }

    fn set_bandwidth(&self, bandwidth: f32) -> Result<()> {
        match Error::from_raw(unsafe {
            liquidizers_sys::agc_crcf_set_bandwidth(self.raw, bandwidth)
        }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn set_gain(&self, gain: f32) -> Result<()> {
        match Error::from_raw(unsafe { liquidizers_sys::agc_crcf_set_gain(self.raw, gain) }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn set_rssi(&self, rssi: f32) -> Result<()> {
        match Error::from_raw(unsafe { liquidizers_sys::agc_crcf_set_rssi(self.raw, rssi) }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn set_scale(&self, scale: f32) -> Result<()> {
        match Error::from_raw(unsafe { liquidizers_sys::agc_crcf_set_scale(self.raw, scale) }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn init(&self, input: &[Complex<f32>], size: u32) -> Result<()> {
        let mut x_bg: Vec<liquidizers_sys::__BindgenComplex<f32>> = input
            .iter()
            .map(|c| liquidizers_sys::__BindgenComplex::<f32> { re: c.re, im: c.im })
            .collect();
        match Error::from_raw(unsafe {
            liquidizers_sys::agc_crcf_init(self.raw, x_bg.as_mut_ptr(), size)
        }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn is_locked(&self) -> bool {
        match unsafe { liquidizers_sys::agc_crcf_is_locked(self.raw) } {
            0 => false,
            1 => true,
            x => panic!("Invalid return value from {}", x),
        }
    }

    fn set_signal_level(&self, signal_level: f32) -> Result<()> {
        match Error::from_raw(unsafe {
            liquidizers_sys::agc_crcf_set_signal_level(self.raw, signal_level)
        }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn squelch_enable(&self) -> Result<()> {
        match Error::from_raw(unsafe { liquidizers_sys::agc_crcf_squelch_enable(self.raw) }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn squelch_disable(&self) -> Result<()> {
        match Error::from_raw(unsafe { liquidizers_sys::agc_crcf_squelch_disable(self.raw) }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn squelch_is_enabled(&self) -> bool {
        match unsafe { liquidizers_sys::agc_crcf_squelch_is_enabled(self.raw) } {
            0 => false,
            1 => true,
            x => panic!("Invalid return value from {}", x),
        }
    }

    fn squelch_get_status(&self) -> AgcSquelchType {
        let raw_status = unsafe { liquidizers_sys::agc_crcf_squelch_get_status(self.raw) };
        AgcSquelchType::from_raw(raw_status)
    }

    fn squelch_get_threshold(&self) -> f32 {
        unsafe { liquidizers_sys::agc_crcf_squelch_get_threshold(self.raw) }
    }

    fn squelch_get_timeout(&self) -> u32 {
        unsafe { liquidizers_sys::agc_crcf_squelch_get_timeout(self.raw) }
    }

    fn squelch_set_threshold(&self, threshold: f32) -> Result<()> {
        match Error::from_raw(unsafe {
            liquidizers_sys::agc_crcf_squelch_set_threshold(self.raw, threshold)
        }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn squelch_set_timeout(&self, timeout: u32) -> Result<()> {
        match Error::from_raw(unsafe {
            liquidizers_sys::agc_crcf_squelch_set_timeout(self.raw, timeout)
        }) {
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

impl Agc<f32> for AgcRrrf {
    fn new() -> Self {
        let raw = unsafe { liquidizers_sys::agc_rrrf_create() };
        Self { raw }
    }

    fn execute(&self, input: f32) -> f32 {
        let mut y = 0.0;
        unsafe { liquidizers_sys::agc_rrrf_execute(self.raw, input, &mut y) };
        y
    }

    fn execute_block(&self, input: &[f32]) -> Vec<f32> {
        let mut input_vec: Vec<f32> = input.into();
        let mut y: Vec<f32> = vec![0.0; input.len()];
        unsafe {
            liquidizers_sys::agc_rrrf_execute_block(
                self.raw,
                input_vec.as_mut_ptr(),
                input.len() as u32,
                y.as_mut_ptr(),
            )
        };
        y
    }

    fn get_bandwidth(&self) -> f32 {
        unsafe { liquidizers_sys::agc_rrrf_get_bandwidth(self.raw) }
    }

    fn get_gain(&self) -> f32 {
        unsafe { liquidizers_sys::agc_rrrf_get_gain(self.raw) }
    }

    fn get_rssi(&self) -> f32 {
        unsafe { liquidizers_sys::agc_rrrf_get_rssi(self.raw) }
    }

    fn get_scale(&self) -> f32 {
        unsafe { liquidizers_sys::agc_rrrf_get_scale(self.raw) }
    }

    fn get_signal_level(&self) -> f32 {
        unsafe { liquidizers_sys::agc_rrrf_get_signal_level(self.raw) }
    }

    fn lock(&self) -> Result<()> {
        match Error::from_raw(unsafe { liquidizers_sys::agc_rrrf_lock(self.raw) }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn unlock(&self) -> Result<()> {
        match Error::from_raw(unsafe { liquidizers_sys::agc_rrrf_unlock(self.raw) }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn is_locked(&self) -> bool {
        match unsafe { liquidizers_sys::agc_rrrf_is_locked(self.raw) } {
            0 => false,
            1 => true,
            x => panic!("Invalid return value from {}", x),
        }
    }

    fn init(&self, input: &[f32], size: u32) -> Result<()> {
        let mut input_vec: Vec<f32> = input.into();
        match Error::from_raw(unsafe {
            liquidizers_sys::agc_rrrf_init(self.raw, input_vec.as_mut_ptr(), size)
        }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn print(&self) -> Result<()> {
        match Error::from_raw(unsafe { liquidizers_sys::agc_rrrf_print(self.raw) }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn reset(&self) -> Result<()> {
        match Error::from_raw(unsafe { liquidizers_sys::agc_rrrf_reset(self.raw) }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn set_bandwidth(&self, bandwidth: f32) -> Result<()> {
        match Error::from_raw(unsafe {
            liquidizers_sys::agc_rrrf_set_bandwidth(self.raw, bandwidth)
        }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn set_gain(&self, gain: f32) -> Result<()> {
        match Error::from_raw(unsafe { liquidizers_sys::agc_rrrf_set_gain(self.raw, gain) }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn set_rssi(&self, rssi: f32) -> Result<()> {
        match Error::from_raw(unsafe { liquidizers_sys::agc_rrrf_set_rssi(self.raw, rssi) }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn set_scale(&self, scale: f32) -> Result<()> {
        match Error::from_raw(unsafe { liquidizers_sys::agc_rrrf_set_scale(self.raw, scale) }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn set_signal_level(&self, signal_level: f32) -> Result<()> {
        match Error::from_raw(unsafe {
            liquidizers_sys::agc_rrrf_set_signal_level(self.raw, signal_level)
        }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn squelch_disable(&self) -> Result<()> {
        match Error::from_raw(unsafe { liquidizers_sys::agc_rrrf_squelch_disable(self.raw) }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn squelch_enable(&self) -> Result<()> {
        match Error::from_raw(unsafe { liquidizers_sys::agc_rrrf_squelch_enable(self.raw) }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn squelch_get_status(&self) -> AgcSquelchType {
        let raw_status = unsafe { liquidizers_sys::agc_rrrf_squelch_get_status(self.raw) };
        AgcSquelchType::from_raw(raw_status)
    }

    fn squelch_get_threshold(&self) -> f32 {
        unsafe { liquidizers_sys::agc_rrrf_squelch_get_threshold(self.raw) }
    }

    fn squelch_get_timeout(&self) -> u32 {
        unsafe { liquidizers_sys::agc_rrrf_squelch_get_timeout(self.raw) }
    }

    fn squelch_set_threshold(&self, threshold: f32) -> Result<()> {
        match Error::from_raw(unsafe {
            liquidizers_sys::agc_rrrf_squelch_set_threshold(self.raw, threshold)
        }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn squelch_set_timeout(&self, timeout: u32) -> Result<()> {
        match Error::from_raw(unsafe {
            liquidizers_sys::agc_rrrf_squelch_set_timeout(self.raw, timeout)
        }) {
            Error::LiquidOk => Ok(()),
            e => Err(e),
        }
    }

    fn squelch_is_enabled(&self) -> bool {
        match unsafe { liquidizers_sys::agc_rrrf_squelch_is_enabled(self.raw) } {
            0 => false,
            1 => true,
            x => panic!("Invalid return value from {}", x),
        }
    }
}

impl Clone for AgcRrrf {
    fn clone(&self) -> Self {
        Self {
            raw: unsafe { liquidizers_sys::agc_rrrf_copy(self.raw) },
        }
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
