#!/bin/bash
declare -a commands=("qemu-system-x86_64" "qemu-system-arm" "qemu-system-aarch64")

arraylength=${#commands[@]}

for ((i = 0; i < ${arraylength}; i++)); do
	if ! command -v ${commands[$i]} &>/dev/null; then
		echo "${commands[$i]} could not be found"
		echo "Building qemu from source"
		wget https://download.qemu.org/qemu-8.2.2.tar.xz
		tar xvJf qemu-8.2.2.tar.xz
		cd qemu-8.2.2
		./configure --enable-slirp
		make
	fi
done
