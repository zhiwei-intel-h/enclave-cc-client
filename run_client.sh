#!/bin/bash
set -e

make && ./target/debug/examples/async-client
