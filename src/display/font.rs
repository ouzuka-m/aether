//! Font configuration constants.
//!
//! Selects the font weight and raster height used by the framebuffer
//! text renderer from the `noto-sans-mono-bitmap` crate.

use noto_sans_mono_bitmap::{FontWeight, RasterHeight};

pub const WEIGHT: FontWeight = FontWeight::Regular;
pub const SIZE: RasterHeight = RasterHeight::Size20;
