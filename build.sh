#!/bin/bash



is_clean=0
while getopts "rf" opt_sg; do
  case $opt_sg in
    f) is_clean=2 ;;
    r) is_clean=1 ;;
    ?) echo "unknown option: $opt_sg" ;;
  esac
done

pwd=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
bdir="_build"

# todo, add into meson.build
# only for test, need sudo
# cp $pwd/data/musictag.gschema.xml /usr/share/glib-2.0/schemas/
# glib-compile-schemas /usr/share/glib-2.0/schemas/

function sudo_run() {
  sudo -u root -H sh -c "$1"
}

function sync_version() {
  local ver_o="" ver_n=""
  ver_o=$(sed -n '2p' meson.build | grep -Eo "[0-9]+\.[0-9]+\.+[0-9]")
  ver_n=$(sed -n '3p' Cargo.toml | grep -Eo "[0-9]+\.[0-9]+\.+[0-9]")
  if [ "$ver_o" != "$ver_n" ]; then
    echo "change Cargo: $ver_n --> $ver_o"
    sed -i "3s/${ver_n}/${ver_o}/g" Cargo.toml
  fi
}

function rm_target() {
  local target="$pwd/$bdir/src/musictag"
  if [ -f "$target" ];then
    rm -f "$target"
  fi
}

function build_target() {
  cd "$pwd" || exit
  if [ $is_clean -eq 1 ]; then
    meson setup $bdir --reconfigure
  elif [ $is_clean -eq 2 ]; then
    rm -rf $bdir
    meson setup $bdir
  else
    meson setup $bdir
  fi
  cd "${pwd}/${bdir}" || exit
  meson compile
  cd - || exit
  cd - || exit
}

function link_target_resource() {
  local sdir="$pwd/$bdir/data" ddir="" src="" dst=""

  src="$sdir/musictag.gresource"
  ddir=$(grep "PKGDATA_DIR" "${pwd}/src/config.rs" | awk '{print $6}' | sed 's/;//g' | sed 's/"//g')
  dst="$ddir/musictag.gresource"

  if [ ! -d "$ddir" ];then
    sudo_run "mkdir -p $ddir"
  fi
  if [ ! -h "$dst" ];then
    sudo_run "ln -s $src $dst"
  fi

  sdir="$pwd/data"
  src="$sdir/com.github.nobodygx.musictag.gschema.xml"
  ddir="/usr/share/glib-2.0/schemas"
  dst="$ddir/com.github.nobodygx.musictag.gschema.xml"
  if [ ! -f "$dst" ];then
    sudo_run "cp -f $src $dst"
    cd $ddir || exit
    sudo_run "glib-compile-schemas ."
    cd - || exit
  fi
}

function run_target() {
  local target="$pwd/$bdir/src/musictag"
  if [ -f "$target" ];then
    cd "$pwd/$bdir/src" || exit
    ./musictag
    cd - || exit
  else
    echo "[error]: build failed."
  fi
}

function main() {
  sync_version

  build_target
  link_target_resource
  run_target
}

main "$@"
