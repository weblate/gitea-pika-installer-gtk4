#! /bin/bash

set -e

VERSION="1.0.2"

source ./pika-build-config.sh

echo "$PIKA_BUILD_ARCH" > pika-build-arch

# Clone Upstream
mkdir -p pika-installer-gtk4
cp -rvf ./* ./pika-installer-gtk4/ || true
cd ./pika-installer-gtk4/

# Get build deps
apt-get build-dep ./ -y
apt-get install curl -y
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | CARGO_HOME=/root/.cargo sh -s -- -y

# Build package
LOGNAME=root dh_make --createorig -y -l -p pika-installer-gtk4_"$VERSION" || echo "dh-make: Ignoring Last Error"
dpkg-buildpackage --no-sign

# Move the debs to output
cd ../
mkdir -p ./output
mv ./*.deb ./output/
