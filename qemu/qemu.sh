#!/bin/sh

BUILD_DIR=$1

source ./env.sh

echo "Quibble build dir: $BUILD_DIR"
echo "Windows drive path: $QUIBBLE_WINDOWS_PATH"

mkdir -vp esp/EFI/BOOT/drivers

cmake -v -DDRIVE_NAME=$QUIBBLE_WINDOWS_PATH -P qemu.cmake
cat esp/EFI/BOOT/freeldr.ini

cp -v font.ttf esp/EFI/BOOT/font.ttf
cp -v $BUILD_DIR/quibble.efi esp/EFI/BOOT/BOOTX64.EFI
cp -v $BUILD_DIR/btrfs.efi esp/EFI/BOOT/drivers/btrfs.efi

if ! [[ -f ovmf/OVMF_VARS.fd && -f ovmf/OVMF_CODE.fd ]]; then
    echo "OVMF is required for UEFI!"
    exit 1
fi

exec qemu-system-x86_64 -enable-kvm \
     -m 4G \
     -cpu host \
     -smp 2,sockets=1,dies=1,cores=2,threads=1 \
     -vga virtio \
     -no-reboot \
     -chardev stdio,mux=off,logfile=stdio.log,id=char0 \
     -serial chardev:char0 \
     -usb -device usb-mouse \
     -drive if=pflash,format=raw,readonly=on,file=ovmf/OVMF_CODE.fd \
     -drive if=pflash,format=raw,readonly=on,file=ovmf/OVMF_VARS.fd \
     -drive format=raw,file=fat:rw:esp \
     -drive format=qcow2,file=windows.qcow2
