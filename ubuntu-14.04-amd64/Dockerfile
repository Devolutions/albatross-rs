FROM ubuntu:14.04
LABEL maintainer "Devolutions Inc."

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update -y
RUN apt-get install -y \
    libx11-dev \
    libxtst-dev \
    libxcursor-dev \
    libxrandr-dev \
    libgtk2.0-0 \
    libgtk2.0-dev \
    libappindicator-dev \
    libnotify-dev
