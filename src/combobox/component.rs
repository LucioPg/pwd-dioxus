use dioxus::prelude::*;

#[component]
pub fn Combobox<T: Clone + PartialEq + 'static>(
    options: Vec<(&'static str, Option<T>)>,
    placeholder: String,
    on_change: EventHandler<Option<T>>,
) -> Element {
    let mut is_open = use_signal(|| false);
    let mut selected_item = use_signal(|| placeholder);
    let dropdown_class = use_memo(move || if is_open() { "dropdown-open" } else { "" });

    rsx! {
        div { class: "dropdown {dropdown_class}",

            // Bottone che apre/chiude lo switch
            div {
                role: "button",
                class: "btn m-1 w-64 justify-between",
                onclick: move |_| is_open.toggle(),
                "{selected_item}"
                // Icona freccia (opzionale)
                span { class: "text-xs", {if is_open() { "▲" } else { "▼" }} }
            }

            // Menu delle opzioni
            if is_open() {
                div {
                    class: "fixed inset-0 z-0",
                    onclick: move |_| is_open.set(false),
                }
                ul { class: "dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-64",
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
