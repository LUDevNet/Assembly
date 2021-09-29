//! # The XML formats
//!
//! The game client uses XML for a lot of data storage. This module contains helpers for
//! typed access to these files.
#![warn(missing_docs)]

pub use quick_xml as quick;

pub mod all_settings;
pub mod behavior;
pub mod block_library;
pub mod common;
pub mod credits;
pub mod database;
pub mod env_data;
pub mod hud;
pub mod lego_primitive;
pub mod localization;
pub mod modular_build;
pub mod module_info;
pub mod nduiml;
pub mod obj;
pub mod triggers;
pub mod ui_settings;
pub mod universe_config;
