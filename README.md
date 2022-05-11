# #4. Coli Sphericus

## TODO

- [ ] Добавить график размеров в `plot.gnuplot`
- [ ] Как-то улучшить 1Д????
- [ ] Презентация

## Build
```sh
cargo install wasm-pack
./build
```

## Run
Use a local server.

## Plot graphs
```sh
gnuplot -e "file='data/data-###.csv'" plot.gnuplot 
```
