//! # ToastHub - Centralized Toast System
//!
//! This module provides a centralized system for managing toasts
//! that survive navigation between pages.

mod component;
mod state;

pub use component::ToastContainer;
pub use state::{show_toast_error, show_toast_success, schedule_toast_success, ToastHubState, ToastMessage, ToastType, use_toast};
