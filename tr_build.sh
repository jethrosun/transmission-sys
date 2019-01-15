#!/bin/env bash

# Run this file from the main Transmission folder

mkdir build

cd build

# A lot of things need to be disabled
cmake \
    -DENABLE_DAEMON=OFF \
    -DENABLE_GTK=OFF \
    -DENABLE_QT=OFF \
    -DENABLE_MAC=OFF \
    -DENABLE_UTILS=OFF \
    -DENABLE_CLI=OFF \
    -DENABLE_TESTS=OFF \
    -DENABLE_LIGHTWEIGHT=OFF \
    -DENABLE_UTP=ON \
    -DENABLE_NLS=OFF \
    -DINSTALL_DOC=OFF \
    -DINSTALL_LIB=ON \
    -DUSE_SYSTEM_EVENT2=ON \
    -DUSE_SYSTEM_DHT=OFF \
    -DUSE_SYSTEM_MINIUPNPC=OFF \
    -DUSE_SYSTEM_NATPMP=OFF \
    -DUSE_SYSTEM_UTP=OFF \
    -DUSE_SYSTEM_B64=OFF \
    -DWITH_CRYPTO=openssl \
    -DWITH_INOTIFY=OFF \
    -DWITH_KQUEUE=OFF \
    -DWITH_LIBAPPINDICATOR=OFF \
    -DWITH_SYSTEMD=OFF \
    ..

make

sudo make install
