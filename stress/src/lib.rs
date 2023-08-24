/*A stress test that hammers the CPU and GPU using PyTorch
*/
use rayon::prelude::*;
use tch::Tensor;

//build a cpu load test function
pub fn cpu_load_test() {
    let slice = vec![0; 1_000_000];
    for i in 1..1_000_000 {
        let t = Tensor::of_slice(&slice).to_device(tch::Device::Cpu);
        println!("{} {:?}", i, t.size())
    }
}

//build a gpu load test function
pub fn gpu_load_test() {
    let slice = vec![0; 1_000_000];
    for i in 1..1_000_000 {
        let t = Tensor::of_slice(&slice).to_device(tch::Device::Cuda(0));
        println!("{} {:?}", i, t.size())
    }
}

//build a gpu load test function that uses threads via rayon iterator that sends the load to the GPU
pub fn gpu_load_test_rayon() {
    let slice = vec![0; 1_000_000];
    (1..1_000_000).into_par_iter().for_each(|i| {
        let t = Tensor::of_slice(&slice).to_device(tch::Device::Cuda(0));
        println!("{} {:?}", i, t.size())
    });
}
