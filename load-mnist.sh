#!/usr/bin/env bash
# Load the MNIST dataset into a data directory
# Usage: load-mnist.sh <data-dir>
# Example: load-mnist.sh data
set -e
if [ "$#" -ne 1 ]; then
    echo "Usage: load-mnist.sh <data-dir>"
    exit 1
fi
DATA_DIR=$1
mkdir -p $DATA_DIR
cd $DATA_DIR
if [ ! -f train-images-idx3-ubyte.gz ]; then
    wget http://yann.lecun.com/exdb/mnist/train-images-idx3-ubyte.gz
fi
if [ ! -f train-labels-idx1-ubyte.gz ]; then
    wget http://yann.lecun.com/exdb/mnist/train-labels-idx1-ubyte.gz
fi
if [ ! -f t10k-images-idx3-ubyte.gz ]; then
    wget http://yann.lecun.com/exdb/mnist/t10k-images-idx3-ubyte.gz
fi
if [ ! -f t10k-labels-idx1-ubyte.gz ]; then
    wget http://yann.lecun.com/exdb/mnist/t10k-labels-idx1-ubyte.gz
fi
gunzip -f train-images-idx3-ubyte.gz
gunzip -f train-labels-idx1-ubyte.gz
gunzip -f t10k-images-idx3-ubyte.gz
gunzip -f t10k-labels-idx1-ubyte.gz
cd -
