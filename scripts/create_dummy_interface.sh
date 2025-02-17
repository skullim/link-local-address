#!/bin/bash

if ! ip link show dummy0 > /dev/null 2>&1; then
    sudo ip link add dummy0 type dummy
fi
