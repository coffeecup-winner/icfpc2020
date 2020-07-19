use crate::app::*;
use crate::image::Image;
pub use crate::prelude::*;
use crate::widget::*;
use fltk_sys::window::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

#[cfg(any(target_os = "windows", target_os = "macos", target_os = "ios", target_os = "android"))]
pub type RawHandle = *mut raw::c_void;
#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
))]
pub type RawHandle = u64;

/// Creates a window widget
#[derive(WidgetExt, GroupExt, WindowExt, Debug)]
pub struct Window {
    _inner: *mut Fl_Window,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl Window {
    /// Find an Fl_Window through a raw handle. The window must have been instatiated by the app
    /// void pointer to: (Windows: HWND, X11: Xid (u64), MacOS: NSWindow)
    /// # Safety
    /// The data must be valid and is OS-dependent.
    pub unsafe fn find_by_handle(handle: RawHandle) -> Option<Window> {
        let ptr = Fl_Window_find_by_handle(mem::transmute(&handle));
        if ptr.is_null() {
            None
        } else {
            Some(Window::from_widget_ptr(ptr as *mut fltk_sys::widget::Fl_Widget))
        }
    }
}

/// Defines the window type, can be set dynamically using the set_type() method
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum WindowType {
    Normal = 240,
    Double = 241,
}

/// Creates a double window widget
#[derive(WidgetExt, GroupExt, WindowExt, Debug)]
pub struct DoubleWindow {
    _inner: *mut Fl_Double_Window,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a Menu window widget
#[derive(WidgetExt, GroupExt, WindowExt, Debug)]
pub struct MenuWindow {
    _inner: *mut Fl_Menu_Window,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// A wrapper around a raw OpenGL context
#[derive(Debug)]
pub struct GlContext {
    _inner: *mut raw::c_void,
}

impl GlContext {
    /// Create a GlContext from an opaque gl context pointer
    /// # Safety
    /// The pointer must be valid
    pub unsafe fn from_raw(ptr: *mut raw::c_void) -> GlContext {
        GlContext { _inner: ptr }
    }

    /// Returns the underlying pointer
    /// # Safety
    /// Can return multiple mutable pointers to the same object
    pub unsafe fn into_raw(self) -> *mut raw::c_void {
        self._inner
    }
}

/// Creates a OpenGL window widget
#[cfg(not(feature = "no-opengl"))]
#[derive(WidgetExt, GroupExt, WindowExt, Debug)]
pub struct GlWindow {
    _inner: *mut Fl_Gl_Window,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

#[cfg(not(feature = "no-opengl"))]
impl GlWindow {
    /// Flush window content
    pub fn flush(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Gl_Window_flush(self._inner) }
    }

    /// Returns whether the OpeGL context is still valid
    pub fn valid(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe {
            match Fl_Gl_Window_valid(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Mark the OpeGL context as still valid
    pub fn set_valid(&mut self, v: bool) {
        assert!(!self.was_deleted());
        unsafe { Fl_Gl_Window_set_valid(self._inner, v as raw::c_char) }
    }

    /// Returns whether the context is valid upon creation
    pub fn context_valid(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe {
            match Fl_Gl_Window_context_valid(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Mark the context as valid upon creation
    pub fn set_context_valid(&mut self, v: bool) {
        assert!(!self.was_deleted());
        unsafe { Fl_Gl_Window_set_context_valid(self._inner, v as raw::c_char) }
    }

    /// Returns the GlContext
    pub fn context(&self) -> Option<GlContext> {
        assert!(!self.was_deleted());
        unsafe {
            let x = Fl_Gl_Window_context(self._inner);
            if x.is_null() {
                None
            } else {
                Some(GlContext { _inner: x })
            }
        }
    }

    /// Sets the GlContext
    pub fn set_context(&mut self, ctx: GlContext, destroy_flag: bool) {
        assert!(!self.was_deleted());
        unsafe { Fl_Gl_Window_set_context(self._inner, ctx._inner, destroy_flag as i32) }
    }

    /// Swaps the back and front buffers
    pub fn swap_buffers(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Gl_Window_swap_buffers(self._inner) }
    }

    /// Sets the projection so 0,0 is in the lower left of the window
    /// and each pixel is 1 unit wide/tall.
    pub fn ortho(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Gl_Window_ortho(self._inner) }
    }

    /// Returns whether the GlWindow can do overlay
    pub fn can_do_overlay(&mut self) -> bool {
        assert!(!self.was_deleted());
        unsafe {
            match Fl_Gl_Window_can_do_overlay(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Redraws the overlay
    pub fn redraw_overlay(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Gl_Window_redraw_overlay(self._inner) }
    }

    /// Hides the overlay
    pub fn hide_overlay(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Gl_Window_hide_overlay(self._inner) }
    }

    /// Makes the overlay current
    pub fn make_overlay_current(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Gl_Window_make_overlay_current(self._inner) }
    }

    /// Returns the pixels per unit
    pub fn pixels_per_unit(&mut self) -> f32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Gl_Window_pixels_per_unit(self._inner) }
    }

    /// Gets the window's width in pixels
    pub fn pixel_w(&mut self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Gl_Window_pixel_w(self._inner) }
    }

    /// Gets the window's height in pixels
    pub fn pixel_h(&mut self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Gl_Window_pixel_h(self._inner) }
    }

    /// Get the Mode of the GlWindow
    pub fn mode(&self) -> Mode {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Gl_Window_mode(self._inner)) }
    }

    /// Set the Mode of the GlWindow
    pub fn set_mode(&mut self, mode: Mode) {
        assert!(!self.was_deleted());
        unsafe {
            Fl_Gl_Window_set_mode(self._inner, mode as i32);
        }
    }
}
