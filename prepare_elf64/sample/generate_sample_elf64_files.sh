#!/bin/sh

set -eux

gcc -c elfsamp.c -Wall
gcc -c main.c -Wall
gcc elfsamp.o main.o -Wall -o elfsamp
