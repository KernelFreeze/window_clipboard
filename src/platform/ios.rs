use crate::ClipboardProvider;

use raw_window_handle::HasRawWindowHandle;
use std::error::Error;

pub fn new_clipboard<W: HasRawWindowHandle>(
    _window: &W,
) -> Result<Box<dyn ClipboardProvider>, Box<dyn Error>> {
    Ok(Box::new(clipboard_ios::Clipboard::new()?))
}

pub struct Clipboard;

impl Clipboard {
    pub fn new() -> Result<Clipboard, Box<dyn Error>> {
        Ok(Self)
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum iOSClipboardError {
    Unimplemented,
}

impl std::fmt::Display for iOSClipboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unimplemented")
    }
}

impl Error for iOSClipboardError {}

impl ClipboardProvider for clipboard_ios::Clipboard {
    fn read(&self) -> Result<String, Box<dyn Error>> {
        Err(Box::new(clipboard_ios::iOSClipboardError::Unimplemented))
    }
}