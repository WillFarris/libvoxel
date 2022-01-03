#![cfg(target_os = "android")]

extern crate jni;
extern crate gl;

use self::jni::JNIEnv;
use self::jni::objects::JClass;
use self::jni::sys::jstring;
use cgmath::Vector3;
use jni::sys::{jfloat, jint, jlong, jobject};
use std::{convert::TryInto, ffi::CString};

use crate::engine::core::ENGINE;

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_test(env: JNIEnv, _: JClass) -> jstring {
    let world_ptr = CString::new("This string was created in Rust!").unwrap();
    let output = env.new_string(world_ptr.to_str().unwrap()).expect("Couldn't create java string!");
    output.into_inner()
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_initLogs(env: JNIEnv) {
    android_log::init("VOXEL_ENGINE").unwrap();
    debug!("Initialized logs");
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
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_playerJump(_env: JNIEnv) {
    if let Some(player) = ENGINE.player.as_mut() {
        player.jump()
    }
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_breakBlock(_env: JNIEnv) {
    //ENGINE.break_block();
    ENGINE.should_break_block = true;
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_placeBlock(_env: JNIEnv) {
    //ENGINE.break_block();
    ENGINE.should_interact = true;
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_prevInventory(_env: JNIEnv) {
    if let Some(player) = ENGINE.player.as_mut() {
        player.inventory.selected = std::cmp::max(player.inventory.selected - 1, 0)
    }
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_nextInventory(_env: JNIEnv) {
    if let Some(player) = ENGINE.player.as_mut() {
        player.inventory.selected = std::cmp::min(player.inventory.selected + 1, 8)
    }
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_update(_env: JNIEnv, elapsed_time: jlong) {
    if let Some(_player) = ENGINE.player.as_ref() {
        ENGINE.update(elapsed_time as f32);
    }
    
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_voxelOnSurfaceCreated(env: JNIEnv, _gl: jobject, _config: jobject, seed: jint, world_radius: jint) -> jstring {
    ENGINE.gl_setup(2560, 1440).unwrap();
    let result = ENGINE.initialize(seed.try_into().unwrap(), world_radius.try_into().unwrap());
    
    if let Err(error) = result {
        let world_ptr = CString::new(error.as_str()).unwrap();
        let output = env.new_string(world_ptr.to_str().unwrap()).expect("Couldn't create java string!");
        return output.into_inner()
    }
    env.new_string("Created surface successfully").expect("Couldn't create string").into_inner()
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_voxelOnSurfaceChanged(_env: JNIEnv, _width: jint, _height: jint) {
    
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_voxelOnDrawFrame(_env: JNIEnv, _gl: jobject, elapsed_time: jfloat) {
    ENGINE.update(f32::from(elapsed_time));
}
