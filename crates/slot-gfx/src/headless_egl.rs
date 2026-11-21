use std::ffi::{c_char, c_void, CString};
use std::ptr::{null, null_mut};

use libloading::Library;

use crate::fbdev::{
    egl_error, open, sym, Ptr, EGL_ALPHA_SIZE, EGL_BLUE_SIZE, EGL_CONTEXT_CLIENT_VERSION,
    EGL_GREEN_SIZE, EGL_NONE, EGL_OPENGL_ES2_BIT, EGL_OPENGL_ES_API, EGL_RED_SIZE,
    EGL_RENDERABLE_TYPE, EGL_SURFACE_TYPE,
};
use crate::surface::{GfxError, Surface, OUT_H, OUT_W};

const EGL_WIDTH: i32 = 0x3057;
const EGL_HEIGHT: i32 = 0x3056;
const EGL_PBUFFER_BIT: i32 = 0x0001;
const EGL_PLATFORM_SURFACELESS_MESA: u32 = 0x31DD;

/// A GLES2 context on Mesa's surfaceless platform, so the render tests run on Linux with no
/// display, the way `HeadlessSurface` runs them on macOS.
pub struct HeadlessSurface {
    display: Ptr,
    surface: Ptr,
    context: Ptr,
    make_current: unsafe extern "C" fn(Ptr, Ptr, Ptr, Ptr) -> u32,
    get_current_context: unsafe extern "C" fn() -> Ptr,
    get_error: unsafe extern "C" fn() -> i32,
    get_proc_address: unsafe extern "C" fn(*const c_char) -> *const c_void,
    terminate: unsafe extern "C" fn(Ptr) -> u32,
    gles: Library,
    _egl: Library,
}

impl HeadlessSurface {
    pub fn new() -> Result<Self, GfxError> {
        let egl = open("libEGL.so.1").or_else(|_| open("libEGL.so"))?;
        let gles = open("libGLESv2.so.2").or_else(|_| open("libGLESv2.so"))?;
        unsafe {
            let get_platform_display: unsafe extern "C" fn(u32, Ptr, *const isize) -> Ptr =
                sym(&egl, "eglGetPlatformDisplay")?;
            let initialize: unsafe extern "C" fn(Ptr, *mut i32, *mut i32) -> u32 =
                sym(&egl, "eglInitialize")?;
            let bind_api: unsafe extern "C" fn(u32) -> u32 = sym(&egl, "eglBindAPI")?;
            let choose_config: unsafe extern "C" fn(
                Ptr,
                *const i32,
                *mut Ptr,
                i32,
                *mut i32,
            ) -> u32 = sym(&egl, "eglChooseConfig")?;
            let create_pbuffer: unsafe extern "C" fn(Ptr, Ptr, *const i32) -> Ptr =
                sym(&egl, "eglCreatePbufferSurface")?;
            let create_context: unsafe extern "C" fn(Ptr, Ptr, Ptr, *const i32) -> Ptr =
                sym(&egl, "eglCreateContext")?;
            let get_error: unsafe extern "C" fn() -> i32 = sym(&egl, "eglGetError")?;
            let fail = |what: &str| egl_error(what, get_error());
            let make_current = sym(&egl, "eglMakeCurrent")?;
            let get_current_context = sym(&egl, "eglGetCurrentContext")?;
            let get_proc_address = sym(&egl, "eglGetProcAddress")?;
            let terminate = sym(&egl, "eglTerminate")?;

            let display = get_platform_display(EGL_PLATFORM_SURFACELESS_MESA, null_mut(), null());
            if display.is_null() {
                return Err(fail("eglGetPlatformDisplay"));
            }
            if initialize(display, null_mut(), null_mut()) == 0 {
                return Err(fail("eglInitialize"));
            }
            let mut headless = HeadlessSurface {
                display,
                surface: null_mut(),
                context: null_mut(),
                make_current,
                get_current_context,
                get_error,
                get_proc_address,
                terminate,
                gles,
                _egl: egl,
            };
            if bind_api(EGL_OPENGL_ES_API) == 0 {
                return Err(fail("eglBindAPI"));
            }
            let attrs = [
                EGL_SURFACE_TYPE,
                EGL_PBUFFER_BIT,
                EGL_RED_SIZE,
                8,
                EGL_GREEN_SIZE,
                8,
                EGL_BLUE_SIZE,
                8,
                EGL_ALPHA_SIZE,
                8,
                EGL_RENDERABLE_TYPE,
                EGL_OPENGL_ES2_BIT,
                EGL_NONE,
            ];
            let mut config: Ptr = null_mut();
            let mut found = 0;
            if choose_config(display, attrs.as_ptr(), &mut config, 1, &mut found) == 0 || found == 0
            {
                return Err(fail("eglChooseConfig"));
            }
            let size = [EGL_WIDTH, OUT_W as i32, EGL_HEIGHT, OUT_H as i32, EGL_NONE];
            headless.surface = create_pbuffer(display, config, size.as_ptr());
            if headless.surface.is_null() {
                return Err(fail("eglCreatePbufferSurface"));
            }
            let context_attrs = [EGL_CONTEXT_CLIENT_VERSION, 2, EGL_NONE];
            headless.context = create_context(display, config, null_mut(), context_attrs.as_ptr());
            if headless.context.is_null() {
                return Err(fail("eglCreateContext"));
            }
            headless.make_current()?;
            Ok(headless)
        }
    }
}

impl Surface for HeadlessSurface {
    fn make_current(&mut self) -> Result<(), GfxError> {
        let made =
            unsafe { (self.make_current)(self.display, self.surface, self.surface, self.context) };
        match made {
            0 => Err(egl_error("eglMakeCurrent", unsafe { (self.get_error)() })),
            _ => Ok(()),
        }
    }

    fn window_size(&self) -> (u32, u32) {
        (OUT_W, OUT_H)
    }

    fn swap(&mut self) -> Result<(), GfxError> {
        unsafe { gl::Flush() };
        Ok(())
    }

    fn proc_address(&self, name: &str) -> *const c_void {
        let Ok(c) = CString::new(name) else {
            return null();
        };
        match unsafe {
            self.gles
                .get::<unsafe extern "C" fn()>(c.as_bytes_with_nul())
        } {
            Ok(f) => *f as *const c_void,
            Err(_) => unsafe { (self.get_proc_address)(c.as_ptr()) },
        }
    }
}

impl Drop for HeadlessSurface {
    fn drop(&mut self) {
        unsafe {
            if !self.context.is_null() && (self.get_current_context)() == self.context {
                (self.make_current)(self.display, null_mut(), null_mut(), null_mut());
            }
            (self.terminate)(self.display);
        }
    }
}
