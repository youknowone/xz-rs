#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
XZ_SRC_DIR="$ROOT_DIR/vendor/xz"
BUILD_DIR="${XZ_SYS_C_TEST_BUILD_DIR:-$ROOT_DIR/target/xz-sys-c-tests}"
CARGO_BUILD_DIR="$BUILD_DIR/cargo"
CMAKE_BUILD_DIR="$BUILD_DIR/cmake"
JOBS="${JOBS:-$(getconf _NPROCESSORS_ONLN 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 2)}"

if [ -f "$CMAKE_BUILD_DIR/CMakeCache.txt" ] \
    && ! grep -Fq "CMAKE_HOME_DIRECTORY:INTERNAL=$XZ_SRC_DIR" "$CMAKE_BUILD_DIR/CMakeCache.txt"; then
    rm -rf "$CMAKE_BUILD_DIR"
fi

if command -v ninja >/dev/null 2>&1; then
    CMAKE_GENERATOR=( -G Ninja )
else
    CMAKE_GENERATOR=()
fi

case "$(uname -s)" in
    Linux)
        # Rust staticlibs require these native libraries when linked by CMake.
        CMAKE_EXE_LINKER_FLAGS="${CMAKE_EXE_LINKER_FLAGS:-} -pthread -ldl -lm -lrt"
        ;;
    *)
        CMAKE_EXE_LINKER_FLAGS="${CMAKE_EXE_LINKER_FLAGS:-}"
        ;;
esac

echo "Building xz-sys staticlib..."
CARGO_TARGET_DIR="$CARGO_BUILD_DIR" cargo build -p xz-sys --release --features parallel

RUST_LIB="$CARGO_BUILD_DIR/release/libxz_sys.a"
CMAKE_LIB="$CMAKE_BUILD_DIR/liblzma.a"

replace_liblzma_with_xz_sys() {
    cp "$RUST_LIB" "$CMAKE_LIB"

    ar t "$CMAKE_LIB" > "$BUILD_DIR/liblzma-archive-members.txt"
    if ! grep -q 'xz_sys' "$BUILD_DIR/liblzma-archive-members.txt"; then
        echo "replacement liblzma archive does not look like xz-sys" >&2
        exit 1
    fi
}

echo "Configuring upstream xz C test harness..."
cmake -S "$XZ_SRC_DIR" -B "$CMAKE_BUILD_DIR" "${CMAKE_GENERATOR[@]}" \
    -DBUILD_SHARED_LIBS=OFF \
    -DBUILD_TESTING=ON \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_EXE_LINKER_FLAGS="$CMAKE_EXE_LINKER_FLAGS" \
    -DXZ_NLS=OFF \
    -DXZ_DOXYGEN=OFF

echo "Building C liblzma target once to materialize CMake outputs..."
cmake --build "$CMAKE_BUILD_DIR" --target liblzma --parallel "$JOBS"

echo "Replacing C liblzma with xz-sys and building all C tests/tools..."
replace_liblzma_with_xz_sys
cmake --build "$CMAKE_BUILD_DIR" --parallel "$JOBS"

# Replace once more and rebuild dependents. This guarantees that CMake test
# executables and tools are linked against xz-sys even if the previous build
# regenerated liblzma.a as part of the all target.
replace_liblzma_with_xz_sys
cmake --build "$CMAKE_BUILD_DIR" --parallel "$JOBS"

echo "Running upstream xz C tests against xz-sys..."
ctest --test-dir "$CMAKE_BUILD_DIR" --output-on-failure "$@"
