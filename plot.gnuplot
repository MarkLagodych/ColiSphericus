set datafile separator ","
set key autotitle columnheader

if (IMGONLY == 1) {
    set term png size 1000,800
    set output IMG
} else {
    set term wxt title "Бактерія звичайна кругова"
    set key font ",8"
    set tics font ",6"
}

set key right center
set grid
# set boxwidth 0
set style fill solid

set multiplot layout 2,2 rowsfirst

plot FILE using 1:2 with lines
plot FILE using 1:3 with lines

if (DISTRIB == 0) {
    set size 1,0.5
}

plot FILE using 1:4 with boxes

if (DISTRIB == 1) {
    plot FILE using 1:5 with boxes
}

unset multiplot

if (IMGONLY == 0) {
    pause -1
}