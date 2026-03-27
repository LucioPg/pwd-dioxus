#![allow(dead_code)]
#![allow(unused_variables)]

use dioxus::prelude::*;

/// Dimensioni disponibili per il Toggle
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ToggleSize {
    #[default]
    Small,  // toggle-sm
    Medium, // toggle-md (default daisyUI)
    Large,  // toggle-lg
}

impl ToggleSize {
    /// Restituisce la classe CSS per la dimensione
    pub fn as_class(&self) -> &'static str {
        match self {
            ToggleSize::Small => "toggle-sm",
            ToggleSize::Medium => "",
            ToggleSize::Large => "toggle-lg",
        }
    }
}

/// Varianti di colore per il Toggle
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ToggleColor {
    #[default]
    Primary,
    Secondary,
    Accent,
    Success,
    Warning,
    Info,
    Error,
    Neutral,
}

impl ToggleColor {
    /// Restituisce la classe CSS per il colore
    pub fn as_class(&self) -> &'static str {
        match self {
            ToggleColor::Primary => "toggle-primary",
            ToggleColor::Secondary => "toggle-secondary",
            ToggleColor::Accent => "toggle-accent",
            ToggleColor::Success => "toggle-success",
            ToggleColor::Warning => "toggle-warning",
            ToggleColor::Info => "toggle-info",
            ToggleColor::Error => "toggle-error",
            ToggleColor::Neutral => "toggle-neutral",
        }
    }
}

/// Componente Toggle - Interruttore on/off
///
/// Componente riutilizzabile per attivare/disattivare opzioni.
/// Basato sul toggle di daisyUI.
///
/// # Esempio
///
/// ```rust,no_run
/// use dioxus::prelude::*;
/// use pwd_dioxus::toggle::{Toggle, ToggleSize, ToggleColor};
///
/// let mut is_enabled = use_signal(|| false);
///
/// // Toggle base
/// Toggle {
///     checked: is_enabled(),
///     onchange: move |_| is_enabled.toggle(),
/// }
///
/// // Toggle con dimensione e colore personalizzati
/// Toggle {
///     checked: is_enabled(),
///     onchange: move |_| is_enabled.toggle(),
///     size: ToggleSize::Large,
///     color: ToggleColor::Success,
/// }
/// ```
#[component]
pub fn Toggle(
    /// Stato corrente del toggle (true = on, false = off)
    checked: bool,
    /// Callback chiamato quando il toggle viene cliccato
    onchange: EventHandler<()>,
    /// Dimensione del toggle
    #[props(default)]
    size: ToggleSize,
    /// Colore del toggle quando attivo
    #[props(default)]
    color: ToggleColor,
    /// Disabilita il toggle
    #[props(default)]
    disabled: bool,
    /// Classe CSS aggiuntiva
    #[props(default)]
    class: Option<String>,
) -> Element {
    let size_class = size.as_class();
    let color_class = color.as_class();

    let mut classes = String::from("toggle");
    if !size_class.is_empty() {
        classes.push(' ');
        classes.push_str(size_class);
    }
    if !color_class.is_empty() {
        classes.push(' ');
        classes.push_str(color_class);
    }
    if let Some(custom_class) = &class {
        classes.push(' ');
        classes.push_str(custom_class);
    }

    rsx! {
        input {
            r#type: "checkbox",
            class: "{classes}",
            checked: checked,
            disabled: disabled,
            onchange: move |_| onchange.call(()),
        }
    }
}
