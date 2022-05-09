# #4. Coli Sphericus

## TODO

- [x] Голодные бактерии
- [x] Продолжать расти после того, как все соседи исчезли
- [x] 3D, 1D
- [ ] Протестировать 3Д/1Д и добавить опции на страничку
- [ ] NOT BOUNDED BY DEFAULT! (изменить value для чекбокса)
- [x] График распределения размеров
- [ ] Добавить график размеров в `plot.gnuplot`
- [ ] Починить кнопку показа опций
- [ ] Презентация

## Build
```sh
cargo install wasm-pack
./build # --release
```

## Run
Use a local server.

## Plot graphs
```sh
gnuplot -e "file='data/data-v#-t#.csv'" plot.gnuplot 
```
