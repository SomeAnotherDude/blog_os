#!/bin/bash
BUILD_FLAGS=''
BINARY_PATH='./target/x86_64-blog_os/debug/blog_os'

if [ "$1" == '--release' ]; then
    BUILD_FLAGS='--release'
    BINARY_PATH='./target/x86_64-blog_os/release/blog_os'
fi

./build ${BUILD_FLAGS}
objdump -M intel -d "${BINARY_PATH}" > ./asm.S
