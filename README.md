# Dungeon Crawler

Project based on `Dungeon Crawl` in [Hands-on Rust Book by Herbert Wolverson](https://github.com/thebracket/HandsOnRust)

For more info, please refer to [this website](https://hands-on-rust.com/) and [the book](https://pragprog.com/titles/hwrust/hands-on-rust/).

<br/>

A special thanks to the author 'MedicineStorm' from [opengameart.org](http://opengameart.org/content/dungeon-crawl-32x32-tiles-supplemental)

Controls:
- arrows to move
- G: grab item (potion, map)
- U: use a weapon
- 1-9: numbers to use objects

I made some changes:
- no more random level selection:
  - level 1: `Forest` with `Cellular Automata`
  - level 2: `Dungeon` with `Rooms`
  - level 3: `Ice` with `Drunkards Walk`
- new monster type at level 3
- improved HUD
- weapons in `template.ron` are defined as `Weapon` instead of `Item`
- use `U` to grab and use a weapon instead of grabbing it as an item with `G`