#!/bin/bash

if [ $# -eq 0 ]; then
    echo "booting existing application.."
else 
    mkdir -vp qemu/esp/EFI/BOOT
    cp $1 qemu/esp/EFI/BOOT/BOOTX64.EFI
    shift
fi

echo "$(pwd)"

qemu-system-x86_64 -accel kvm \
     -m 4G \
     -cpu host \
     -smp 2,sockets=1,dies=1,cores=2,threads=1 \
     -vga virtio \
     -nodefaults \
     -no-reboot \
     -serial stdio \
     -usb -device usb-mouse \
     -drive if=pflash,format=raw,readonly=on,file=/usr/share/ovmf/x64/OVMF_CODE.fd \
     -drive if=pflash,format=raw,readonly=on,file=/usr/share/ovmf/x64/OVMF_VARS.fd \
     -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
     -drive format=raw,file=fat:rw:qemu/esp $@ \
     -drive format=qcow2,file=qemu/windows-btrfs.qcow2

case $? in
    33)
        exit 0;;
    *)
        exit $?;;
esac

