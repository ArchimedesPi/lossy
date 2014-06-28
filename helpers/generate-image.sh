#!/bin/bash
# Generate a HD image with GRUB installed, and mount the EXT2 partition on /mnt for playing with and copying stuff over onto
# Usage: generate-image <filename>

dd if=/dev/zero of=$1 bs=512 count=131072

(echo n; echo p; echo 1; echo; echo; echo a; echo 1; echo w;) | fdisk $1

losetup /dev/loop0 $1
losetup /dev/loop1 $1 -o 1048576

mke2fs /dev/loop1

mount /dev/loop1 /mnt

grub-install --root-directory=/mnt --no-floppy --modules="normal part_msdos ext2 multiboot" /dev/loop0