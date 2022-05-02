set datafile separator ","
set key autotitle columnhead
set y2tics 0,70
set ytics nomirror
plot "data/data.csv" using 1:2 with lines axis x1y1, \
     "data/data.csv" using 1:3 with lines axis x1y2