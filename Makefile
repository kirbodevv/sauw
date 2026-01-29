APP_NAME = sauw
ASSETS = assets
RELEASE_DIR = release
TMP_DIR = release/tmp

LINUX_TARGET = x86_64-unknown-linux-gnu
WIN_TARGET   = x86_64-pc-windows-msvc

LINUX_BIN = target/$(LINUX_TARGET)/release/$(APP_NAME)
WIN_BIN   = target/$(WIN_TARGET)/release/$(APP_NAME).exe

.PHONY: all pc android linux windows clean

release: clean-release linux-release windows-release android-release clean-tmp

# -------- RUN PC --------

pc:
	cargo run

# -------- RUN ANDROID --------

android:
	cargo ndk -t armeabi-v7a -t arm64-v8a \
		-o android/app/src/main/jniLibs build --release
	./android/gradlew -p android build
	adb push android/app/build/outputs/apk/debug/app-debug.apk /data/local/tmp/
	adb shell pm install /data/local/tmp/app-debug.apk

# ---------- RELEASE LINUX ----------

linux-release:
	cargo build --release --target $(LINUX_TARGET)
	rm -rf $(TMP_DIR)/linux
	mkdir -p $(TMP_DIR)/linux
	cp $(LINUX_BIN) $(TMP_DIR)/linux/
	cp -r $(ASSETS) $(TMP_DIR)/linux/
	mkdir -p $(RELEASE_DIR)
	tar -czf $(RELEASE_DIR)/linux-$(APP_NAME).tar.gz -C $(TMP_DIR)/linux .

# ---------- RELEASE WINDOWS ----------

windows-release:
	cargo build --release --target $(WIN_TARGET)
	rm -rf $(TMP_DIR)/windows
	mkdir -p $(TMP_DIR)/windows
	cp $(WIN_BIN) $(TMP_DIR)/windows/
	cp -r $(ASSETS) $(TMP_DIR)/windows/
	mkdir -p $(RELEASE_DIR)
	cd $(TMP_DIR)/windows && zip -r ../../windows-$(APP_NAME).zip * > /dev/null

# --------- RELEASE ANDROID ----------

android-release:
	cargo ndk -t armeabi-v7a -t arm64-v8a \
		-o android/app/src/main/jniLibs build --release
	./android/gradlew -p android build
	cp android/app/build/outputs/apk/debug/app-debug.apk $(RELEASE_DIR)/android-$(APP_NAME).apk

# -------- CLEAN --------

clean:
	cargo clean

clean-release:
	rm -rf $(RELEASE_DIR)

clean-tmp:
	rm -rf $(TMP_DIR)
