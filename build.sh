#!/usr/bin/env bash

docker build . --build-arg=service=$1 -t andreymgn/rust-todo-$1 && docker push andreymgn/rust-todo-$1:latest