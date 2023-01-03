#!/bin/sh

BUILD_DIR=$1
qemu_dir=$2

ovmf_vars=$qemu_dir/ovmf/OVMF_VARS.fd
ovmf_code=$qemu_dir/ovmf/OVMF_CODE.fd
esp=$qemu_dir/esp

source $qemu_dir/env.sh

# windows_fs_path=""
case $WINDOWS_FS in
    /*) windows_fs_path="$WINDOWS_FS"
        ;;
    *) windows_fs_path="$qemu_dir/$WINDOWS_FS"
       ;;
esac

echo "Quibble build dir: $BUILD_DIR"
echo "QEMU dir: $qemu_dir"
echo "OVMF code: $ovmf_code"
echo "OVMF vars: $ovmf_vars"
echo "ESP directory: $esp"
echo "Windows drive path: $windows_fs_path"
echo "Windows drive path: $QUIBBLE_WINDOWS_PATH"

mkdir -vp $esp/EFI/BOOT/drivers

cmake -v -DDRIVE_NAME=$QUIBBLE_WINDOWS_PATH -DESP=$esp -P qemu.cmake
cat $esp/EFI/BOOT/freeldr.ini

cp -v font.ttf $esp/EFI/BOOT/font.ttf
cp -v $BUILD_DIR/quibble.efi $esp/EFI/BOOT/BOOTX64.EFI
cp -v $BUILD_DIR/btrfs.efi $esp/EFI/BOOT/drivers/btrfs.efi

if ! [[ -f $ovmf_vars && -f $ovmf_code ]]; then
    echo "OVMF is required for UEFI!"
    exit 1
fi

exec qemu-system-x86_64 -enable-kvm \
     -m 4G \
     -cpu host \
     -smp 2,sockets=1,dies=1,cores=2,threads=1 \
     -vga virtio \
     -no-reboot \
     -chardev file,mux=off,path=chardev.log,id=char0 \
     -serial chardev:char0 \
     -usb -device usb-mouse \
     -drive if=pflash,format=raw,readonly=on,file=$ovmf_code \
     -drive if=pflash,format=raw,readonly=on,file=$ovmf_vars \
     -drive format=raw,file=fat:rw:$esp \
     -drive format=qcow2,file=$windows_fs_path
