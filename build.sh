export WASI_SDK_PATH="/Users/gentle/wasi-sdk-24.0"
export CC="${WASI_SDK_PATH}/bin/clang"
export CXX="${WASI_SDK_PATH}/bin/clang++"
export AR="${WASI_SDK_PATH}/bin/ar"
export CFLAGS="--sysroot=${WASI_SDK_PATH}/share/wasi-sysroot"
export CXXFLAGS="${CFLAGS}"
#export PKG_CONFIG_SYSROOT_DIR="${WASI_SDK_PATH}/share/wasi-sysroot"
cargo build -vv --target wasm32-wasip1 --features bundled
