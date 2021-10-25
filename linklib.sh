cargo build --release --target aarch64-linux-android &&
cargo build --release --target x86_64-linux-android &&
cargo build --release --target i686-linux-android &&
cargo build --release --target armv7-linux-androideabi

ln -sf $HOME/Documents/code/libvoxel/target/aarch64-linux-android/release/libvoxel.so    $HOME/Documents/code/AndroidStudioProjects/VoxelGame/app/src/main/jniLibs/arm64/libvoxel.so
ln -sf $HOME/Documents/code/libvoxel/target/armv7-linux-androideabi/release/libvoxel.so  $HOME/Documents/code/AndroidStudioProjects/VoxelGame/app/src/main/jniLibs/armeabi/libvoxel.so
ln -sf $HOME/Documents/code/libvoxel/target/x86_64-linux-android/release/libvoxel.so     $HOME/Documents/code/AndroidStudioProjects/VoxelGame/app/src/main/jniLibs/x86_64/libvoxel.so
ln -sf $HOME/Documents/code/libvoxel/target/i686-linux-android/release/libvoxel.so       $HOME/Documents/code/AndroidStudioProjects/VoxelGame/app/src/main/jniLibs/x86/libvoxel.so 
