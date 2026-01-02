use std::time::Instant;

use ratatui::style::Color;

use crate::ui::theme;

const MAX_TOASTS: usize = 5;
const TOAST_DURATION_SECS: u64 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastType {
    Error,
    Success,
    Info,
}

impl ToastType {
    pub fn icon(&self) -> &'static str {
        match self {
            ToastType::Error => "[!]",
            ToastType::Success => "[+]",
            ToastType::Info => "[i]",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ToastType::Error => "ERROR",
            ToastType::Success => "SUCCESS",
            ToastType::Info => "INFO",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            ToastType::Error => theme::RED,
            ToastType::Success => theme::GREEN,
            ToastType::Info => theme::BLUE,
        }
    }
}

pub struct Toast {
    pub toast_type: ToastType,
    pub message: String,
    pub created_at: Instant,
}

impl Toast {
    pub fn new(toast_type: ToastType, message: impl Into<String>) -> Self {
        Self {
            toast_type,
            message: message.into(),
            created_at: Instant::now(),
        }
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed().as_secs() >= TOAST_DURATION_SECS
    }
}

#[derive(Default)]
pub struct ToastQueue {
    toasts: Vec<Toast>,
}

impl ToastQueue {
    pub fn push(&mut self, toast: Toast) {
        // Add to front (most recent on top)
        self.toasts.insert(0, toast);
        // Limit total toasts
        if self.toasts.len() > MAX_TOASTS {
            self.toasts.pop();
        }
    }

    /// Remove expired toasts, called each frame
    pub fn cleanup(&mut self) {
        self.toasts.retain(|t| !t.is_expired());
    }

    pub fn toasts(&self) -> &[Toast] {
        &self.toasts
    }

    // Convenience methods for triggering toasts
    pub fn error(&mut self, message: impl Into<String>) {
        self.push(Toast::new(ToastType::Error, message));
    }

    pub fn success(&mut self, message: impl Into<String>) {
        self.push(Toast::new(ToastType::Success, message));
    }

    pub fn info(&mut self, message: impl Into<String>) {
        self.push(Toast::new(ToastType::Info, message));
    }
}
