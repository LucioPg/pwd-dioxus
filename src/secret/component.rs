use crate::form::FormSecret;
use crate::icons::{ClipboardIcon, EyeIcon, EyeOffIcon};
use dioxus::prelude::*;
use secrecy::ExposeSecret;

/// Copia il testo negli appunti del sistema (nativo desktop)
fn copy_to_clipboard(text: String) {
    match arboard::Clipboard::new() {
        Ok(mut clipboard) => {
            if let Err(e) = clipboard.set_text(&text) {
                eprintln!("Errore clipboard: {}", e);
            }
        }
        Err(e) => eprintln!("Impossibile accedere agli appunti: {}", e),
    }
}

/// Componente SecretDisplay - visualizza dati sensibili con toggle visibility
///
/// Usato per: password, location, e altri campi sensibili
#[component]
pub fn SecretDisplay(
    /// Il valore segreto da visualizzare (FormSecret wrappa SecretString)
    secret: FormSecret,
    /// Classe CSS aggiuntiva per il container (opzionale)
    #[props(default)]
    class: Option<String>,
    /// Larghezza massima del contenitore (default: 200px)
    #[props(default = "200px".to_string())]
    max_width: String,
) -> Element {
    let mut visible = use_signal(|| false);

    let value_len = secret.expose_secret().len();
    let display_value = if value_len == 0 {
        String::new()
    } else if visible() {
        secret.expose_secret().to_string()
    } else {
        "•".repeat(value_len)
    };

    // Clona il valore segreto per usarlo nel closure
    let secret_value = secret.expose_secret().to_string();

    rsx! {
        div { class: "secret-display-wrapper {class.clone().unwrap_or_default()}",
            input {
                class: "pwd-secret-display font-mono",
                r#type: if visible() { "text" } else { "password" },
                value: "{display_value}",
                readonly: true,
                title: if visible() {
                    Some(secret.expose_secret().to_string())
                } else {
                    None
                },
                style: "max-width: {max_width}",
            }

            div { class: "secret-display-actions flex gap-1",
                button {
                    class: "pwd-display-action-btn",
                    r#type: "button",
                    tabindex: "-1",
                    onclick: move |_| visible.set(!visible()),
                    aria_label: if visible() { "Nascondi" } else { "Mostra" },
                    if visible() {
                        EyeOffIcon { class: Some("text-current".to_string()) }
                    } else {
                        EyeIcon { class: Some("text-current".to_string()) }
                    }
                }

                button {
                    class: "pwd-display-action-btn",
                    r#type: "button",
                    tabindex: "-1",
                    aria_label: "Copia negli appunti",
                    disabled: value_len == 0,
                    onclick: move |_| {
                        copy_to_clipboard(secret_value.clone());
                    },
                    ClipboardIcon { class: Some("text-current".to_string()) }
                }
            }
        }
    }
}
