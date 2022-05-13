#!/usr/bin/bash
gnuplot -e "FILE='data/data-$1.csv'; DISTRIB=$2; IMGONLY=$3; IMG='data/graphs/data-$1.png'" plot.gnuplot