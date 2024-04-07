#!/bin/bash -x

PROGRAM=$1

# rust-gdb -q -x gdb_init "$PROGRAM"
gdb-multiarch -q -x gdb_init "$PROGRAM"

