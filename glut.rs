/* automatically generated by rust-bindgen */

use bindgen::{glutCreateWindow, glutDestroyWindow, glutDisplayFunc, glutGetWindow, glutInit};
use bindgen::{glutInitDisplayMode, glutPostRedisplay, glutReshapeFunc, glutReshapeWindow};
use bindgen::{glutSetWindow, glutSwapBuffers, glutTimerFunc};
use libc::*;
use dvec::DVec;
use ptr::{null, to_unsafe_ptr};
use str::to_bytes;
use task::local_data::{local_data_get, local_data_set};
use cast::reinterpret_cast;
use vec::raw::to_ptr;

/* FIXME: global variable glutStrokeRoman */

/* FIXME: global variable glutStrokeMonoRoman */

/* FIXME: global variable glutBitmap9By15 */

/* FIXME: global variable glutBitmap8By13 */

/* FIXME: global variable glutBitmapTimesRoman10 */

/* FIXME: global variable glutBitmapTimesRoman24 */

/* FIXME: global variable glutBitmapHelvetica10 */

/* FIXME: global variable glutBitmapHelvetica12 */

/* FIXME: global variable glutBitmapHelvetica18 */

pub type GLenum = i32;
pub type GLint = i32;
pub type GLfloat = f32;
pub type GLdouble = f64;

pub enum Window = c_int;

pub const DOUBLE: c_uint = 2 as c_uint;

pub fn destroy<T>(_value: ~[T]) {
    // let it drop
}

pub fn init() unsafe {
    let argc = 0 as c_int;
    let command = to_bytes(~"glut");
    let argv: (*u8, *u8) = (to_ptr(command), null());
    let argv_p = reinterpret_cast(&to_unsafe_ptr(&argv));

    glutInit(to_unsafe_ptr(&argc), argv_p);

    destroy(command);
}

pub fn create_window(name: ~str) -> Window unsafe {
    let bytes = to_bytes(name);
    return Window(glutCreateWindow(to_ptr(bytes) as *c_char));
}

pub fn destroy_window(window: Window) unsafe {
    glutDestroyWindow(*window);
}

pub fn reshape_window(window: Window, width: c_int, height: c_int) unsafe {
    let current_window = glutGetWindow();
    glutSetWindow(*window);
    glutReshapeWindow(width, height);
    glutSetWindow(current_window);
}

pub fn display_callback_tls_key(_callback: @fn@()) {
    // Empty.
}

pub extern fn display_callback() unsafe {
    let callback = local_data_get(display_callback_tls_key).get();
    (*callback)();
}

pub fn display_func(callback: fn@()) unsafe {
    local_data_set(display_callback_tls_key, @callback);
    glutDisplayFunc(display_callback);
}

pub fn timer_callback_tls_key(_callback: @DVec<fn@()>) {
    // Empty.
}

pub extern fn timer_callback(index: int) unsafe {
    let callbacks = local_data_get(timer_callback_tls_key).get();
    ((*callbacks)[index as uint])();
}

pub fn timer_func(msecs: u32, callback: fn@()) unsafe {
    let callbacks;
    match local_data_get(timer_callback_tls_key) {
        None => {
            callbacks = @DVec();
            local_data_set(timer_callback_tls_key, copy callbacks);
        }
        Some(existing_callbacks) => {
            callbacks = existing_callbacks;
        }
    }

    callbacks.push(callback);
    let index = (callbacks.len() - 1) as c_int;
    glutTimerFunc(msecs, timer_callback, index);
}

pub fn reshape_callback_tls_key(_callback: @fn@(x: c_int, y: c_int)) {
    // Empty.
}

pub extern fn reshape_callback(++width: c_int, ++height: c_int) unsafe {
    let callback = local_data_get(reshape_callback_tls_key).get();
    (*callback)(width, height);
}

pub fn reshape_func(_window: Window, callback: fn@(x: c_int, y: c_int)) unsafe {
    local_data_set(reshape_callback_tls_key, @callback);
    glutReshapeFunc(reshape_callback);
}

#[cfg(target_os="macos")]
pub fn check_loop() unsafe {
    ext::glutCheckLoop();
}

