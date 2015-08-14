use std::os::raw::{c_int, c_char, c_void};
use std::path::Path;
use std::ffi::{CString, CStr};
use std::ptr::null;

#[repr(C)]
struct GVCPtr(*const c_void);

impl Copy for GVCPtr {}
impl Clone for GVCPtr {
    fn clone(&self) -> Self { GVCPtr(self.0) }
}

#[repr(C)]
struct AgraphPtr(*const c_void);

impl Copy for AgraphPtr {}
impl Clone for AgraphPtr {
    fn clone(&self) -> Self { AgraphPtr(self.0) }
}

#[repr(C)]
struct AgsymPtr(*const c_void);

impl Clone for AgsymPtr {
    fn clone(&self) -> Self { AgsymPtr(self.0) }
}

const AGRAPH: c_int	= 0; /* can't exceed 2 bits. see Agtag_t. */
const AGNODE: c_int = 1;
const AGOUTEDGE: c_int = 2;
//const AGINEDGE: c_int = 3; /* (1 << 1) indicates an edge tag.   */
const AGEDGE: c_int = AGOUTEDGE; /* synonym in object kind args */

#[link(name = "cgraph")]
extern {
    // Agraph_t *agmemread(char*);
    fn agmemread(text: *const c_char) -> AgraphPtr;
    // int agclose(Agraph_t *g);
    fn agclose(g: AgraphPtr) -> c_int;
    // char *agnameof(void*);
    fn agnameof(obj: *const c_void) -> *const c_char;
    // Agsym_t *agattr(Agraph_t *g, int kind, char *name, char *value);
    fn agattr(g: AgraphPtr, kind: c_int, name: *const c_char, value: *const c_char) -> AgsymPtr;
}

pub struct Graph(AgraphPtr);

impl<T: Into<Vec<u8>>> From<T> for Graph {
    fn from(s: T) -> Self {
        Graph( unsafe {
            agmemread(CString::new(s.into()).unwrap().as_ptr())
        } )
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
    pub fn render_dot<T: AsRef<Path>>(&self, file: T) -> Result<(), RenderError> {
        let file = file.as_ref();
        unsafe {
            let res = agattr(self.0, AGNODE, CString::new("fontname").unwrap().as_ptr(), CString::new("helvetica").unwrap().as_ptr());
            if res.0 == null() {
                return Err(AgAttr);
            }
            let res = agattr(self.0, AGRAPH, CString::new("fontname").unwrap().as_ptr(), CString::new("helvetica").unwrap().as_ptr());
            if res.0 == null() {
                return Err(AgAttr);
            }
            let res = agattr(self.0, AGEDGE, CString::new("fontname").unwrap().as_ptr(), CString::new("helvetica").unwrap().as_ptr());
            if res.0 == null() {
                return Err(AgAttr);
            }
            let gvc = gvContext();
            if gvc.0.is_null() {
                return Err(ContextNull);
            }
            let res = gvLayout(gvc, self.0, CString::new("dot").unwrap().as_ptr());
            if res != 0 {
                return Err(GvLayout(res));
            }
            let res = gvRenderFilename(
                gvc,
                self.0,
                CString::new("svg").unwrap().as_ptr(),
                //file.as_os_str().to_bytes().unwrap().as_ptr(), // TODO: wait for stable
                CString::new(file.to_str().unwrap()).unwrap().as_ptr(),
            );
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
}

#[link(name = "gvc")]
extern {
    // extern GVC_t *gvContext(void);
    fn gvContext() -> GVCPtr;
    // extern int gvLayout(GVC_t *gvc, graph_t *g, char *engine);
    fn gvLayout(
        gvc: GVCPtr,
        g: AgraphPtr,
        engine: *const c_char,
    ) -> c_int;
    // extern int gvRenderFilename(GVC_t *gvc, graph_t *g, char *format, char *filename);
    fn gvRenderFilename(
        gvc: GVCPtr,
        g: AgraphPtr,
        format: *const c_char,
        filename: *const c_char,
    ) -> c_int;
    // extern int gvFreeLayout(GVC_t *gvc, graph_t *g);
    fn gvFreeLayout(
        gvc: GVCPtr,
        g: AgraphPtr,
    ) -> c_int;
    // extern int gvFreeContext(GVC_t *gvc);
    fn gvFreeContext(
        gvc: GVCPtr,
    ) -> c_int;
}
