use dioxus::prelude::*;

#[component]
pub fn Combobox<T: Clone + PartialEq + 'static>(
    options: Vec<(&'static str, Option<T>)>,
    placeholder: String,
    on_change: EventHandler<Option<T>>,
    #[props(default)] disabled: Signal<bool>,
    #[props(default)] selected_value: Option<T>,
) -> Element {
    let initial_label = selected_value
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
    let dropdown_class = use_memo(move || if is_open() { "dropdown-open" } else { "" });
    let is_disabled = use_memo(move || disabled());

    rsx! {
        div { class: "dropdown {dropdown_class}",

            // Bottone che apre/chiude lo switch
            div {
                role: "button",
                class: if is_disabled() {
                    "btn m-1 w-64 justify-between btn-disabled"
                } else {
                    "btn m-1 w-64 justify-between"
                },
                onclick: move |_| if !is_disabled() { is_open.toggle() },
                "{selected_item}"
                // Icona freccia (opzionale)
                span { class: "text-xs", {if is_open() { "▲" } else { "▼" }} }
            }

            // Menu delle opzioni
            if is_open() && !is_disabled() {
                div {
                    class: "fixed inset-0 z-0",
                    onclick: move |_| is_open.set(false),
                }
                ul { class: "dropdown-content z-[9999] menu p-2 shadow bg-base-100 rounded-box w-64",
                    for (label , value) in options {
                        li {
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