#[cfg(target_os="linux")]
pub fn check_loop() unsafe {
    ext::glutMainLoopEvent();
}

pub fn init_display_mode(mode: c_uint) unsafe {
    glutInitDisplayMode(mode);
}

pub fn swap_buffers() unsafe {
    glutSwapBuffers();
}

pub fn post_redisplay() unsafe {
    glutPostRedisplay();
}

#[cfg(target_os="macos")]
#[nolink]
#[link_args="-framework GLUT"]
pub extern mod dummy {
}

#[cfg(target_os="linux")]
#[link_name="glut"]
pub extern mod dummy {
}

#[cfg(target_os="macos")]
#[nolink]
pub extern mod ext {
    // Mac GLUT extension.
    fn glutCheckLoop();
}

#[cfg(target_os="linux")]
#[nolink]
pub extern mod ext {
    // freeglut extension.
    fn glutMainLoopEvent();
}

#[nolink]
pub extern mod bindgen {

pub fn glutInit(++argcp: *c_int, ++argv: **c_char);

pub fn glutInitDisplayMode(++mode: c_uint);

pub fn glutInitDisplayString(++string: *c_char);

pub fn glutInitWindowPosition(++x: c_int, ++y: c_int);

pub fn glutInitWindowSize(++width: c_int, ++height: c_int);

pub fn glutMainLoop();

pub fn glutCreateWindow(++title: *c_char) -> c_int;

pub fn glutCreateSubWindow(++win: c_int, ++x: c_int, ++y: c_int, ++width: c_int, ++height: c_int) -> c_int;

pub fn glutDestroyWindow(++win: c_int);

pub fn glutPostRedisplay();

pub fn glutPostWindowRedisplay(++win: c_int);

pub fn glutSwapBuffers();

pub fn glutGetWindow() -> c_int;

pub fn glutSetWindow(++win: c_int);

pub fn glutSetWindowTitle(++title: *c_char);

pub fn glutSetIconTitle(++title: *c_char);

pub fn glutPositionWindow(++x: c_int, ++y: c_int);

pub fn glutReshapeWindow(++width: c_int, ++height: c_int);

pub fn glutPopWindow();

pub fn glutPushWindow();

pub fn glutIconifyWindow();

pub fn glutShowWindow();

pub fn glutHideWindow();

pub fn glutFullScreen();

pub fn glutSetCursor(++cursor: c_int);

pub fn glutWarpPointer(++x: c_int, ++y: c_int);

pub fn glutEstablishOverlay();

pub fn glutRemoveOverlay();

pub fn glutUseLayer(++layer: GLenum);

pub fn glutPostOverlayRedisplay();

pub fn glutPostWindowOverlayRedisplay(++win: c_int);

pub fn glutShowOverlay();

pub fn glutHideOverlay();

pub fn glutCreateMenu(++arg1: *u8) -> c_int;

pub fn glutDestroyMenu(++menu: c_int);

pub fn glutGetMenu() -> c_int;

pub fn glutSetMenu(++menu: c_int);

pub fn glutAddMenuEntry(++label: *c_char, ++value: c_int);

pub fn glutAddSubMenu(++label: *c_char, ++submenu: c_int);

pub fn glutChangeToMenuEntry(++item: c_int, ++label: *c_char, ++value: c_int);

pub fn glutChangeToSubMenu(++item: c_int, ++label: *c_char, ++submenu: c_int);

pub fn glutRemoveMenuItem(++item: c_int);

pub fn glutAttachMenu(++button: c_int);

pub fn glutDetachMenu(++button: c_int);

pub fn glutDisplayFunc(++func: *u8);

pub fn glutReshapeFunc(++func: *u8);

pub fn glutKeyboardFunc(++func: *u8);

pub fn glutMouseFunc(++func: *u8);

pub fn glutMotionFunc(++func: *u8);

pub fn glutPassiveMotionFunc(++func: *u8);

pub fn glutEntryFunc(++func: *u8);

pub fn glutVisibilityFunc(++func: *u8);

pub fn glutIdleFunc(++func: *u8);

pub fn glutTimerFunc(++millis: c_uint, ++func: *u8, ++value: c_int);

pub fn glutMenuStateFunc(++func: *u8);

pub fn glutSpecialFunc(++func: *u8);

pub fn glutSpaceballMotionFunc(++func: *u8);

pub fn glutSpaceballRotateFunc(++func: *u8);

pub fn glutSpaceballButtonFunc(++func: *u8);

pub fn glutButtonBoxFunc(++func: *u8);

pub fn glutDialsFunc(++func: *u8);

pub fn glutTabletMotionFunc(++func: *u8);

pub fn glutTabletButtonFunc(++func: *u8);

pub fn glutMenuStatusFunc(++func: *u8);

pub fn glutOverlayDisplayFunc(++func: *u8);

pub fn glutWindowStatusFunc(++func: *u8);

pub fn glutKeyboardUpFunc(++func: *u8);

pub fn glutSpecialUpFunc(++func: *u8);

pub fn glutJoystickFunc(++func: *u8, ++pollInterval: c_int);

pub fn glutSetColor(++arg1: c_int, ++red: GLfloat, ++green: GLfloat, ++blue: GLfloat);

pub fn glutGetColor(++ndx: c_int, ++component: c_int) -> GLfloat;

pub fn glutCopyColormap(++win: c_int);

pub fn glutGet(++_type: GLenum) -> c_int;

pub fn glutDeviceGet(++_type: GLenum) -> c_int;

pub fn glutExtensionSupported(++name: *c_char) -> c_int;

pub fn glutGetModifiers() -> c_int;

pub fn glutLayerGet(++_type: GLenum) -> c_int;

pub fn glutGetProcAddress(++procName: *c_char) -> *c_void;

pub fn glutBitmapCharacter(++font: *c_void, ++character: c_int);

pub fn glutBitmapWidth(++font: *c_void, ++character: c_int) -> c_int;

pub fn glutStrokeCharacter(++font: *c_void, ++character: c_int);

pub fn glutStrokeWidth(++font: *c_void, ++character: c_int) -> c_int;

pub fn glutBitmapLength(++font: *c_void, ++string: *c_uchar) -> c_int;

pub fn glutStrokeLength(++font: *c_void, ++string: *c_uchar) -> c_int;

pub fn glutWireSphere(++radius: GLdouble, ++slices: GLint, ++stacks: GLint);

pub fn glutSolidSphere(++radius: GLdouble, ++slices: GLint, ++stacks: GLint);

pub fn glutWireCone(++base: GLdouble, ++height: GLdouble, ++slices: GLint, ++stacks: GLint);

pub fn glutSolidCone(++base: GLdouble, ++height: GLdouble, ++slices: GLint, ++stacks: GLint);

pub fn glutWireCube(++size: GLdouble);

pub fn glutSolidCube(++size: GLdouble);

pub fn glutWireTorus(++innerRadius: GLdouble, ++outerRadius: GLdouble, ++sides: GLint, ++rings: GLint);

pub fn glutSolidTorus(++innerRadius: GLdouble, ++outerRadius: GLdouble, ++sides: GLint, ++rings: GLint);

pub fn glutWireDodecahedron();

pub fn glutSolidDodecahedron();

pub fn glutWireTeapot(++size: GLdouble);

pub fn glutSolidTeapot(++size: GLdouble);

pub fn glutWireOctahedron();

pub fn glutSolidOctahedron();

pub fn glutWireTetrahedron();

pub fn glutSolidTetrahedron();

pub fn glutWireIcosahedron();

pub fn glutSolidIcosahedron();

pub fn glutVideoResizeGet(++param: GLenum) -> c_int;

pub fn glutSetupVideoResizing();

pub fn glutStopVideoResizing();

pub fn glutVideoResize(++x: c_int, ++y: c_int, ++width: c_int, ++height: c_int);

pub fn glutVideoPan(++x: c_int, ++y: c_int, ++width: c_int, ++height: c_int);

pub fn glutReportErrors();

pub fn glutIgnoreKeyRepeat(++ignore: c_int);

pub fn glutSetKeyRepeat(++repeatMode: c_int);

pub fn glutForceJoystickFunc();

pub fn glutGameModeString(++string: *c_char);

pub fn glutEnterGameMode() -> c_int;

pub fn glutLeaveGameMode();

pub fn glutGameModeGet(++mode: GLenum) -> c_int;

}
