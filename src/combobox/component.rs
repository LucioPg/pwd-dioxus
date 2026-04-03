// Copyright (c) 2026 Lucio Di Capua <ldcproductions@proton.me>
// Licensed under the Prosperity Public License 3.0.0
// Commercial use requires a license. See LICENSE.md for details.

use dioxus::prelude::*;
use dioxus::document::eval;
use std::sync::atomic::{AtomicUsize, Ordering};

static COMBO_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

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
    #[props(default)] selected_value: ReadOnlySignal<Option<T>>,
    #[props(default)] size: ComboboxSize,
) -> Element {
    let initial_label = selected_value()
        .as_ref()
        .and_then(|val| {
            options
                .iter()
                .find(|(_, opt_val)| opt_val.as_ref() == Some(val))
        })
        .map(|(label, _)| label.to_string())
        .unwrap_or_else(|| placeholder.clone());
    let mut is_open = use_signal(|| false);
    let mut selected_item = use_signal(|| initial_label);
    // Keep selected_item in sync when selected_value or options change after mount
    use_effect(use_reactive(
        &options,
        move |options| {
            let label = selected_value()
                .as_ref()
                .and_then(|val| {
                    options
                        .iter()
                        .find(|(_, opt_val)| opt_val.as_ref() == Some(val))
                })
                .map(|(label, _)| label.to_string());
            if let Some(label) = label {
                selected_item.set(label);
            }
        },
    ));
    let is_disabled = use_memo(move || disabled());
    let combo_id = use_signal(|| {
        format!(
            "pwd-combo-{}",
            COMBO_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
        )
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
                onclick: move |_| if !is_disabled() { is_open.toggle() },
                "{selected_item}"
                // Icona freccia
                span { class: "text-xs", {if is_open() { "▲" } else { "▼" }} }
            }

            // Menu delle opzioni - position: fixed per superare overflow dei parent
            if is_open() && !is_disabled() {
                div {
                    class: "fixed inset-0 z-[9998]",
                    onclick: move |_| is_open.set(false),
                }
                ul {
                    id: "{combo_id}-dropdown",
                    class: "menu p-2 shadow-lg bg-base-100 rounded-box w-64",
                    style: "position: fixed; z-index: 9999; top: -9999px; left: 0;",
                    onmounted: move |_| {
                        let id = combo_id().clone();
                        eval(&format!(
                            r#"const t=document.getElementById('{id}');const d=document.getElementById('{id}-dropdown');if(t&&d){{const r=t.getBoundingClientRect();d.style.top=r.bottom+'px';d.style.left=r.left+'px';}}"#,
                        ));
                    },
                    for (label, value) in options {
                        li {
                            key: "{label}",
                            a {
                                onclick: move |_| {
                                    selected_item.set(label.to_string());
                                    on_change.call(value.clone());
                                    is_open.set(false);
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
