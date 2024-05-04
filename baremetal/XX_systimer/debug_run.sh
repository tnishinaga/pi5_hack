#!/bin/bash -x

PROGRAM=$1

openocd -f interface/cmsis-dap.cfg -f ../bcm2712.cfg &
OPENOCD_PID=$!
# rust-gdb -q -x gdb_init "$PROGRAM"
gdb-multiarch -q -x gdb_init "$PROGRAM"

kill -TERM ${OPENOCD_PID}

