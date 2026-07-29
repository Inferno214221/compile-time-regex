#!/usr/bin/env sh

cd bench
cargo build --release --bins
cd ..

hyperfine -N -w 5 './target/release/ct-regex-capture email \
    "me@example.com" "spam@example.com" "example@gmail.com" "dotless@email" "@example.com" "example.com"' -n "My Compile Time Regex" \
    './target/release/regex-capture email "me@example.com" "spam@example.com" "example@gmail.com" "dotless@email" "@example.com" "example.com"' -n "Regex Crate"
    
hyperfine -N -w 5 './target/release/ct-regex-capture phonenum \
    "+12123456789" "0123456789" "+9876543210" "0123321789" "567890" "+234E2"' -n "My Compile Time Regex" \
    './target/release/regex-capture phonenum "+12123456789" "0123456789" "+9876543210" "0123321789" "567890" "+234E2"' -n "Regex Crate"