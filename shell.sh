#!/bin/sh

mkdir /mnt/FizzBuzz
/root/target/debug/fizz_buzz_fs /mnt/FizzBuzz &

exec "$@"
