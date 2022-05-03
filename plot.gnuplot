set datafile separator ","
set key autotitle columnheader

set key font ",8"
set tics font ",6"
set key right center

set multiplot layout 2,2 rowsfirst
plot file using 1:2 with lines
plot file using 1:3 with lines
set size 1,0.5
plot file using 1:4 with lines
unset multiplot

pause -1