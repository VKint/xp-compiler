#!/bin/bash

cargo build

bytecode=`./target/debug/xp-dynamic-test`

echo "Bytecode of the recompiled smart Contract:"
echo $bytecode

cd lib/xp-compiler-broadcast-test
npm install
npm run dev -- -b $bytecode -n $1 -k $2 --broadcast-only
