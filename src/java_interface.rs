extern crate jni;
extern crate gl;

use self::jni::JNIEnv;
use self::jni::objects::JClass;
use self::jni::sys::jstring;
use cgmath::Vector3;
use jni::sys::{JNI_TRUE, jboolean, jfloat};
use std::ffi::CString;

use crate::engine::core::ENGINE;

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_RustInterface_test(env: JNIEnv, _: JClass) -> jstring {
    let world_ptr = CString::new("This string was created in Rust!").unwrap();
    let output = env.new_string(world_ptr.to_str().unwrap()).expect("Couldn't create java string!");
    output.into_inner()
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_RustInterface_lookAround(_env: JNIEnv, dx: jfloat, dy: jfloat) {
    if let Some(player) = ENGINE.player.as_mut() {
        player.camera.rotate_on_x_axis(f32::from(dy));
        player.camera.rotate_on_y_axis(f32::from(dx));
    }
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_RustInterface_moveAround(_env: JNIEnv, dx: jfloat, dy: jfloat, dz: jfloat, forward: jboolean) {
    if let Some(player) = ENGINE.player.as_mut() {
        player.move_direction(Vector3::new(f32::from(dx), f32::from(dy), f32::from(dz)))
    }
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_RustInterface_stopMoving(_env: JNIEnv, dx: jfloat, dy: jfloat, dz: jfloat, forward: jboolean) {
    if let Some(player) = ENGINE.player.as_mut() {
        player.stop_move()
    }
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_RustInterface_engineTick() {
    ENGINE.tick();
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
pub unsafe extern fn Java_org_farriswheel_voxelgame_RustInterface_onDrawFrame(_env: JNIEnv, elapsed_time: jfloat) {
    ENGINE.render(f32::from(elapsed_time));
}