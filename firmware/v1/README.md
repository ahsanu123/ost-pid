# Introduction 

The demo runs on an ST STM32F4-Discovery board.

The demo has been tested by using the free Codesourcery GCC-based toolchain
and YAGARTO. just modify the TRGT line in the makefile in order to use
different GCC toolchains.

# Setup Info

project use makefile to build process, to generate `compile-commands.json` you can use [bear](https://github.com/rizsotto/Bear), `bear -- make`

# Project Log 

- 2024-08-25 - try to create chibios project outside chibios repo
