APP_NAME = sauw
APP_PACKAGE = com.kirbodevv.sauw
ASSETS = assets
RELEASE_DIR = release
TMP_DIR = release/tmp

# Platform directories
BASE_ANDROID_DIR    	= platform/android
BASE_WINDOWS_DIR    	= platform/windows
BASE_IOS_DIR        	= platform/ios

# Targets
ANDROID_TARGETS			= -t armeabi-v7a -t arm64-v8a
LINUX_TARGET 			= x86_64-unknown-linux-gnu
WIN_TARGET				= x86_64-pc-windows-msvc
IOS_TARGETS				= aarch64-apple-ios

# Android specific paths
DEBUG_APK				= app-debug.apk
RELEASE_APK				= app-release-unsigned.apk
ANDROID_DEBUG_APK		= app-debug.apk
GRADLE 					= ./$(BASE_ANDROID_DIR)/gradlew
DEBUG_ANDROID_PATH		= $(BASE_ANDROID_DIR)/app/build/outputs/apk/debug
RELEASE_ANDROID_PATH	= $(BASE_ANDROID_DIR)/app/build/outputs/apk/release
ANDROID_JNI_LIBS 		= $(BASE_ANDROID_DIR)/app/src/main/jniLibs
ANDROID_TEMP			= /data/local/tmp/

# iOS specific paths
XCODE_PROJECT			= $(BASE_IOS_DIR)/$(APP_NAME).xcodeproj

LINUX_BIN = target/$(LINUX_TARGET)/release/$(APP_NAME)
WIN_BIN   = target/$(WIN_TARGET)/release/$(APP_NAME).exe

.PHONY: all ios install-iphone build-iphone pc android linux windows clean build-android

release: clean-release release-linux release-windows release-android clean-tmp

IOS_DEVICE_ID ?= $(shell bash scripts/get_ios_device.sh)

ios: 		build-ios install-ios run-ios
android:	build-android install-android run-android


# |------------- RUN --------------|
run-pc:
	cargo run

run-ios: install-ios
	xcrun devicectl device process launch \
	--device $(IOS_DEVICE_ID) \
	$(APP_PACKAGE)

run-android: install-android
	echo "not implemented"

# |------------- INSTALL --------------|

install-ios: build-ios
	xcrun devicectl device install app \
	--device $(IOS_DEVICE_ID) \
	$(BASE_IOS_DIR)/build/Build/Products/Debug-iphoneos/$(APP_NAME).app

install-android: build-android
	adb push $(DEBUG_ANDROID_PATH)/$(DEBUG_APK) $(ANDROID_TEMP)     # Push APK to device
	adb shell pm install $(ANDROID_TEMP)/$(DEBUG_APK)               # Install APK

# [--------------- BUILD ---------------]

build-android:
	cargo ndk $(ANDROID_TARGETS) -o $(ANDROID_JNI_LIBS) build --release # Build native libraries
	$(GRADLE) -p $(BASE_ANDROID_DIR) build                              # Build APK

build-ios:
	IOS_TARGETS=$(IOS_TARGETS) xcodebuild \
	-project $(XCODE_PROJECT) \
	-scheme $(APP_NAME) \
	-configuration Debug \
	-derivedDataPath $(BASE_IOS_DIR)/build \
	-sdk iphoneos

# |------------- RELEASE --------------|

release-linux:
	cargo build --release --target $(LINUX_TARGET)
	rm -rf $(TMP_DIR)/linux
	mkdir -p $(TMP_DIR)/linux
	cp $(LINUX_BIN) $(TMP_DIR)/linux/
	cp -r $(ASSETS) $(TMP_DIR)/linux/
	mkdir -p $(RELEASE_DIR)
	tar -czf $(RELEASE_DIR)/linux-$(APP_NAME).tar.gz -C $(TMP_DIR)/linux .

release-windows:
	cargo build --release --target $(WIN_TARGET)
	rm -rf $(TMP_DIR)/windows
	mkdir -p $(TMP_DIR)/windows
	cp $(WIN_BIN) $(TMP_DIR)/windows/
	cp -r $(ASSETS) $(TMP_DIR)/windows/
	mkdir -p $(RELEASE_DIR)
	cd $(TMP_DIR)/windows && zip -r ../../windows-$(APP_NAME).zip * > /dev/null

release-android: build-android
	mkdir -p $(RELEASE_DIR)
	cp $(DEBUG_ANDROID_PATH)/$(DEBUG_APK) $(RELEASE_DIR)/android-$(APP_NAME).apk # Copy APK to release directory

# -------- CLEAN --------

clean:
	cargo clean

clean-release:
	rm -rf $(RELEASE_DIR)

clean-tmp:
	rm -rf $(TMP_DIR)
