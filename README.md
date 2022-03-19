# Developing Zygisk Modules

This repository hosts a template zygisk module for developers to start developing Zygisk modules. Before developing Zygisk modules, you should first check out the official documentation for [Magisk Modules](https://topjohnwu.github.io/Magisk/guides.html). Do not fork this repository for your new module; either manually clone this repository.

## API

Use Zygisk-Rust-bindings: https://github.com/Kazurin-775/Zygisk-Rust-bindings
Only Support API of Version 2

below is cated from Zygisk Document:
- The canonical URL of the latest public Zygisk API is [module/jni/zygisk.hpp](https://github.com/topjohnwu/zygisk-module-sample/blob/master/module/jni/zygisk.hpp).
- The header file is self documented; directly refer to the header source code for all Zygisk API details.
- Magisk is committed to maintain backwards compatibility forever. That is, whenever there is an API update for Zygisk in a newer Magisk version, Magisk can always load Zygisk modules built for an older Zygisk API. If you do not need the new features introduced in newer API versions, feel free to stay on the older API version to maintain maximum compatibility.

| Zygisk API | Minimal Magisk |
| :--------: | :------------: |
|     2      |     24000      |
|     3      |     24300      |

## Notes

- This repository can be opened with Android Studio.

## Building

### 1. Install Rust

Get Details, visit https://www.rust-lang.org/

### 2. Install Android target for Rust

```shell
rustup target add armv7-linux-androideabi   # for arm
rustup target add i686-linux-android        # for x86
rustup target add aarch64-linux-android     # for arm64
rustup target add x86_64-linux-android      # for x86_64
```

### 3. Build Project with Gradle

Gradle build script support by: Rust-Android-Gradle: https://github.com/mozilla/rust-android-gradle

**Windows**: 

`./gradlew.bat :module:zipRelease`

**Linux**:

`./gradlew :module:zipRelease`

And then, artifact will put in `out`

## Problem

### HardCoding

In this Template Project, Cargo build a dynamic lib which named `libtemplate.so`.The name "template" is hardcoded in multi files:

```shell
/template/magisk_module/customize.sh
/module/build.gradle
/module/rust/Cargo.toml
```

So, if you want to customize a Zygote Plugin from this Project, there may be a Hard Start is waiting in the front of your path.
