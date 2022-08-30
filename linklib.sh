#!/bin/bash

if [ ! -d "NDK/android-ndk-r22b" ]; then
    echo "Missing  Android NDK in 'NDK/android-ndk-r22b'"
    exit -1
fi

echo "Compiling library..."

cargo build --release --target aarch64-linux-android &&
cargo build --release --target x86_64-linux-android &&
cargo build --release --target i686-linux-android &&
cargo build --release --target armv7-linux-androideabi

JNILIBS_DIR=$1/app/src/main/jniLibs
echo "Copying object files to " $JNILIBS_DIR

mkdir -p $JNILIBS_DIR/arm64
mkdir -p $JNILIBS_DIR/arm64-v8a
mkdir -p $JNILIBS_DIR/armeabi
mkdir -p $JNILIBS_DIR/arm
mkdir -p $JNILIBS_DIR/x86_64
mkdir -p $JNILIBS_DIR/x86

WD=$( pwd )

ln -sf $WD/target/aarch64-linux-android/release/libvoxel.so    $JNILIBS_DIR/arm64/libvoxel.so
ln -sf $WD/target/aarch64-linux-android/release/libvoxel.so    $JNILIBS_DIR/arm64-v8a/libvoxel.so
ln -sf $WD/target/armv7-linux-androideabi/release/libvoxel.so  $JNILIBS_DIR/armeabi/libvoxel.so
ln -sf $WD/target/armv7-linux-androideabi/release/libvoxel.so  $JNILIBS_DIR/arm/libvoxel.so
ln -sf $WD/target/x86_64-linux-android/release/libvoxel.so     $JNILIBS_DIR/x86_64/libvoxel.so
ln -sf $WD/target/i686-linux-android/release/libvoxel.so       $JNILIBS_DIR/x86/libvoxel.so

ls $JNILIBS_DIR/*
