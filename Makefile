.PHONY: all pc android linux windows clean

# -------- PC --------

pc:
	cargo run

# -------- ANDROID --------

android:
	cargo ndk -t armeabi-v7a -t arm64-v8a \
		-o android/app/src/main/jniLibs build --release
	./android/gradlew -p android build
	adb push android/app/build/outputs/apk/debug/app-debug.apk /data/local/tmp/
	adb shell pm install /data/local/tmp/app-debug.apk

# -------- DESKTOP BUILDS --------

linux:
	cargo build --release --target=x86_64-unknown-linux-gnu

windows:
	cargo build --release --target=x86_64-pc-windows-gnu

# -------- CLEAN --------

clean:
	cargo clean
