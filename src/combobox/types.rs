// Copyright (c) 2026 Lucio Di Capua <ldcproductions@proton.me>
// Licensed under the Prosperity Public License 3.0.0
// Commercial use requires a license. See LICENSE.md for details.

use pwd_types::PasswordPreset;

#[derive(Clone, PartialEq, Copy)]
pub enum AnyPreset {
    Standard(PasswordPreset),
    Custom,
}