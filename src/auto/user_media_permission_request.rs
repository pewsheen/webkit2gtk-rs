// This file was generated by gir (7484d29) from gir-files (71d73f0)
// DO NOT EDIT

use PermissionRequest;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct UserMediaPermissionRequest(Object<ffi::WebKitUserMediaPermissionRequest>): PermissionRequest;

    match fn {
        get_type => || ffi::webkit_user_media_permission_request_get_type(),
    }
}

pub trait UserMediaPermissionRequestExt {
    fn get_property_is_for_audio_device(&self) -> bool;

    fn get_property_is_for_video_device(&self) -> bool;

    fn connect_property_is_for_audio_device_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_is_for_video_device_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<UserMediaPermissionRequest> + IsA<glib::object::Object>> UserMediaPermissionRequestExt for O {
    fn get_property_is_for_audio_device(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "is-for-audio-device".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_is_for_video_device(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "is-for-video-device".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn connect_property_is_for_audio_device_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::is-for-audio-device",
                transmute(notify_is_for_audio_device_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_is_for_video_device_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::is-for-video-device",
                transmute(notify_is_for_video_device_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_is_for_audio_device_trampoline<P>(this: *mut ffi::WebKitUserMediaPermissionRequest, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<UserMediaPermissionRequest> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&UserMediaPermissionRequest::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_is_for_video_device_trampoline<P>(this: *mut ffi::WebKitUserMediaPermissionRequest, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<UserMediaPermissionRequest> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&UserMediaPermissionRequest::from_glib_borrow(this).downcast_unchecked())
}
