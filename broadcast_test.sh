#!/bin/bash

cargo build

bytecode=`./target/debug/xp-compiler-test`

echo $bytecode

cd lib/xp-compiler-broadcast-test
npm install
npm run dev -- -b $bytecode -n $1 -k $2
