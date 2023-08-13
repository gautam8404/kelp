#!/bin/bash


depth="$1"
fen="$2"


cargo run "$depth" "$fen"
