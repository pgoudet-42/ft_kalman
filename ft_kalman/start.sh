#!/usr/bin/bash


if [ $# -eq 0 ]; then
    ./imu-sensor-stream-linux &
    cargo run
elif [ "$1" == "-d" ]; then
    ./imu-sensor-stream-linux --debug -s 987 &
    cargo run
elif [ "$1" == "-t" ]; then
    for i  in {0..100}; do
        ./imu-sensor-stream-linux &
        tmp=$(cargo run)
        if echo $tmp | grep -qE "Time:\s+01:29"; then
            echo -e "\e[32mOK\033[0m"
        else
            echo -e "\e[31mKO\033[0m"
        fi
    done
else
    echo "Error: wrong argument given"
    exit 1
fi
