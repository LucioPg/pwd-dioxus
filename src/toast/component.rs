//! Toast container component

use crate::toast::state::{ToastHubState, ToastType};
use dioxus::prelude::*;

/// Container component for displaying toasts.
///
/// This component must be placed at the root of your app.
/// It automatically manages toast lifecycles and animations.
#[component]
pub fn ToastContainer() -> Element {
    let mut state = use_context::<Signal<ToastHubState>>();

    // Flush pending toasts on each render
    use_effect(move || {
        state.write().flush_pending();
    });

    // Timer to update toast timeouts
    use_effect(move || {
        let mut state = state.clone();
        spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                state.write().update_timeouts();
            }
        });
    });

    let toast_type_class = |ts: &ToastType| match ts {
        ToastType::Success => "pwd-toast-success",
        ToastType::Error => "pwd-toast-error",
        ToastType::Warning => "pwd-toast-warning",
        ToastType::Info => "pwd-toast-info",
    };

    rsx! {
        div { class: "pwd-toast-container",
            for toast in state.read().messages().iter() {
                {
                    let transition_class = if toast.is_leaving { "pwd-toast-out" } else { "pwd-toast-in" };
                    rsx! {
                        div {
                            key: "{toast.id}",
                            class: "{toast_type_class(&toast.toast_type)} {transition_class}",
                            "{toast.message}"
                        }
                    }
                }
            }
        }
    }
}
