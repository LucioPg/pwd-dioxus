// Copyright (c) 2026 Lucio Di Capua <ldcproductions@proton.me>
// Licensed under the Prosperity Public License 3.0.0
// Commercial use requires a license. See LICENSE.md for details.

use dioxus::prelude::*;
use dioxus::document::eval;
use std::sync::atomic::{AtomicUsize, Ordering};

static COMBO_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Restore overflow on any ancestor marked by pwd-combo-dropdown-open
static COMBO_RESTORE_OVERFLOW: &str =
    r#"document.querySelectorAll('[data-pwd-orig-overflow]').forEach(function(el){el.style.overflow=el.dataset.pwdOrigOverflow;delete el.dataset.pwdOrigOverflow})"#;

/// Tear down the global scroll listener (called when dropdown closes)
static COMBO_TEARDOWN_SCROLL: &str =
    r#"if(window._pwdComboScrollClose){window.removeEventListener('scroll',window._pwdComboScrollClose,true);delete window._pwdComboScrollClose}"#;

#[derive(Clone, Copy, Default, PartialEq)]
pub enum ComboboxSize {
    Small,
    Medium,
    #[default]
    Large,
    Full,
}

impl ComboboxSize {
    pub fn size_class(&self) -> &'static str {
        match self {
            ComboboxSize::Small => "w-32",
            ComboboxSize::Medium => "w-40",
            ComboboxSize::Large => "w-64",
            ComboboxSize::Full => "w-full",
        }
    }
}
#[component]
pub fn Combobox<T: Clone + PartialEq + 'static>(
    options: Vec<(&'static str, Option<T>)>,
    placeholder: String,
    on_change: EventHandler<Option<T>>,
    #[props(default)] disabled: ReadOnlySignal<bool>,
    #[props(default)] selected_value: Option<T>,
    #[props(default)] size: ComboboxSize,
) -> Element {
    let mut is_open = use_signal(|| false);

    // Internal display signal: updated immediately on selection for instant feedback,
    // and synced from selected_value when the parent sets it externally.
    let initial_label = selected_value
        .as_ref()
        .and_then(|val| {
            options
                .iter()
                .find(|(_, opt_val)| opt_val.as_ref() == Some(val))
        })
        .map(|(label, _)| label.to_string());
    let mut display_label = use_signal(|| initial_label);

    // Sync display when parent changes selected_value or options after mount
    let sync_options = options.clone();
    use_effect(move || {
        let label = selected_value
            .as_ref()
            .and_then(|val| {
                sync_options
                    .iter()
                    .find(|(_, opt_val)| opt_val.as_ref() == Some(val))
            })
            .map(|(label, _)| label.to_string());
        display_label.set(label);
    });

    let is_disabled = use_memo(move || disabled());
    let combo_id = use_signal(|| {
        format!(
            "pwd-combo-{}",
            COMBO_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
        )
    });

    // Close dropdown on page scroll (prevents detachment)
    use_effect(move || {
        if is_open() {
            let id = combo_id().clone();
            eval(&format!(
                r#"window._pwdComboScrollClose=function(e){{var d=document.getElementById('{id}-dropdown');if(d&&d.contains(e.target))return;var o=document.getElementById('{id}');if(o)o.click()}};window.addEventListener('scroll',window._pwdComboScrollClose,true)"#,
            ));
        } else {
            eval(COMBO_TEARDOWN_SCROLL);
        }
    });

    rsx! {
        div {
            // Bottone trigger
            div {
                id: "{combo_id}",
                role: "button",
                class: if is_disabled() {
                    "btn m-1 {size.size_class()} justify-between btn-disabled"
                } else {
                    "btn m-1 {size.size_class()} justify-between"
                },
                onclick: move |_| {
                    if is_disabled() {
                        return;
                    }
                    if is_open() {
                        is_open.set(false);
                        eval(COMBO_RESTORE_OVERFLOW);
                    } else {
                        is_open.set(true);
                    }
                },
                "{display_label.read().clone().unwrap_or(placeholder.clone())}"
                // Icona freccia
                span { class: "text-xs", {if is_open() { "▲" } else { "▼" }} }
            }

            // Menu delle opzioni - position: fixed per superare overflow dei parent
            if is_open() && !is_disabled() {
                div {
                    class: "fixed inset-0 z-[9998]",
                    onclick: move |_| {
                        is_open.set(false);
                        eval(COMBO_RESTORE_OVERFLOW);
                    },
                }
                ul {
                    id: "{combo_id}-dropdown",
                    class: "menu p-2 shadow-lg bg-base-100 rounded-box w-64",
                    style: "position: fixed; z-index: 9999; top: -9999px; left: 0; max-height: 50vh; overflow-y: auto;",
                    onmounted: move |_| {
                        let id = combo_id().clone();
                        eval(&format!(
                            r#"const t=document.getElementById('{id}');const d=document.getElementById('{id}-dropdown');if(t&&d){{const tr=t.getBoundingClientRect();let cb=d.parentElement,found=null;while(cb&&cb!==document.body){{const m=document.createElement('div');m.style.cssText='position:fixed;top:0;visibility:hidden';cb.appendChild(m);if(m.getBoundingClientRect().top!==0){{found=cb;cb.removeChild(m);break}}cb.removeChild(m);cb=cb.parentElement}}if(found){{const cr=found.getBoundingClientRect();d.style.top=(tr.bottom-cr.top)+'px';d.style.left=(tr.left-cr.left)+'px';found.dataset.pwdOrigOverflow=found.style.overflow||getComputedStyle(found).overflow;found.style.overflow='visible'}}else{{d.style.top=tr.bottom+'px';d.style.left=tr.left+'px'}}}}"#,
                        ));
                    },
                    for (label, value) in options {
                        li {
                            key: "{label}",
                            a {
                                onclick: move |_| {
                                    display_label.set(Some(label.to_string()));
                                    on_change.call(value.clone());
                                    is_open.set(false);
                                    eval(COMBO_RESTORE_OVERFLOW);
                                },
                                "{label}"
                            }
                        }
                    }
                }
            }
        }
    }
}
