#!/usr/bin/env bash

OUTDIR="tests/expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

echo "Hello world" > $OUTDIR/hello1.txt
echo "Hello" "world" > $OUTDIR/hello2.txt
echo "Hello   world" > $OUTDIR/hello3.txt
echo -n "Hello world" > $OUTDIR/hello4.txt
