#!/usr/bin/env bash
#download libtorch v1.13.0 and extract and overwrite to libtorch with CUDA support 11.7
rm -rf libtorch
wget https://download.pytorch.org/libtorch/cu117/libtorch-cxx11-abi-shared-with-deps-1.13.1%2Bcu117.zip
unzip libtorch-cxx11-abi-shared-with-deps-1.13.1+cu117.zip
rm -rf libtorch-cxx11-abi-shared-with-deps-1.13.1+cu117.zip