# Clone Upstream
mkdir -p ./pika-installer-gtk4
rsync -av --progress ./* ./pika-installer-gtk4 --exclude ./pika-installer-gtk4
cd ./pika-installer-gtk4

# Get build deps
apt-get build-dep ./ -y
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Build package
dpkg-buildpackage --no-sign

# Move the debs to output
cd ../
mkdir -p ./output
mv ./*.deb ./output/
