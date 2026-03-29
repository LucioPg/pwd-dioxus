// Copyright (c) 2026 Lucio Di Capua <ldcproductions@proton.me>
// Licensed under the Prosperity Public License 3.0.0
// Commercial use requires a license. See LICENSE.md for details.

mod analyzer;
mod handler;

pub use analyzer::StrengthAnalyzer;
pub use handler::{PasswordHandler, EvaluationResult, GenerationMethod};
