use anyhow::{Error, Result};

pub trait Convert<T> {
    fn convert(x: T) -> Self;
}

impl Convert<usize> for f64 {
    fn convert(x: usize) -> Self {
        x as f64
    }
}

impl Convert<u8> for f64 {
    fn convert(x: u8) -> Self {
        f64::from(x)
    }
}
impl Convert<u16> for f64 {
    fn convert(x: u16) -> Self {
        f64::from(x)
    }
}
impl Convert<i32> for f64 {
    fn convert(x: i32) -> Self {
        f64::from(x)
    }
}
impl Convert<i64> for f64 {
    fn convert(x: i64) -> Self {
        x as f64
    }
}

impl Convert<f32> for f64 {
    fn convert(x: f32) -> Self {
        x as f64
    }
}

impl Convert<f64> for f64 {
    fn convert(x: f64) -> Self {
        x
    }
}

impl Convert<&f64> for f64 {
    fn convert(x: &f64) -> Self {
        x.clone()
    }
}

impl Convert<String> for f64 {
    fn convert(x: String) -> Self {
        let parsed = x.parse::<f64>();
        parsed.unwrap_or_else(|err| panic!("{}", err))
    }
}

pub fn convert_slice_to_f64<T: Copy>(raw: &[T], offset: f64, scale: f64) -> Result<Vec<f64>, Error>
where
    f64: Convert<T>,
{
    Ok(raw
        .iter()
        .map(|&x| (f64::convert(x) + offset) * scale)
        .collect::<Vec<f64>>())
}
