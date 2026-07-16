---
title: "Consolidative memory replay"
date: 2026-04-03
---

I had to write a research proposal recently, and I came across this really interesting problem in the exploration phase.

There are certain experimental systems which are very amenable to damage. A spatial example is the mouse hippocampus, which encodes place fields - essentially, sets of neurons that fire when recognising a certain part of space. The HVC in zebra finches is a nucleus that encodes temporally-associated memories.

In both cases, you can damage almost half of the excitatory neurons, and the mice/songbirds will still recover their behaviour within days or weeks. This is remarkable in itself, but there is also a temporal dynamic to it. In songbirds, we observe steady improvement during the say, spontaneous neuronal activity during the night, and backsliding during the day.

Now, there are a few possibilities: this may be explained by a Hebbian mechanism, or perhaps homeostasis. But perhaps a more interesting line of inquiry is the idea of consolidative memory replay: how do you know whether you've progressed in the right direction or not? Perhaps spontaneous neuronal activation during sleep acts as a *simulated annealing* process.

Simulated annealing derives from a metallurgical metaphor, where slow cooling represents slowly increasing the acceptance threshold as we explore solution space for the global optimum.

```text
s=s_0
for k in range (0, k_max):
	T ← temperature(1-(k+1)/k_max)
	s_new ← neighbour_random(s)
	if P(E(s), E(S_new), T) >= Rand(0,1):
		s ← s_new
return s
```

Perhaps memory replay acts as a way to anneal out of undesired learned behaviours after circuit perturbation. Given that these behaviours can be recovered without practice, this could be an intriguing way to explain why brain circuits are so resilient.
 