---
title: Quantisation
date: 2026-07-31
math: true
---
[Article](https://newsletter.maartengrootendorst.com/p/a-visual-guide-to-quantization)

# Quantisation
## Floats and why they matter
A float has a sign bit, exponent bits, and significand (mantissa) bits. The exponent determines its dynamic range—the values it can represent—while the significand determines its precision, or the spacing between nearby values.

$$\textnormal{memory (bytes)} = \textnormal{bits per parameter} / 8 \times\textnormal{number of parameters}$$

Why does this matter? A full-precision model with N billion parameters needs roughly 4N GB of memory just to store its parameters.

Quantisation lowers the bit width, reducing precision. Moving from FP32 to FP16 also reduces dynamic range. BF16 (brain floating point) uses the same number of bits as FP16 but has roughly the same dynamic range as FP32.

## Quantisation methods
A further reduction is INT8, which represents integer values between $-127$ and $127$ in symmetric quantisation. To use it, you must map the model's parameter values into that range. One approach is to scale the largest absolute value to $127$. The quantisation error is the difference between the original value and the value recovered after converting back to a float.

Asymmetric quantisation instead maps the minimum and maximum float values, $x_{\min}$ and $x_{\max}$, to the minimum and maximum quantised values, $q_{\min}$ and $q_{\max}$. The scale is $s=(x_{\max}-x_{\min})/(q_{\max}-q_{\min})$, and the zero point is $z=\textnormal{round}(q_{\min}-x_{\min}/s)$. A value $x$ is then quantised as $q=\textnormal{round}(x/s)+z$.

## Range clipping
Clipping the range limits the effect of outliers, but it can introduce large quantisation errors if many values fall outside the clipped range.

## Weights and biases in practice
Weights usually outnumber biases, so biases can remain at higher precision. For weights, you can sweep over clipping percentiles and choose the one that minimises mean squared error (MSE) or KL divergence.

## Activations and training methods
Activations can also be quantised. Two common approaches are post-training quantisation (PTQ) and quantisation-aware training (QAT).

### PTQ
Activation quantisation can be dynamic or static. Dynamic PTQ calculates the scale and zero point from activations at inference time. Static PTQ calculates them in advance using a calibration dataset, then applies them during inference.

GPTQ is a post-training method for weights. It uses inverse-Hessian information to estimate how much quantising a weight affects the layer's output. After quantising and dequantising a weight ($x_1\rightarrow x_1'\rightarrow x_1''$), it uses the resulting error to adjust the remaining weights.

GGUF is a format for storing models, including quantised weights. Compatible runtimes can place some layers on the CPU when the whole model does not fit in GPU memory. Some GGUF quantisation schemes split weights into larger blocks and smaller sub-blocks, combining block-level scales with sub-block information.

### QAT
QAT simulates quantisation between layers during training, allowing the model to adapt to its errors. It can be more accurate than PTQ; one explanation is that it encourages wider loss minima, which are less sensitive to quantisation.

### Ternary weights
BitNet b1.58 takes low precision further by training with weights restricted to $-1$, $0$, or $1$. Three possible values require about $\log_2 3 \approx 1.58$ bits of information per weight in theory, which gives the model its name.

The broader trade-off remains the same: fewer bits reduce memory use, but the model must still perform well with the precision it has. PTQ manages that trade-off after training; QAT and approaches such as BitNet account for it during training.
