#!/usr/bin/env bash

BSP=rpi4 make
cp kernel8.img demo_payload_rpi4.img
make
cp kernel8.img demo_payload_rpi3.img
rm kernel8.img