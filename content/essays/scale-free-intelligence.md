---
title: "On scale-free intelligence"
date: 2025-12-23
math: true
---

[Paper]

Some brief notes and thoughts on intelligence and agency, taken from Richard Ngo's essay on intelligent agency ([Paper]). I do not claim originality.

First, we define an *agent* as an entity that develops an understanding of the world, and acts to exert an influence on the world. How, then, can this agent act *intelligently*?

## The universal intelligence measure
Legg and Hutter's definition of universal intelligence from 2007 reads as follows:

$$\Upsilon(\pi)=\sum_{\mu\in E}2^{-K(\mu)}V^{\pi}_{\mu}$$

Note that the universal intelligence measure, $\Upsilon$, satisfies the invariance properties of Kolmogorov complexity. The underlying computational architecture does not matter: we could encode the tasks in a Scratch-based universal Turing machine or the 'human coding language', to similar effect. Moreover, the hierarchy of problems in the scale-free architecture has scale-free properties.

## Ngo: two current theories on intelligent agency

### Expected utility maximisation (EUM)
Search and planning enable an agent to discover strategies that maximise utility in expectation. Ngo points out that EUM handles goals and beliefs in isolation, and develops heuristics through experience. He suggests that theoretical progress in deep reinforcement learning is too broad and slow to generalise the theory of EUM to describe sequential decision making.

### Active inference
This idea comes from neuroscience. Imagine that the brain is a hierarchical network. From the top downwards, each level tries to predict the behaviour of the level below it, and the bottom layer predicts sensory inputs. This model is a good analogy for both artificial neural nets and biological vision. The strength of the abstract representation at the top can disseminate throughout the network: think about how illusions clearly snap into focus when you spot the trick.

In the same line of thought, actions are formulated as strong predictions that override our default, resting states. Instead of treating goals separately, consider *active inference*. There can be a gap between our beliefs and our observations. If the cost of changing future observations is lower than the cost to update conflicting beliefs, these are goals.

What does this imply? We can borrow directly from Friston's free energy princple.

Let's say we're in a situation where taking actions $a\in\mathcal{A}$ leads to outcomes $o\in\mathcal{O}$. Active inference provides a model where we have a goal distribution $P(o)$ and expected distribution $P(o\vert a)$. We can then define the expected free energy (F): 

$$F(a)\approx D_{KL}[P(o\vert a) \parallel P(o)]=\left(\sum_o P(o\vert a)\ln P(o\vert a)\right)-\left(\sum_o P(o\vert a)\ln P(o)\right)$$

In this expression, the first term describes the value of the information we have in our model. The latter describes the expected utility of the active inference model. Hence, minimising expected free energy means maximising expected utility:
$$\min G(a)\approx\max\sum_o P(o\vert a)\ln P(o)$$

## A unified theory of intelligent agency
What if I, an agent, have multiple goals? Strategies come into play. We have studied this in multi-agent scenarios, but how do goals interact in strategic terms within an agent? Shanahan describes this in the **global workspace theory** in *Embodiment and the inner mind*. More on this later.

Ngo argues that intelligent agency must be scale-free: the models of conflict that describe inter-agent interactions should of sufficient explanatory power to describe scales both above and below this.

How can we privilege the individuality, perhaps indivisibility of agents in our theory? We can perhaps pose this as a strong constraint on the space of possible strategies accessible to the agent's sub-agents. Ngo terms this **coalitional agency**, and provides two possible routes to describing this phenomenon.

### EUM to active inference
Starting from expected utility maximisation, we run into the problem of combining EUM agents. EUM agents provide a utility function over an outcome probability distribution. Harsanyi's utilitarian theorem essentially states that for utility functions $U_i(o)$, under weak Pareto, rational conditions, the social welfare function is described as:

$$W(o)=\beta+\sum_i\alpha_i U_i(o)$$

This is a limiting welfare function: our notions of fairness and credit assignment are limited by the fact that we're performing a linear combination of their original utility functions: there is no term for the interaction between the subagents - we've essentially squashed their individuality. How can we respect incentives in a hierarchical structure?

Ngo presents a classic example in cutting cake: one cuts, one chooses the half. The framework that covers the most ground is that of **bargaining theory**. As a sub-agent in a system, I'd consider the following:

1. How do I consider and weigh decisions I might have to take in the future?
2. What level do I bargain at? Individual decisions, methods, or systems?

This is an open problem. I think that Shanahan's global workspace theory can add to this. The key idea is that the brain is a set of parallel processes competing to influence the global workspace, whose state is then broadcast back to all of the processes. How does one win this competition? Coalitions of coupled processes are attractors in state space, and the process of action and feedback constantly shifts the landscape of attractors.

This is a fine description of the problem, and I'd be interested in connecting this back to multi-level game theory. Do the Matryoshka dolls terminate? More to follow.

Returning to Ngo's argument, he believes that a theory of intelligent agency should incorporate the fact that agents should have beliefs and goals. It's difficult to find a good description for how decision procedures incorporate this fact.

### Active inference to EUM
We extend the hierarchical model described above to contain beliefs and goals. A subagent is then a consistent cluster of goals and beliefs in this agent. 

Now, what happens when two subagents disagree on their predictions? We want a system that is compatible with incentieves. Ngo points to the precision-weighted average as a mechanism. But game-theoretically we return to an impasse: we can endlessly consider what how other subagents in this game respond. By shifting to a prediction market system, we can specialise by predicting different aspects of the agent. 

In this formulation, perhaps we select an action based on the dynamics of the prediction market. We break the symmmetry of multi-layer game theory by conditioning the dynamics of the market on outcomes. If a subagent can predict some aspect of a complex plan, it can profit. But in order for this complex plan to work, it must be coordinated in full. Therefore, the incentive to implement plans allows rational subagents to take control of the whole system.

In my mind, we've arrived back at global workspace theory. In GWT, subagents can be overlapping in their goals and apparatus: the system is hierarchical in its representation of concepts and ideas, but reuses its components and actuators. Perhaps the study of overlapping coalition formation games might provide some insight into the problem.

## Why we need to understand coalitional agency?
This is a useful framework not just for understanding artificial intelligence, but also biological/social systems:

- Cooperative biofilms that are also fighting it out for resources and ecological niches (e.g. regions of the gut)
- Interactions between the tumour microenvironment and surrounding tissue.
- The credit assignment problem in companies 
- Race dynamics in neural networks (e.g. work from Andrew Saxe's group at the Gatsby unit at UCL)

These are just my preliminary thoughts. Looking forward to reading more about this topic.

[Paper]: https://www.mindthefuture.info/p/towards-a-scale-free-theory-of-intelligent