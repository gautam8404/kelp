#!/bin/bash


depth="$1"
fen="$2"


cargo run --release -q --bin kelp_perft "$depth" "$fen"
