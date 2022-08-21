extern crate jni;
extern crate gl;

use crate::engine::engine::{Engine, PlayerMovement};

use self::jni::JNIEnv;
use self::jni::objects::JClass;
use self::jni::sys::jstring;
use cgmath::Vector3;
use jni::sys::{jfloat, jint, jlong, jobject};
use std::{convert::TryInto, ffi::CString};

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_initEngine(_env: JNIEnv, _: JClass, width: jint, height: jint, seed: jint, chunk_radius: jlong) -> jlong {
    android_log::init("VOXEL_ENGINE").unwrap();
    debug!("Initializing engine: {}x{} window, seed={}, radius={}", width, height, seed, chunk_radius);
    
    let engine = Engine::new(width as i32, height as i32, seed as u32, chunk_radius as isize);
    Box::into_raw(Box::new(engine)) as jlong
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_update(_env: JNIEnv, _: JClass, engine_ptr: jlong, delta_time: jfloat) {
    let engine = &mut *(engine_ptr as *mut Engine);
    //debug!("Update: dt={}s", delta_time);
    engine.update(delta_time as f32);
    engine.render();
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_pauseGame(_env: JNIEnv, _: JClass, engine_ptr: jlong) {
    if engine_ptr == 0 {
        return;
    }
    let engine = &mut *(engine_ptr as *mut Engine);
    engine.pause();
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_resumeGame(_env: JNIEnv, _: JClass, engine_ptr: jlong) {
    if engine_ptr == 0 {
        return;
    }
    let engine = &mut *(engine_ptr as *mut Engine);
    engine.resume();
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_test(env: JNIEnv, _: JClass) -> jstring {
    let world_ptr = CString::new("This string was created in Rust!").unwrap();
    let output = env.new_string(world_ptr.to_str().unwrap()).expect("Couldn't create java string!");
    output.into_inner()
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_lookAround(_env: JNIEnv, _: JClass, engine_ptr: jlong, dx: jfloat, dy: jfloat) {
    let engine = &mut *(engine_ptr as *mut Engine);
    //engine.process_mouse_input(f32::from(dx), f32::from(dy));
    engine.player_movement(PlayerMovement::Look(dx, dy));
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_moveAround(_env: JNIEnv, _: JClass, engine_ptr: jlong, dx: jfloat, dy: jfloat, dz: jfloat) {
    let engine = &mut *(engine_ptr as *mut Engine);
    //engine.process_movement_input(f32::from(dx), f32::from(dy), f32::from(dz));
    engine.player_movement(PlayerMovement::Walk(dx, dy, dz));
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_stopMoving(_env: JNIEnv, _: JClass, engine_ptr: jlong) {
    let engine = &mut *(engine_ptr as *mut Engine);
    //engine.player.stop_move();
    engine.player_movement(PlayerMovement::Stop);
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_playerJump(_env: JNIEnv, _: JClass, engine_ptr: jlong) {
    let engine = &mut *(engine_ptr as *mut Engine);
    //engine.player.jump();
    engine.player_movement(PlayerMovement::Jump);
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_breakBlock(_env: JNIEnv, _: JClass, engine_ptr: jlong) {
    let engine = &mut *(engine_ptr as *mut Engine);
    
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_placeBlock(_env: JNIEnv, _: JClass, engine_ptr: jlong) {
    let engine = &mut *(engine_ptr as *mut Engine);
    
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_prevInventory(_env: JNIEnv, _: JClass, engine_ptr: jlong) {
    let engine = &mut *(engine_ptr as *mut Engine);

}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_nextInventory(_env: JNIEnv, _: JClass, engine_ptr: jlong) {
    let engine = &mut *(engine_ptr as *mut Engine);
    
}


