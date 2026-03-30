// Copyright (c) 2026 Lucio Di Capua <ldcproductions@proton.me>
// Licensed under the Prosperity Public License 3.0.0
// Commercial use requires a license. See LICENSE.md for details.

//! Reusable Dioxus components for password handling.

// Icons: incluso se qualsiasi feature icons* è attiva
#[cfg(any(feature = "icons", feature = "icons-visibility",
          feature = "icons-action", feature = "icons-alert"))]
pub mod icons;

// ============================================================================
// CSS ASSETS FOR CONSUMER BUILD SYSTEMS
// ============================================================================

/// Custom CSS for pwd-dioxus components.
///
/// This CSS defines component-specific styles (spinner animations, toast
/// notifications, strength analyzer, etc.) that are NOT Tailwind classes.
///
/// ## Usage in Consumer Project
///
/// ### Option 1: Include in your Tailwind build (recommended)
///
/// In your build.rs, extract this CSS to a file before running Tailwind:
///
/// ```rust,ignore
/// use pwd_dioxus::COMPONENT_CSS;
/// std::fs::write("assets/pwd-dioxus-components.css", COMPONENT_CSS)?;
/// ```
///
/// Then in your CSS:
/// ```css
/// @import "./pwd-dioxus-components.css";
/// ```
///
/// ### Option 2: Include directly in main CSS
///
/// ```rust,ignore
/// // In your main.rs or component
/// const PWD_DIOXUS_CSS: &str = pwd_dioxus::COMPONENT_CSS;
/// ```
pub const COMPONENT_CSS: &str = include_str!("../assets/components.css");

#[cfg(feature = "spinner")]
pub mod spinner;

#[cfg(feature = "modal")]
pub mod modal;

#[cfg(feature = "form")]
pub mod form;

#[cfg(feature = "secret-display")]
pub mod secret;

// password module: incluso per analyzer o handler
#[cfg(any(feature = "analyzer", feature = "handler"))]
pub mod password;

#[cfg(feature = "toast")]
pub mod toast;

#[cfg(feature = "toggle")]
pub mod toggle;

#[cfg(feature = "combobox")]
pub mod combobox;
// === RE-EXPORTS per convenienza ===

#[cfg(feature = "form")]
pub use form::{FormSecret, FormValue, InputType, SPECIAL_CHARS};

#[cfg(feature = "analyzer")]
pub use password::StrengthAnalyzer;

#[cfg(feature = "handler")]
pub use password::{PasswordHandler, EvaluationResult};

#[cfg(feature = "toggle")]
pub use toggle::{Toggle, ToggleColor, ToggleSize};

#[cfg(feature = "toast")]
pub use toast::{
    show_toast_error, show_toast_success, schedule_toast_success,
    ToastContainer, ToastHubState, ToastMessage, ToastType, use_toast,
};

// Re-export tipi pwd-types usati nelle API pubbliche
// I consumer possono usarli senza aggiungere pwd-types alle dipendenze
#[cfg(any(feature = "analyzer", feature = "handler"))]
pub use pwd_types::{PasswordScore, PasswordStrength};

#[cfg(feature = "handler")]
pub use pwd_types::PasswordChangeResult;

#[cfg(feature = "combobox")]
pub use combobox::{Combobox,AnyPreset};
