// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::translate::*;
#[cfg(any(feature = "v1_38", feature = "dox"))]
use std::mem;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Matrix(Boxed<ffi::PangoMatrix>);

    match fn {
        copy => |ptr| ffi::pango_matrix_copy(mut_override(ptr)),
        free => |ptr| ffi::pango_matrix_free(ptr),
        get_type => || ffi::pango_matrix_get_type(),
    }
}

impl Matrix {
    pub fn concat(&mut self, new_matrix: &Matrix) {
        unsafe {
            ffi::pango_matrix_concat(self.to_glib_none_mut().0, new_matrix.to_glib_none().0);
        }
    }

    pub fn get_font_scale_factor(&self) -> f64 {
        unsafe {
            ffi::pango_matrix_get_font_scale_factor(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v1_38", feature = "dox"))]
    pub fn get_font_scale_factors(&self) -> (f64, f64) {
        unsafe {
            let mut xscale = mem::uninitialized();
            let mut yscale = mem::uninitialized();
            ffi::pango_matrix_get_font_scale_factors(self.to_glib_none().0, &mut xscale, &mut yscale);
            (xscale, yscale)
        }
    }

    pub fn rotate(&mut self, degrees: f64) {
        unsafe {
            ffi::pango_matrix_rotate(self.to_glib_none_mut().0, degrees);
        }
    }

    pub fn scale(&mut self, scale_x: f64, scale_y: f64) {
        unsafe {
            ffi::pango_matrix_scale(self.to_glib_none_mut().0, scale_x, scale_y);
        }
    }

    pub fn transform_distance(&self, dx: &mut f64, dy: &mut f64) {
        unsafe {
            ffi::pango_matrix_transform_distance(self.to_glib_none().0, dx, dy);
        }
    }

    //pub fn transform_pixel_rectangle<'a, P: Into<Option<&'a Rectangle>>>(&self, rect: P) {
    //    unsafe { TODO: call ffi::pango_matrix_transform_pixel_rectangle() }
    //}

    pub fn transform_point(&self, x: &mut f64, y: &mut f64) {
        unsafe {
            ffi::pango_matrix_transform_point(self.to_glib_none().0, x, y);
        }
    }

    //pub fn transform_rectangle<'a, P: Into<Option<&'a Rectangle>>>(&self, rect: P) {
    //    unsafe { TODO: call ffi::pango_matrix_transform_rectangle() }
    //}

    pub fn translate(&mut self, tx: f64, ty: f64) {
        unsafe {
            ffi::pango_matrix_translate(self.to_glib_none_mut().0, tx, ty);
        }
    }
}
