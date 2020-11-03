#!/bin/bash
LIBRESSL_VERSION=3.0.2
SCRIPTPATH="$( cd "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"

wget https://ftp.openbsd.org/pub/OpenBSD/LibreSSL/libressl-$LIBRESSL_VERSION.tar.gz
tar -xvzf libressl-$LIBRESSL_VERSION.tar.gz

PREFIX=$SCRIPTPATH/openssl
cd libressl-$LIBRESSL_VERSION
./configure --host=x86_64-linux-musl LDFLAGS=-lrt --prefix=$PREFIX --with-openssldir=/etc/ssl
make install-strip -j 4