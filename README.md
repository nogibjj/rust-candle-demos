### Hugging Face Candle Demos

### Installation (GPU works for most)

* Follow steps from [Hugging Face](https://huggingface.github.io/candle/guide/installation.html)
* Run `make verify` to ensure CUDA compiler driver and GPU capability

### Invoke an LLM for Starcoder

Run starcoder:
    * Checkout the repo: `git clone https://github.com/huggingface/candle.git`
    * cd into candle
    * `cargo run --example bigcode --release`

#### Troubleshooting

Necessary for Starcoder model:
`echo $HUGGING_FACE_HUB_TOKEN > $HOME/.cache/huggingface/token`

or ```pip install huggingface_hub
huggingface-cli login```

See this [issue](https://github.com/huggingface/candle/issues/350)

* Also you must allow [Gated model access](https://huggingface.co/bigcode/starcoderbase-1b)

### Invoke an LLM for Falcon

This works...
```
RUST_BACKTRACE=1 && cargo run --example falcon --release -- --prompt "which 100m sprinter won the 1984 olympics"? --use-f32
```

This does not...

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

### References

* [HUGGING_FACE_HUB_TOKEN](https://huggingface.co/docs/huggingface_hub/package_reference/environment_variables)

