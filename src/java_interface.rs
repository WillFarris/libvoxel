extern crate jni;
extern crate gl;

use self::jni::JNIEnv;
use self::jni::objects::JClass;
use self::jni::sys::jstring;
use cgmath::Vector3;
use jni::sys::{jfloat, jint, jlong, jobject};
use std::ffi::CString;

use crate::engine::core::ENGINE;

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_test(env: JNIEnv, _: JClass) -> jstring {
    let world_ptr = CString::new("This string was created in Rust!").unwrap();
    let output = env.new_string(world_ptr.to_str().unwrap()).expect("Couldn't create java string!");
    output.into_inner()
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_lookAround(_env: JNIEnv, dx: jfloat, dy: jfloat) {
    if let Some(player) = ENGINE.player.as_mut() {
        player.camera.rotate_on_x_axis(f32::from(dy));
        player.camera.rotate_on_y_axis(f32::from(dx));
    }
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_moveAround(_env: JNIEnv, dx: jfloat, dy: jfloat, dz: jfloat) {
    if let Some(player) = ENGINE.player.as_mut() {
        player.move_direction(Vector3::new(f32::from(dx), f32::from(dy), f32::from(dz)))
    }
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_stopMoving(_env: JNIEnv) {
    if let Some(player) = ENGINE.player.as_mut() {
        player.stop_move()
    }
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_tick(_env: JNIEnv, elapsed_time: jlong) {
    if let Some(_player) = ENGINE.player.as_ref() {
        ENGINE.tick(elapsed_time);
    }
    
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_voxelOnSurfaceCreated(_env: JNIEnv, _gl: jobject, _config: jobject, start_time: jlong) {
    ENGINE.start_engine(start_time);
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_voxelOnSurfaceChanged(_env: JNIEnv, _width: jint, _height: jint) {
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_voxelOnDrawFrame(_env: JNIEnv, _gl: jobject, _elapsed_time: jfloat) {
    ENGINE.render();
}