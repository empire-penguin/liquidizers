#!/bin/bash

# FUNCTION_WHITELIST="^liquid.*|^agc.*|^cvsd.*|^cbuffer.*|^window.*|^wdelay.*|^channel.*|^tvmpch.*|^dotprod.*"\
# "|^eqlms.*|^eqrls.*|^fec.*|^crc.*|^packetizer_.*|"

# TYPE_WHITELIST="^liquid.*|^agc.*|^cvsd.*|^cbuffer.*|^window.*|^wdelay.*"

# VAR_WHITELIST="^LIQUID.*|^AGC.*|^CVSD.*|^CBUFFER.*|^WINDOW.*|^WDELAY.*|^CHANNEL.*|^TVMPCH.*|^DOTPROD.*"\
# "|^EQLMS.*|^EQRLS.*|^FEC.*"


if [ -d $1 ]; then
    bindgen \
    --with-derive-default \
    --whitelist-function "*" \
    --whitelist-type "*" \
    --whitelist-var "*" \
    $1/include/liquid.h > ./liquidizers-sys/src/ffi.rs
else
    echo "Error: Directory $1 does not exist."
    exit 1
fi