### Hugging Face Candle Demos


![4 1-candle-framework-rust](https://github.com/nogibjj/rust-candle-demos/assets/58792/1b90a2e5-1343-4088-aab3-ce08134ee384)


### Installation (GPU works for most)

* Follow steps from [Hugging Face](https://huggingface.github.io/candle/guide/installation.html)
* Run `make verify` to ensure CUDA compiler driver and GPU capability

#### Verify CUDA:

* Run the following commands to verify your setup:
  * `cargo run --features cuda --example whisper --release`
  * `cargo run --features cuda --example bert --release`


This repo contains a pre-configured [GitHub .devcontainer](https://github.com/nogibjj/rust-candle-demos/tree/main/.devcontainer) that sets up CUDA for you.  It utilizes the [features shown here](https://docs.github.com/en/enterprise-cloud@latest/codespaces/developing-in-codespaces/getting-started-with-github-codespaces-for-machine-learning#configuring-nvidia-cuda-for-your-codespace).

[![Watch A Demo of Using GitHub Codespaces with Rust Candle](https://img.youtube.com/vi/ALqw6vfottY/0.jpg)](https://youtu.be/ALqw6vfottY)

### Developing in Rust

* Follow Guide Here:  https://huggingface.github.io/candle/guide/installation.html
* [![Rust Hugging Face Candle Hello World CUDA](https://img.youtube.com/vi/5vFPlv6M9Cs/0.jpg)](https://youtu.be/5vFPlv6M9Cs)


### Invoke an LLM for Starcoder

Run starcoder:
  * Checkout the repo: `git clone https://github.com/huggingface/candle.git`
  * cd into candle
  * `cargo run --features cuda --example bigcode --release -- --prompt "python function that adds two numbers"`

[![Exploring Hugging Face Starcoder in Rust](https://img.youtube.com/vi/g7WGCU3YSXc/0.jpg)](https://youtu.be/g7WGCU3YSXc)



#### Troubleshooting

Necessary for Starcoder model:
`echo $HUGGING_FACE_HUB_TOKEN > $HOME/.cache/huggingface/token`

or ```pip install huggingface_hub
huggingface-cli login```

See this [issue](https://github.com/huggingface/candle/issues/350)

* Also you must allow [Gated model access](https://huggingface.co/bigcode/starcoderbase-1b)

### Invoke an LLM for Falcon

### CUDA Falcon

cargo run --features cuda --example falcon --release -- --prompt "What is the best type of Apple to eat"?  

This is an error
```
   Compiling candle-examples v0.1.3 (/workspaces/rust-candle-demos/candle/candle-examples)
    Finished release [optimized] target(s) in 6.23s
     Running `target/release/examples/falcon --prompt 'What is the best type of Apple to eat?'`
tokenizer.json [00:00:00] [████████████████████████████████████████████████████████████████] 2.61 MiB/2.61 MiB 31.24 MiB/s (0s)
..del-00001-of-00002.safetensors [00:00:32] [█████████████████████████████████████████████] 9.27 GiB/9.27 GiB 295.19 MiB/s (0s)
..del-00002-of-00002.safetensors [00:00:16] [██████████████████████████████████████████████████████████████████████████████████████] 4.18 GiB/4.18 GiB 261.20 MiB/s (0s)retrieved the files in 81.9103885s
loaded the model in 8.3061299s
starting the inference loop
Error: DriverError(CUDA_ERROR_NOT_FOUND, "named symbol not found") when loading is_u32_bf16

```


This works...
```
RUST_BACKTRACE=1 && cargo run --example falcon --release -- --prompt "which 100m sprinter won the 1984 olympics"? --use-f32
```

This does not...because the box running this only has a <8 cap GPU.

```
codespace@codespaces-f226cf:/workspaces/rust-candle-demos/candle$ RUST_BACKTRACE=1 && cargo run --example falcon --release -- --prompt "which 100m sprinter won the 1984 olympics"? --features cuda
warning: some crates are on edition 2021 which defaults to `resolver = "2"`, but virtual workspaces default to `resolver = "1"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
    Finished release [optimized] target(s) in 0.20s
     Running `target/release/examples/falcon --prompt 'which 100m sprinter won the 1984 olympics?' --features cuda`
error: unexpected argument '--features' found

Usage: falcon <--cpu|--prompt <PROMPT>|--use-f32|--temperature <TEMPERATURE>|--seed <SEED>|--sample-len <SAMPLE_LEN>|--model-id <MODEL_ID>|--revision <REVISION>>

For more information, try '--help'.
codespace@codespaces-f226cf:/workspaces/rust-candle-demos/candle$ RUST_BACKTRACE=1 && cargo run --features cuda --example falco
n --release -- --prompt "which 100m sprinter won the 1984 olympics"? 
warning: some crates are on edition 2021 which defaults to `resolver = "2"`, but virtual workspaces default to `resolver = "1"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
   Compiling crossbeam-deque v0.8.3
   Compiling cudarc v0.9.14
   Compiling candle-examples v0.1.3 (/workspaces/rust-candle-demos/candle/candle-examples)
error: failed to run custom build command for `cudarc v0.9.14`

Caused by:
  process didn't exit successfully: `/workspaces/rust-candle-demos/candle/target/release/build/cudarc-4b11c4da84f29f3d/build-script-build` (exit status: 101)
  --- stdout
  cargo:rerun-if-changed=build.rs
  cargo:rerun-if-env-changed=CUDA_ROOT
  cargo:rerun-if-env-changed=CUDA_PATH
  cargo:rerun-if-env-changed=CUDA_TOOLKIT_ROOT_DIR

  --- stderr
  thread 'main' panicked at 'Unable to find `include/cuda.h` under any of: ["/usr", "/usr/local/cuda", "/opt/cuda", "/usr/lib/cuda", "C:/Program Files/NVIDIA GPU Computing Toolkit", "C:/CUDA"]. Set the `CUDA_ROOT` environment variable to `$CUDA_ROOT/include/cuda.h` to override path.', /usr/local/cargo/registry/src/index.crates.io-6f17d22bba15001f/cudarc-0.9.14/build.rs:21:13
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
error: failed to run custom build command for `candle-examples v0.1.3 (/workspaces/rust-candle-demos/candle/candle-examples)`

Caused by:
  process didn't exit successfully: `/workspaces/rust-candle-demos/candle/target/release/build/candle-examples-f3567e9c5827622f/build-script-build` (exit status: 1)
  --- stdout
  cargo:rerun-if-changed=build.rs

  --- stderr
  Error: cannot find include/cuda.h

```
### Verify Starcoder

```
     Running `target/release/examples/bigcode --prompt 'build a python marco polo function'`
tokenizer.json [00:00:00] [████████████████████████████████████████████████████████████████] 1.96 MiB/1.96 MiB 17.99 MiB/s (0s)
model.safetensors [00:00:13] [████████████████████████████████████████████████████████████] 4.24 GiB/4.24 GiB 321.27 MiB/s (0s)retrieved the files in 20.9290968s
loaded the model in 3.0363304s
starting the inference loop
build a python marco polo function to call the marco polo function.

```python
def marco_polo(x, y, z):
    return x + y + z
```

```python
marco_polo(1, 2, 3)
```

```python
marco_polo(x=1, y=2, z=3)
```

```python

```

### Inference on AWS

One way to do inference for Rust Candle is to use the [AWS Deep Learning AMI](https://docs.aws.amazon.com/dlami/latest/devguide/what-is-dlami.html), then remotely talk to it via VSCode + SSH.  For Rust, a good choice is the [Deep Learning Base AMI](https://docs.aws.amazon.com/dlami/latest/devguide/overview-base.html).  A good price point for performance is the [G5 Instance Type](https://aws.amazon.com/ec2/instance-types/g5/)

[![Inference on AWS via Remote SSH and VS Code](https://img.youtube.com/vi/dSPQtZaQ-BE/0.jpg)](https://youtu.be/dSPQtZaQ-BE)


#### Steps to Run on AWS

* Launch an [Accelerated Computing instance](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/accelerated-computing-instances.html)
* Select the Deep Learning Base AMI (Ubuntu)
* SSH and setup rust via [Rustup](https://rustup.rs/)  
* clone candle:  `git clone https://github.com/huggingface/candle.git`
* Install [VS Code SSH-Remote plugin](https://code.visualstudio.com/docs/remote/ssh)
* Open the candle folder in VS Code after setting up SSH

![Screenshot 2023-08-25 at 4 57 10 PM](https://github.com/nogibjj/rust-candle-demos/assets/58792/6f57943f-7665-48f6-b582-fbc2f7325835)


### Notes to get NVCC installed

* sudo apt-get install cuda-nvcc-12-2
* export PATH=$PATH:/usr/local/cuda-12.2/bin
* export LD_LIBRARY_PATH=/usr/local/cuda/lib64:$LD_LIBRARY_PATH
* sudo apt-get install cuda-toolkit-12-2

```
ls /usr/local/cuda/lib64/libnvrtc.so
ls /usr/local/cuda/lib64/libcurand.so
ls /usr/local/cuda/lib64/libcublas.so
ls /usr/local/cuda/lib64/libcublasLt.so

```

### Potential Development and Deployment Architecture

![4 7-exploring-remote-dev-aws](https://github.com/nogibjj/rust-candle-demos/assets/58792/5d524446-8017-42a9-9899-56eb9a4565f1)


### References

* [GitHub CodeSpaces CUDA](https://docs.github.com/en/codespaces/developing-in-codespaces/getting-started-with-github-codespaces-for-machine-learning)
* [HUGGING_FACE_HUB_TOKEN](https://huggingface.co/docs/huggingface_hub/package_reference/environment_variables)
* [Serverless Hosting Hugging Face](https://aws.amazon.com/blogs/compute/hosting-hugging-face-models-on-aws-lambda/)

