# musictag readme

[简体中文说明](./readme_CN.md)

## description

musictag is an auxiliary tool intended to simplify ascii text manipulation, include svgbob and asciidoc, the current implementation of the following features:

1. use mermaid syntax to generate svgbob, it will generate svgbob ascii text and output image(by svgbob)
2. align and beautify asciidoc tables.

## how to run

run `sh build.sh` in project root dir

> if you want to debug with cargo, the config.rs will be generated differently than the installation. so you link file with your build, which should be done with ``build.sh``, if not, you should run `ln -s $PKGDATA_DIR/musictag.gresource $PROJECTDIR/_build/data/musictag.gresource`, `PKGDATA_DIR` define in `config.rs`, `PROJECTDIR` is project folder locate

## roadmap

- [ ] svgbob
    - [x] zh-cn support
    - [x] basic arrow(left/down/up/right) support
    - [ ] subgraph support
    - [ ] multi arrow support
    - [x] preview
- [ ] asciidoc
    - [ ] beautify table
    - [ ] beautify code
    - [ ] transform from markdown
    - [ ] preview

## install

### linux

```shell
# for arch linux
paru -S musictag

# for other linux
git clone --depth=1 https://github.com/nobodygx/musictag
meson setup build
meson compile -C build
meson install -C build

# run musictag to enjoy!!
```

### windows

```shell

# Install MSYS2
pacman -S mingw-w64-x86_64-gtk4 mingw-w64-x86_64-gettext mingw-w64-x86_64-libxml2 mingw-w64-x86_64-librsvg mingw-w64-x86_64-pkgconf mingw-w64-x86_64-gcc mingw-w64-x86_64-libadwaita

# add into paths
C:\msys64\mingw64\include
C:\msys64\mingw64\bin
C:\msys64\mingw64\lib

# install rust
rustup toolchain install stable-gnu
rustup default stable-gnu

# before cargo run
# sh build.sh  # --> to build musictag, but without install
# cp data/com.github.nobodygx.musictag.gschema.xml /usr/share/glib-2.0/schemas/
# cp _build/data/musictag.gresource /mingw/share/musictag/
# cargo run to enjoy
```
