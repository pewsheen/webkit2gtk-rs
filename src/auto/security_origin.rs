// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_16", feature = "dox"))]
use glib::GString;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use glib::translate::*;
use std::fmt;
use webkit2_sys;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SecurityOrigin(Shared<webkit2_sys::WebKitSecurityOrigin>);

    match fn {
        ref => |ptr| webkit2_sys::webkit_security_origin_ref(ptr),
        unref => |ptr| webkit2_sys::webkit_security_origin_unref(ptr),
        get_type => || webkit2_sys::webkit_security_origin_get_type(),
    }
}

impl SecurityOrigin {
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    pub fn new(protocol: &str, host: &str, port: u16) -> SecurityOrigin {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(webkit2_sys::webkit_security_origin_new(protocol.to_glib_none().0, host.to_glib_none().0, port))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    pub fn new_for_uri(uri: &str) -> SecurityOrigin {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(webkit2_sys::webkit_security_origin_new_for_uri(uri.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    pub fn get_host(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_security_origin_get_host(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    pub fn get_port(&self) -> u16 {
        unsafe {
            webkit2_sys::webkit_security_origin_get_port(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    pub fn get_protocol(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_security_origin_get_protocol(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    pub fn is_opaque(&self) -> bool {
        unsafe {
            from_glib(webkit2_sys::webkit_security_origin_is_opaque(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn to_string(&self) -> GString {
        unsafe {
            from_glib_full(webkit2_sys::webkit_security_origin_to_string(self.to_glib_none().0))
        }
    }
}

impl fmt::Display for SecurityOrigin {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
