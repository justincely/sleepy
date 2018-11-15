# Sleepy Microservice
[![Build Status](https://travis-ci.org/justincely/sleepy.svg?branch=master)](https://travis-ci.org/justincely/sleepy)

This is a really contrived microservice playing with rust.

## Build

`docker build -t justincely/sleepy .`

## Run
- `docker run -p 80:80 justincely/sleepy`
- `curl localhost:80/sleep/5`
