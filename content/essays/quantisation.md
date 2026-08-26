---
title: Quantisation
date: 2026-07-31
math: true
---
[article](https://newsletter.maartengrootendorst.com/p/a-visual-guide-to-quantization)

# Quantisation
## Floats and why it matters
A float is represented with a sign bit, exponent bits, and significand/mantissa bits. The interval on the real number line is the dynamic range, with the precision being the inter-value distance.

$$\textnormal{memory} = \textnormal{no. of bits} / 8 \times\textnormal{no. of params}$$

Why do we care? Because loading full-precision N billion parameter models requires 4N GB of memory just to store the parameters.

Quantisation lowers the bit-width, reducing granularity/precision.
If we were to go from FP32 to FP16, the dynamic range decreases. Hence, BF16 (brain-float) was invented - it uses the same number of bits as FP16 but matches FP32's dynamic range.
## Quantisation methods
A further reduction is INT8 $x\in\mathcal{Z}\in[-127,127]$. However, you need to map the range of the model's parameters into INT8. Here, symmetric and asymmetric quantisation are possible. A rather obvious way is to scale by the absolute maximum, and set that to 127. Quantisation error is then the difference in value when you return to FP32.

Asymmetric quantisation maps min and max from the float range to the min $\beta$ and max $\alpha$ of the quantised range. The scale factor is given by $s=R/(\alpha-\beta)$, with the zero point being $z=\textnormal{round}(-s\beta)-2^{b-1}$, and the quantised $x'=\textnormal{round}(sx+z)$
## Range clipping
To prevent outliers from having an outsized effect, you can clip the dynamic range. But this results in massive quantisation errors if you have lots of outliers.
## W&B quantisation in practice
How do you know what to do? Weights usually outnumber biases, so you can keep biases at higher precision. But for the weights, you can paramter sweep a percentile of the input range (for clipping), then optimise the MSE or KL divergence.
## Activations
How about activations? Two methods dominate - post-training quantisation (PTQ) and quantisation aware training (QAT). 
### PTQ
You can also dynamically or statically quantise activations. In dynamic PTQ, take the distribution of activations in each layer and calculate the zero-point and scale factors for quantisation. In static PTQ, a calibration dataset is used to calculate $z,s$ once then perform quantisation during inference. 

GPTQ does dynamic, asymmetric quantisation. The layer's weights are converted into the inverse-Hessian, telling us how important each weight is to the layer. The first row is quantised/dequantised ($x_1\rightarrow x_1'\rightarrow x_1''$) to calculate the quantisation error, weighted by the inverse Hessian, giving $q=(x_1-x_1'')/h_1$, then redistributing the weighted error across the other weights, $x_2=x_2+qh_2$.

GPTQ takes the whole model and puts it on the GPU, but you can use GGUF to offload a layer of the LLM to the CPU (bypassing the insufficient VRAM problem). The general idea is to split up a weight block into super and sub blocks, and use a scale factor from the super combined with the information of the sub to quantise. 
### QAT
QAT will be more accurate than PTQ. In the training process, pseudo-quantisation occurs between layers. The idea is that this helps find wide loss minima, which reduces quantisation errors.
### Taking it to the max (min)
1.58B bitnets! $(-1,0,1)$