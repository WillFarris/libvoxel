extern crate jni;
extern crate gl;

use self::jni::JNIEnv;
use self::jni::objects::JClass;
use self::jni::sys::jstring;
use jni::sys::jfloat;
use std::ffi::CString;

static mut x: f32 = 0.0;
static mut y: f32 = 0.0;

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_RustInterface_test(env: JNIEnv, _: JClass) -> jstring {
    let world_ptr = CString::new("This string was created in Rust!").unwrap();
    let output = env.new_string(world_ptr.to_str().unwrap()).expect("Couldn't create java string!");
    output.into_inner()
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_RustInterface_setXY(_: JNIEnv, nxj: jfloat, nyj: jfloat) {
    let nx: f32 = f32::from(nxj);
    x = nx / 1400.0;

    let ny: f32 = f32::from(nyj);
    y = ny / 2560.0;
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_RustInterface_onSurfaceCreated() {
    gl::load_with(|s| unsafe { std::mem::transmute(egli::egl::get_proc_address(s)) });

    crate::engine::core::init_engine();
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_RustInterface_onSurfaceChanged() {
    //gl::load_with(|s| unsafe { std::mem::transmute(egli::egl::get_proc_address(s)) });
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_RustInterface_onDrawFrame() {
    gl::ClearColor(x, y, 0.0, 0.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);
}