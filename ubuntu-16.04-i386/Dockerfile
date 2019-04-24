FROM i386/ubuntu:16.04
LABEL maintainer "Devolutions Inc."

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update -y
RUN apt-get install -y \
    libx11-dev \
    libxtst-dev \
    libxcursor-dev \
    libxrandr-dev \
    libgtk3-dev \
    libappindicator3-dev \
    libnotify-dev \
    libssl-dev
