#!/bin/bash
./build
objdump -M intel -d ./target/x86_64-blog_os/debug/blog_os > ./asm.S
