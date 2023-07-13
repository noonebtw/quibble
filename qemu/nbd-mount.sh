#!/bin/sh

user_id="$EUID"
user_shell="$SHELL"

echo "user id is    : $user_id"
echo "user shell is : $user_shell"

as_sudo () {
    modprobe nbd && \
        qemu-nbd -c /dev/nbd0 windows-btrfs.qcow2 && \
        su $(id -un "$user_id") --shell="$user_shell" && qemu-nbd -d /dev/nbd0
}

AS_SUDO=$(declare -f as_sudo)

if [ $EUID != 0 ]; then
    sudo bash -c "$AS_SUDO;user_id=$user_id user_shell=$user_shell as_sudo"
else
    as_sudo
fi

echo "goodbye!"
