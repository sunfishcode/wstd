use wasi::http::types::Scheme as WasiScheme;

use alloc::borrow::ToOwned;
use core::str::FromStr;
pub use http::uri::{InvalidUri, Scheme};

pub(crate) fn to_wasi_scheme(value: &Scheme) -> WasiScheme {
    match value.as_str() {
        "http" => WasiScheme::Http,
        "https" => WasiScheme::Https,
        other => WasiScheme::Other(other.to_owned()),
    }
}

pub(crate) fn from_wasi_scheme(value: WasiScheme) -> Result<Scheme, InvalidUri> {
    Ok(match value {
        WasiScheme::Http => Scheme::HTTP,
        WasiScheme::Https => Scheme::HTTPS,
        WasiScheme::Other(other) => Scheme::from_str(&other)?,
    })
}
