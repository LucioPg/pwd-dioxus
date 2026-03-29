// Copyright (c) 2026 Lucio Di Capua <ldcproductions@proton.me>
// Licensed under the Prosperity Public License 3.0.0
// Commercial use requires a license. See LICENSE.md for details.

//! Toast state management

use dioxus::prelude::*;
use std::time::Instant;

// ============================================================================
// TYPES
// ============================================================================

#[derive(Clone, PartialEq, Debug)]
pub enum ToastType {
    Success,
    Error,
    #[allow(dead_code)]
    Warning,
    Info,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ToastMessage {
    pub id: usize,
    pub message: String,
    pub duration: usize,
    pub toast_type: ToastType,
    pub is_leaving: bool,
    pub created_at: Instant,
}

impl Default for ToastMessage {
    fn default() -> Self {
        Self {
            id: Default::default(),
            message: Default::default(),
            duration: 3,
            toast_type: ToastType::Info,
            is_leaving: false,
            created_at: Instant::now(),
        }
    }
}

// ============================================================================
// STATE
// ============================================================================

#[derive(Clone, Default, Debug)]
pub struct ToastHubState {
    messages: Vec<ToastMessage>,
    counter: usize,
    pending: Vec<ToastMessage>,
}

impl ToastHubState {
    fn push(&mut self, message: String, duration: usize, toast_type: ToastType) -> usize {
        let id = self.counter;
        let toast = ToastMessage {
            id,
            message,
            duration,
            toast_type,
            is_leaving: false,
            created_at: Instant::now(),
        };
        self.messages.push(toast);
        self.counter += 1;
        id
    }

    fn remove(&mut self, id: usize) {
        self.messages.retain(|m| m.id != id);
    }

    fn schedule(&mut self, message: String, duration: usize, toast_type: ToastType) {
        let id = self.counter;
        let toast = ToastMessage {
            id,
            message,
            duration,
            toast_type,
            is_leaving: false,
            created_at: Instant::now(),
        };
        self.pending.push(toast);
        self.counter += 1;
    }

    pub fn flush_pending(&mut self) {
        let pending = std::mem::take(&mut self.pending);
        for toast in pending {
            self.messages.push(toast);
        }
    }

    pub fn update_timeouts(&mut self) -> bool {
        let now = Instant::now();
        let mut changed = false;
        let mut to_remove = Vec::new();

        for toast in &mut self.messages {
            if !toast.is_leaving {
                let elapsed = now.duration_since(toast.created_at).as_secs() as usize;
                if elapsed >= toast.duration {
                    toast.is_leaving = true;
                    changed = true;
                }
            } else {
                let elapsed = now.duration_since(toast.created_at).as_secs() as usize;
                let leave_duration = toast.duration;
                if elapsed >= toast.duration + leave_duration {
                    to_remove.push(toast.id);
                    changed = true;
                }
            }
        }

        for id in to_remove {
            self.remove(id);
        }

        changed
    }

    /// Returns a reference to the active messages
    pub fn messages(&self) -> &[ToastMessage] {
        &self.messages
    }
}

// ============================================================================
// HOOK - Main API for components
// ============================================================================

/// Hook to use ToastHub in a component.
///
/// Returns a `Signal<ToastHubState>` that can be used with helper functions.
pub fn use_toast() -> Signal<ToastHubState> {
    use_context::<Signal<ToastHubState>>()
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Show a success toast immediately
pub fn show_toast_success(message: String, mut state: Signal<ToastHubState>) {
    state.write().push(message, 3, ToastType::Success);
}

/// Show an error toast immediately
pub fn show_toast_error(message: String, mut state: Signal<ToastHubState>) {
    state.write().push(message, 4, ToastType::Error);
}

/// Schedule a toast to show after navigation
pub fn schedule_toast_success(message: String, mut state: Signal<ToastHubState>) {
    state.write().schedule(message, 3, ToastType::Success);
}
