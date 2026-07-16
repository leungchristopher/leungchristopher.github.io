---
title: "Physics-informed neural networks 1"
date: 2026-04-02
---

Overfitting and generalisation are perpetual problems in neural network training. One thing we can do is regularisation, where we add $L(\textrm{regularisation})=\lambda\lVert \theta \rVert$.

A physics-informed neural network (PINN) can simply guide the shape of the neural network - a regularising prior on a universal function approximator. So we can set Loss = Data Loss + Physics-informed Loss. If we pick the function $g:X\rightarrow Y$ where the neural network $f:X\rightarrow Y$ is parameterised by $\theta$, we can use $g(x,f(x\mid\theta))$ weighted by $\lambda$ as the physics-informed loss.

If we're modelling a physical system, you might call this cheating: for me, it's just a really strong prior on what we expect our functions to look like. Perhaps you're working with some projectile data, but you've forgotten which planet you fired your projectile on: set $g$ to be your differentiable parameter, and let backpropagation find your solution!

I will put together a demo when I find time - perhaps after Monday.