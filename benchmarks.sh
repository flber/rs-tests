#!/bin/bash

hyperfine --warmup 3 -r 100 "./target/release/rs-tests iter"
hyperfine --warmup 3 -r 100 "./target/release/rs-tests for"
hyperfine --warmup 3 -r 100 "./target/release/rs-tests bytes"
