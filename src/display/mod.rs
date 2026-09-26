//! Display subsystem.
//!
//! Implements a text-mode framebuffer renderer with glyph rasterization,
//! cursor tracking, vertical scrolling, and `core::fmt::Write` integration.

pub mod cell;
pub mod font;
pub mod framebuffer;
pub mod greet;
pub mod macros;
