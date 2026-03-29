// Copyright (c) 2026 Lucio Di Capua <ldcproductions@proton.me>
// Licensed under the Prosperity Public License 3.0.0
// Commercial use requires a license. See LICENSE.md for details.

//! # ToastHub - Centralized Toast System
//!
//! This module provides a centralized system for managing toasts
//! that survive navigation between pages.

mod component;
mod state;

pub use component::ToastContainer;
pub use state::{show_toast_error, show_toast_success, schedule_toast_success, ToastHubState, ToastMessage, ToastType, use_toast};
