
## Installing clang+llvm

https://releases.llvm.org/download.html

wget https://releases.llvm.org/6.0.1/clang+llvm-6.0.1-x86_64-linux-gnu-ubuntu-16.04.tar.xz
tar -xf clang+llvm-6.0.1-x86_64-linux-gnu-ubuntu-16.04.tar.xz
sudo mv clang+llvm-6.0.1-x86_64-linux-gnu-ubuntu-16.04/ /opt/llvm

nano ~/.bashrc
export PATH=$PATH:/opt/llvm/bin

## Create working directory

sudo mkdir /opt/albatross
sudo chmod 777 /opt/albatross
mkdir /opt/albatross/sysroot
mkdir /opt/albatross/cmake

## Building sysroots

### ubuntu-14.04-i386

cd ubuntu-14.04-i386
docker build . -t ubuntu-14.04-i386

docker run -it -v /opt/albatross/sysroot/ubuntu-14.04-i386:/sysroot ubuntu-14.04-i386 /bin/bash
cd /sysroot && cp -R -L /lib lib && cp -R -L /usr usr
exit

cp toolchain.cmake /opt/albatross/cmake/ubuntu-14.04-i386.cmake

### ubuntu-14.04-amd64

cd ubuntu-14.04-amd64
docker build . -t ubuntu-14.04-amd64

docker run -it -v /opt/albatross/sysroot/ubuntu-14.04-amd64:/sysroot ubuntu-14.04-amd64 /bin/bash
cd /sysroot && cp -R -L /lib lib && cp -R -L /usr usr
exit

cp toolchain.cmake /opt/albatross/cmake/ubuntu-14.04-amd64.cmake

## Cross-compilation

mkdir build-i386 && cd build-i386
cmake -DCMAKE_TOOLCHAIN_FILE="/opt/albatross/cmake/ubuntu-14.04-i386.cmake" ..

mkdir build-amd64 && cd build-amd64
cmake -DCMAKE_TOOLCHAIN_FILE="/opt/albatross/cmake/ubuntu-14.04-amd64.cmake" ..
