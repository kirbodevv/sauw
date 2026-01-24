#!/bin/bash
cargo ndk -t armeabi-v7a -o android/app/src/main/jniLibs build --release
./android/gradlew -p android build
adb push android/app/build/outputs/apk/debug/app-debug.apk /data/local/tmp/
adb shell pm install /data/local/tmp/app-debug.apk
