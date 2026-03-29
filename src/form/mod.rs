// Copyright (c) 2026 Lucio Di Capua <ldcproductions@proton.me>
// Licensed under the Prosperity Public License 3.0.0
// Commercial use requires a license. See LICENSE.md for details.

mod types;
mod field;

pub use types::{FormSecret, FormValue, InputType, NonNegativeInt, PositiveInt, SPECIAL_CHARS};
pub use field::FormField;
