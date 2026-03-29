# pwd-dioxus

Reusable Dioxus components for password handling UI.

## Requirements

This library uses CSS classes compatible with **DaisyUI 5** and Tailwind CSS. Make sure your project includes:

- [Tailwind CSS](https://tailwindcss.com/)
- [DaisyUI 5](https://daisyui.com/)

## Features

### Atomic Components

| Feature | Description |
|---------|-------------|
| `icons` | All icon components |
| `icons-visibility` | Show/hide password icons |
| `icons-action` | Action icons (copy, delete, etc.) |
| `icons-alert` | Alert/warning icons |
| `spinner` | Loading spinner component |
| `modal` | Modal dialog component |

### Composite Components

| Feature | Description |
|---------|-------------|
| `form` | Form field with label and validation |
| `secret-display` | Secret field with visibility toggle |
| `analyzer` | Password strength analyzer UI |
| `handler` | Complete password input with strength analysis |

### Convenience

| Feature | Description |
|---------|-------------|
| `full` | All features enabled |

## Usage

### Minimal (icons only)

```toml
[dependencies]
pwd-dioxus = { git = "https://github.com/LucioPg/pwd-dioxus", features = ["icons-visibility"] }
```

### Password Handler (recommended)

```toml
[dependencies]
pwd-dioxus = { git = "https://github.com/LucioPg/pwd-dioxus", features = ["handler"] }
```

## Example

```rust
use pwd_dioxus::{PasswordHandler, PasswordChangeResult};
use dioxus::prelude::*;

fn LoginForm(cx: Scope) -> Element {
    let mut password_result = use_signal(|| PasswordChangeResult::default());

    rsx! {
        PasswordHandler {
            on_password_change: move |result| {
                password_result.set(result);
            },
            password_required: true,
            initial_password: None,
            initial_score: None,
        }
    }
}
```

## Components

- `PasswordHandler` - Complete password input with strength analysis
- `StrengthAnalyzer` - Visual password strength indicator
- `FormField` - Generic form field component
- `SecretDisplay` - Password field with visibility toggle
- `Spinner` - Loading indicator
- `Modal` - Dialog component

## License and Commercial Use

This project is licensed under the **Prosperity Public License 3.0.0**.

### What does this mean for you?

- **Personal and Non-Profit Use:** You are free to use, study, and modify this software at no cost for personal,
  educational, or research purposes.
- **Commercial Use:** If you are a company or a professional using this software for profit-making activities, you are
  granted a **30-day trial period**.

### How to Obtain a Commercial License

To continue using the software for commercial purposes after the 30-day trial, you must purchase a dedicated commercial
license.

To request a quote or activate your license, please contact:
**ldcproductions@proton.me**

*Please use the subject line: "Commercial License Request - pwd-dioxus"*

---
*Note: This software is built using the Dioxus framework (MIT/Apache 2.0). All third-party open-source components remain
subject to their respective licenses.*
