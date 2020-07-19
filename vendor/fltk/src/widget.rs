use crate::image::Image;
pub use crate::prelude::*;
use fltk_sys::widget::*;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw;

/// An abstract type, shouldn't be instantiated in user code
#[derive(WidgetExt, Debug)]
pub struct Widget {
    _inner: *mut Fl_Widget,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

// /// A conversion function for internal use
// impl<W: WidgetExt> From<W> for Widget {
//     fn from(s: W) -> Self {
//         let widg: *mut Fl_Widget = s.as_widget_ptr();
//         Widget { _inner: widg }
//     }
// }

impl Widget {
    /// Initialize a Widget base from a pointer
    /// # Safety
    /// The pointer must be valid
    pub unsafe fn from_raw(ptr: *mut Fl_Widget) -> Self {
        assert!(!ptr.is_null());
        let tracker = fltk_sys::fl::Fl_Widget_Tracker_new(ptr as *mut fltk_sys::fl::Fl_Widget);
        assert!(!tracker.is_null());
        Widget {
            _inner: ptr,
            _tracker: tracker,
        }
    }

    /// Returns the inner pointer
    /// # Safety
    /// Can return multiple mutable pointers to the widget
    pub unsafe fn as_ptr(&self) -> *mut Fl_Widget {
        self._inner
    }

    /// Transform Widget base to another Widget
    /// # Safety
    /// Can be if used to downcast to an incorrect widget type
    pub unsafe fn into<W: WidgetExt>(self) -> W {
        assert!(!self.was_deleted());
        W::from_widget_ptr(self._inner)
    }
}
