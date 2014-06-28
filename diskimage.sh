#!/bin/bash
# Written by Samy Pessé, modified by ArchimedesPi
qemu-img create c.img 2M
fdisk ./c.img << EOF
x
c
4
h
16
s
63
r
n
p
1
1
4
a
1
w
EOF
fdisk -l -u ./c.img
losetup -o 32256 /dev/loop1 ./c.img

mke2fs /dev/loop1
mount /dev/loop1 /mnt/

cp -R ./bootdisk/* /mnt/
mkdir -p /mnt/boot/grub
cat > /mnt/boot/grub/device.map <<EOF
(hd0) /dev/loop0
(hd0,1) /dev/loop1
EOF

mount --bind /dev /mnt/dev
chroot /mnt grub-mkconfig -o /boot/grub/grub.cfg

umount /mnt/dev
umount /mnt

losetup -d /dev/loop1

