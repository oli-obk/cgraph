use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::Path;

#[repr(C)]
#[derive(Copy, Clone)]
struct GVCPtr(*const c_void);

#[repr(C)]
#[derive(Copy, Clone)]
struct AgraphPtr(*const c_void);

#[repr(C)]
struct AgsymPtr(*const c_void);

impl Clone for AgsymPtr {
    fn clone(&self) -> Self {
        AgsymPtr(self.0)
    }
}

#[link(name = "cgraph")]
#[link(name = "gvc")]
extern "C" {
    // Agraph_t *agmemread(char*);
    fn agmemread(text: *const c_char) -> AgraphPtr;
    // int agclose(Agraph_t *g);
    fn agclose(g: AgraphPtr) -> c_int;
    // char *agnameof(void*);
    fn agnameof(obj: *const c_void) -> *const c_char;
}

pub struct Graph(AgraphPtr);

impl Graph {
    pub fn parse<T: Into<Vec<u8>>>(t: T) -> Result<Graph, RenderError> {
        let s = CString::new(t.into()).unwrap();
        let data = unsafe { agmemread(s.as_ptr()) };
        if data.0.is_null() {
            Err(RenderError::ParseError)
        } else {
            Ok(Graph(data))
        }
    }
}

impl Drop for Graph {
    fn drop(&mut self) {
        unsafe {
            assert_eq!(agclose(self.0), 0);
        }
    }
}

#[derive(Debug)]
pub enum RenderError {
    ContextNull,
    ParseError,
    GvLayout(c_int),
    GvRenderFilename(c_int),
    GvFreeLayout(c_int),
    GvFreeContext(c_int),
    AgAttr,
}

use RenderError::*;

impl Graph {
    pub fn name(&self) -> &str {
        unsafe {
            let name = agnameof((self.0).0);
            std::str::from_utf8(CStr::from_ptr(name).to_bytes()).unwrap()
        }
    }
    pub fn render_dot_to_file<T: AsRef<Path>>(&self, file: T) -> Result<(), RenderError> {
        assert!(!(self.0).0.is_null());
        let file = file.as_ref();
        unsafe {
            let gvc = gvContext();
            if gvc.0.is_null() {
                return Err(ContextNull);
            }
            let dot = CString::new("dot").unwrap();
            let res = gvLayout(gvc, self.0, dot.as_ptr());
            if res != 0 {
                return Err(GvLayout(res));
            }
            let svg = CString::new("svg").unwrap();
            let file_path = file.as_os_str().to_str().unwrap();
            let res = gvRenderFilename(gvc, self.0, svg.as_ptr(), file_path.as_ptr() as *const i8);
            if res != 0 {
                return Err(GvRenderFilename(res));
            }
            let res = gvFreeLayout(gvc, self.0);
            if res != 0 {
                return Err(GvFreeLayout(res));
            }
            let res = gvFreeContext(gvc);
            if res != 0 {
                return Err(GvFreeContext(res));
            }
        }
        Ok(())
    }
    pub fn render_dot(&self) -> Result<Vec<u8>, RenderError> {
        assert!(!(self.0).0.is_null());
        unsafe {
            let gvc = gvContext();
            if gvc.0.is_null() {
                return Err(ContextNull);
            }
            let dot = CString::new("dot").unwrap();
            let res = gvLayout(gvc, self.0, dot.as_ptr());
            if res != 0 {
                return Err(GvLayout(res));
            }
            let svg = CString::new("svg").unwrap();
            let mut ptr: *const c_char = std::ptr::null();
            let mut len: c_int = 0;
            let res = gvRenderData(gvc, self.0, svg.as_ptr(), &mut ptr, &mut len);
            if res != 0 {
                return Err(GvRenderFilename(res));
            }
            let data = std::slice::from_raw_parts(ptr as *const u8, len as usize).to_owned();
            gvFreeRenderData(ptr);
            let res = gvFreeLayout(gvc, self.0);
            if res != 0 {
                return Err(GvFreeLayout(res));
            }
            let res = gvFreeContext(gvc);
            if res != 0 {
                return Err(GvFreeContext(res));
            }
            Ok(data)
        }
    }
}

#[link(name = "gvc")]
extern "C" {
    // extern GVC_t *gvContext(void);
    fn gvContext() -> GVCPtr;
    // extern int gvLayout(GVC_t *gvc, graph_t *g, char *engine);
    fn gvLayout(gvc: GVCPtr, g: AgraphPtr, engine: *const c_char) -> c_int;
    // extern int gvRenderFilename(GVC_t *gvc, graph_t *g, char *format, char *filename);
    fn gvRenderFilename(
        gvc: GVCPtr,
        g: AgraphPtr,
        format: *const c_char,
        filename: *const c_char,
    ) -> c_int;
    // int gvRenderData (GVC_t *gvc, graph_t *g, char *format, char **result)
    fn gvRenderData(
        gvc: GVCPtr,
        g: AgraphPtr,
        format: *const c_char,
        buffer: *mut *const c_char,
        length: *mut c_int,
    ) -> c_int;
    // extern int gvFreeLayout(GVC_t *gvc, graph_t *g);
    fn gvFreeLayout(gvc: GVCPtr, g: AgraphPtr) -> c_int;
    // extern int gvFreeContext(GVC_t *gvc);
    fn gvFreeContext(gvc: GVCPtr) -> c_int;
    // gvFreeRenderData
    fn gvFreeRenderData(data: *const c_char) -> c_void;
}
