struct MplLayerTrain {
    weigths: 2dMatrix<f32>//Vec<Vec<f32>>
    biases: 1dMatrix<f32>
    activation_cache: 2dMatrix<f32>
}
struct MplLayerPredict {
    weigths: 2dMatrix<f32>//Vec<Vec<f32>>
    biases: 1dMatrix<f32>
}

struct droupout {
    
}

impl LayerTrait {
    
    pub fn forward(m: Matrix) -> Matrix
    pub fn backward(right: Matrix, alpha: f32)
}

impl LayerTrait for MplLayerPredict {
    pub fn forward(&mut self, m: Matrix) -> {
        self . m + b
    }
}

impl LayerTrait for MplLayerTrain {
    pub fn forward(&mut self, m: Matrix) -> {
        self.activation_cache = m
        MplLayerPredict.forward(self, m)
    }
    pub fn backward(right: Matrix, alpha: f32) {
        // check input dimension vs weights/biases dimensions
        weigths - alpha (m * activation_cache)
    }
}

impl From<MplLayerPredict> for MplLayerTrain {
    fn from(&self) {
        MplLayerPredict {
            weigths: self.weigths,
            biases: self.biases,
        }
    }
}

let train = MplLayerTrain::default()

let predict = train.into()

let train = predict.into()
