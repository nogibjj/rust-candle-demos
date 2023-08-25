# Setup
CUDA_PATH="/usr/local/cuda-12.2"
export PATH="$PATH:$CUDA_PATH/bin"
export LD_LIBRARY_PATH="$CUDA_PATH/lib64:$LD_LIBRARY_PATH"

# Write to ~/.bashrc
echo "export PATH=\"$PATH:$CUDA_PATH/bin\"" >> ~/.bashrc
echo "export LD_LIBRARY_PATH=\"$CUDA_PATH/lib64:\$LD_LIBRARY_PATH\"" >> ~/.bashrc
