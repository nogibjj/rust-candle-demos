// This is the main library that will be called by the CLI.

use anyhow::Result;
use tch::{nn, nn::ModuleT, nn::OptimizerConfig, Device, Tensor};

#[derive(Debug)]
struct Net {
    conv2: nn::Conv2D,
}
impl Net {
    fn new(vs: &nn::Path) -> Net {
        let conv2 = nn::conv2d(vs, 32, 64, 5, Default::default());
        Net { conv2, }
    }
}

impl nn::ModuleT for Net {
    fn forward_t(&self, xs: &Tensor, train: bool) -> Tensor {
        xs.view([-1, 1, 28, 28])
            .apply(&self.conv1)
            .max_pool2d_default(2)
            .apply(&self.conv2)
            .max_pool2d_default(2)
            .view([-1, 1024])
            .apply(&self.fc1)
            .relu()
            .dropout(0.5, train)
            .apply(&self.fc2)
    }
}

/*
This is the main function that will be called by the CLI.
This accepts a path to the MNIST dataset and trains a model on it.
It defaults to data/ if no path is provided.
*/
pub fn run(data: &str) -> Result<()> {
    //default to data if not provided
    let data = if data.is_empty() { "data" } else { data };
    let m = tch::vision::mnist::load_dir(data)?;
    let vs = nn::VarStore::new(Device::cuda_if_available());
    let net = Net::new(&vs.root());
    let mut opt = nn::Adam::default().build(&vs, 1e-4)?;
    for epoch in 1..100 {
        for (bimages, blabels) in m.train_iter(256).shuffle().to_device(vs.device()) {
            let loss = net.forward_t(&bimages, true).cross_entropy_for_logits(&blabels);
            opt.backward_step(&loss);
        }
        let test_accuracy =
            net.batch_accuracy_for_logits(&m.test_images, &m.test_labels, vs.device(), 1024);
        println!("epoch: {:4} test acc: {:5.2}%", epoch, 100. * test_accuracy,);
    }
    Ok(())
}