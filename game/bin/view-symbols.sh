#!/bin/bash

cd "$(dirname "$0")/.."

nm -gU target/debug/libgame.dylib
