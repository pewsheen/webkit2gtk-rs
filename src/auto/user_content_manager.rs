// This file was generated by gir (7484d29) from gir-files (71d73f0)
// DO NOT EDIT

#[cfg(feature = "v2_8")]
use JavascriptResult;
#[cfg(feature = "v2_6")]
use UserScript;
#[cfg(feature = "v2_6")]
use UserStyleSheet;
use ffi;
use glib;
#[cfg(feature = "v2_8")]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(feature = "v2_8")]
#[cfg(feature = "v2_8")]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(feature = "v2_8")]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(feature = "v2_8")]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct UserContentManager(Object<ffi::WebKitUserContentManager>);

    match fn {
        get_type => || ffi::webkit_user_content_manager_get_type(),
    }
}

impl UserContentManager {
    #[cfg(feature = "v2_6")]
    pub fn new() -> UserContentManager {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_user_content_manager_new())
        }
    }
}

#[cfg(feature = "v2_6")]
impl Default for UserContentManager {
    fn default() -> Self {
        Self::new()
    }
}

pub trait UserContentManagerExt {
    #[cfg(feature = "v2_6")]
    fn add_script(&self, script: &UserScript);

    #[cfg(feature = "v2_6")]
    fn add_style_sheet(&self, stylesheet: &UserStyleSheet);

    #[cfg(feature = "v2_8")]
    fn register_script_message_handler(&self, name: &str) -> bool;

    #[cfg(feature = "v2_6")]
    fn remove_all_scripts(&self);

    #[cfg(feature = "v2_6")]
    fn remove_all_style_sheets(&self);

    #[cfg(feature = "v2_8")]
    fn unregister_script_message_handler(&self, name: &str);

    #[cfg(feature = "v2_8")]
    fn connect_script_message_received<F: Fn(&Self, &JavascriptResult) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<UserContentManager> + IsA<glib::object::Object>> UserContentManagerExt for O {
    #[cfg(feature = "v2_6")]
    fn add_script(&self, script: &UserScript) {
        unsafe {
            ffi::webkit_user_content_manager_add_script(self.to_glib_none().0, script.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_6")]
    fn add_style_sheet(&self, stylesheet: &UserStyleSheet) {
        unsafe {
            ffi::webkit_user_content_manager_add_style_sheet(self.to_glib_none().0, stylesheet.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_8")]
    fn register_script_message_handler(&self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_user_content_manager_register_script_message_handler(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_6")]
    fn remove_all_scripts(&self) {
        unsafe {
            ffi::webkit_user_content_manager_remove_all_scripts(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_6")]
    fn remove_all_style_sheets(&self) {
        unsafe {
            ffi::webkit_user_content_manager_remove_all_style_sheets(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_8")]
    fn unregister_script_message_handler(&self, name: &str) {
        unsafe {
            ffi::webkit_user_content_manager_unregister_script_message_handler(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_8")]
    fn connect_script_message_received<F: Fn(&Self, &JavascriptResult) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &JavascriptResult) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "script-message-received",
                transmute(script_message_received_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v2_8")]
unsafe extern "C" fn script_message_received_trampoline<P>(this: *mut ffi::WebKitUserContentManager, js_result: *mut ffi::WebKitJavascriptResult, f: glib_ffi::gpointer)
where P: IsA<UserContentManager> {
    callback_guard!();
    let f: &&(Fn(&P, &JavascriptResult) + 'static) = transmute(f);
    f(&UserContentManager::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(js_result))
}
