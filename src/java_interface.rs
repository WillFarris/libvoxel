extern crate jni;
extern crate gl;

use crate::engine::engine::Engine;

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
    //debug!("Delta time: {}s", delta_time);
    engine.update(delta_time as f32);
    engine.render();
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
    engine.player.camera.rotate_on_x_axis(f32::from(dy));
    engine.player.camera.rotate_on_y_axis(f32::from(dx));
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_moveAround(_env: JNIEnv, _: JClass, engine_ptr: jlong, dx: jfloat, dy: jfloat, dz: jfloat) {
    let engine = &mut *(engine_ptr as *mut Engine);
    engine.player.move_direction(Vector3::new(f32::from(dx), f32::from(dy), f32::from(dz)));
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_stopMoving(_env: JNIEnv, _: JClass, engine_ptr: jlong) {
    let engine = &mut *(engine_ptr as *mut Engine);
    engine.player.stop_move();
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_playerJump(_env: JNIEnv, _: JClass, engine_ptr: jlong) {
    let engine = &mut *(engine_ptr as *mut Engine);
    engine.player.jump();
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_breakBlock(_env: JNIEnv, _: JClass, engine_ptr: jlong) {
    let engine = &mut *(engine_ptr as *mut Engine);
    //ENGINE.break_block();
    //ENGINE.should_break_block = true;
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_placeBlock(_env: JNIEnv, _: JClass, engine_ptr: jlong) {
    let engine = &mut *(engine_ptr as *mut Engine);
    //ENGINE.break_block();
    //ENGINE.should_interact = true;
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_prevInventory(_env: JNIEnv, _: JClass, engine_ptr: jlong) {
    let engine = &mut *(engine_ptr as *mut Engine);
    /*if let Some(player) = ENGINE.player.as_mut() {
        player.inventory.selected = std::cmp::max(player.inventory.selected - 1, 0)
    }*/
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_nextInventory(_env: JNIEnv, _: JClass, engine_ptr: jlong) {
    let engine = &mut *(engine_ptr as *mut Engine);
    /* if let Some(player) = ENGINE.player.as_mut() {
        player.inventory.selected = std::cmp::min(player.inventory.selected + 1, 8)
    } */
}


