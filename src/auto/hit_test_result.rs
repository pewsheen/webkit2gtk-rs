// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use webkit2_sys;

glib_wrapper! {
    pub struct HitTestResult(Object<webkit2_sys::WebKitHitTestResult, webkit2_sys::WebKitHitTestResultClass, HitTestResultClass>);

    match fn {
        get_type => || webkit2_sys::webkit_hit_test_result_get_type(),
    }
}

pub const NONE_HIT_TEST_RESULT: Option<&HitTestResult> = None;

pub trait HitTestResultExt: 'static {
    fn context_is_editable(&self) -> bool;

    fn context_is_image(&self) -> bool;

    fn context_is_link(&self) -> bool;

    fn context_is_media(&self) -> bool;

    fn context_is_scrollbar(&self) -> bool;

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn context_is_selection(&self) -> bool;

    fn get_context(&self) -> u32;

    fn get_image_uri(&self) -> Option<GString>;

    fn get_link_label(&self) -> Option<GString>;

    fn get_link_title(&self) -> Option<GString>;

    fn get_link_uri(&self) -> Option<GString>;

    fn get_media_uri(&self) -> Option<GString>;
}

impl<O: IsA<HitTestResult>> HitTestResultExt for O {
    fn context_is_editable(&self) -> bool {
        unsafe {
            from_glib(webkit2_sys::webkit_hit_test_result_context_is_editable(self.as_ref().to_glib_none().0))
        }
    }

    fn context_is_image(&self) -> bool {
        unsafe {
            from_glib(webkit2_sys::webkit_hit_test_result_context_is_image(self.as_ref().to_glib_none().0))
        }
    }

    fn context_is_link(&self) -> bool {
        unsafe {
            from_glib(webkit2_sys::webkit_hit_test_result_context_is_link(self.as_ref().to_glib_none().0))
        }
    }

    fn context_is_media(&self) -> bool {
        unsafe {
            from_glib(webkit2_sys::webkit_hit_test_result_context_is_media(self.as_ref().to_glib_none().0))
        }
    }

    fn context_is_scrollbar(&self) -> bool {
        unsafe {
            from_glib(webkit2_sys::webkit_hit_test_result_context_is_scrollbar(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn context_is_selection(&self) -> bool {
        unsafe {
            from_glib(webkit2_sys::webkit_hit_test_result_context_is_selection(self.as_ref().to_glib_none().0))
        }
    }

    fn get_context(&self) -> u32 {
        unsafe {
            webkit2_sys::webkit_hit_test_result_get_context(self.as_ref().to_glib_none().0)
        }
    }

    fn get_image_uri(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_hit_test_result_get_image_uri(self.as_ref().to_glib_none().0))
        }
    }

    fn get_link_label(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_hit_test_result_get_link_label(self.as_ref().to_glib_none().0))
        }
    }

    fn get_link_title(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_hit_test_result_get_link_title(self.as_ref().to_glib_none().0))
        }
    }

    fn get_link_uri(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_hit_test_result_get_link_uri(self.as_ref().to_glib_none().0))
        }
    }

    fn get_media_uri(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_hit_test_result_get_media_uri(self.as_ref().to_glib_none().0))
        }
    }
}

impl fmt::Display for HitTestResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HitTestResult")
    }
}
