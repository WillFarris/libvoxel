#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JObject, JString, JClass};
    use self::jni::sys::jstring;
    use std::ffi::CString;

    #[no_mangle]
    pub unsafe extern fn Java_org_farriswheel_voxelgame_MainActivity_test(env: JNIEnv, _: JClass) -> jstring {
        let world_ptr = CString::new("This string was created in Rust!").unwrap();
        let output = env.new_string(world_ptr.to_str().unwrap()).expect("Couldn't create java string!");
        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_org_farriswheel_voxelgame_MainActivity_rustGLSetup(env: JNIEnv, _: JClass, gl: JObject) -> jstring {
        let return_msg = if gl::Viewport::is_loaded() {
            "GL Viewport is loaded - Rust land"
        } else {
            "GL Viewport no load :( - Rust land"
        };

        let world_ptr = CString::new(return_msg).unwrap();
        let output = env.new_string(world_ptr.to_str().unwrap()).expect("Couldn't create java string!");
        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_org_farriswheel_voxelgame_MainActivity_clearColorGL(env: JNIEnv, _: JClass) {
        
        /*gl::Enable(gl::DEPTH_TEST);
        
        gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::BACK);
        gl::FrontFace(gl::CW);

        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        //gl::ClearColor(0.1, 0.4, 0.95, 1.0);
        */
    }

    #[no_mangle]
    pub unsafe extern fn Java_org_farriswheel_voxelgame_MainActivity_drawFrameGL(env: JNIEnv, _: JClass) {
        /*gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);*/
    }
}