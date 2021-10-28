extern crate jni;
extern crate gl;

use self::jni::JNIEnv;
use self::jni::objects::JClass;
use self::jni::sys::jstring;
use jni::sys::jfloat;
use std::ffi::CString;

use crate::engine::core::ENGINE;

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_RustInterface_test(env: JNIEnv, _: JClass) -> jstring {
    let world_ptr = CString::new("This string was created in Rust!").unwrap();
    let output = env.new_string(world_ptr.to_str().unwrap()).expect("Couldn't create java string!");
    output.into_inner()
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_RustInterface_setXY(_env: JNIEnv, nxj: jfloat, nyj: jfloat) {
    ENGINE.set_xy(f32::from(nxj), f32::from(nyj));
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_RustInterface_onSurfaceCreated() {
    ENGINE.start_engine();
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_RustInterface_onSurfaceChanged() {
    //gl::load_with(|s| unsafe { std::mem::transmute(egli::egl::get_proc_address(s)) });
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_RustInterface_onDrawFrame() {
    ENGINE.render();
}