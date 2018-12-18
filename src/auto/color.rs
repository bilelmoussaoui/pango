// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::GString;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Color(Boxed<ffi::PangoColor>);

    match fn {
        copy => |ptr| ffi::pango_color_copy(mut_override(ptr)),
        free => |ptr| ffi::pango_color_free(ptr),
        get_type => || ffi::pango_color_get_type(),
    }
}

impl Color {
    pub fn parse(&mut self, spec: &str) -> bool {
        unsafe {
            from_glib(ffi::pango_color_parse(self.to_glib_none_mut().0, spec.to_glib_none().0))
        }
    }

    fn to_string(&self) -> GString {
        unsafe {
            from_glib_full(ffi::pango_color_to_string(self.to_glib_none().0))
        }
    }
}

impl fmt::Display for Color {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
