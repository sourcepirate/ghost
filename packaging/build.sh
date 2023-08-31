#!/bin/bash

echo "Builing the debian package."
mkdir -p ghost/usr/bin
cp ../target/$1/ghost ghost/usr/bin/
dpkg-deb --build ghost
tar -cvf ghost.tar.gz ghost.deb