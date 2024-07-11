#!/bin/sh

if [ $(stat -c '%a %n' "/dev/uinput" | cut -f1  -d " ") -ne 777 ]; then
    sudo chmod 777 "/dev/uinput"
fi

gcc -o uinput_test main.c 2>/dev/null
./uinput_test
# rm ./uinput_test
