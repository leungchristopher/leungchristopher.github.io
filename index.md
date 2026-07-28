---
layout: home
title: Home
permalink: /
---

I study cooperative and emergent behaviour in intelligent systems, at the intersection of neuroscience, genetics, and machine learning.
I am currently at the [California Institute of Technology (Caltech)](https://www.caltech.edu/), studying consolidative memory replay in neural circuits during sleep.

Alongside my academic work, I am developing algorithms for openended scientific discovery: more on this soon! Fundamentally, I'm excited about the unknown unknowns: as the production of biomedical data outpaces our ability to parse and understand it, how can we bring its fruits to bear?

### Contact me

Book a meeting [here](https://calendly.com/chcl4-cam/30min) or reach out by [email](mailto:chcl4@cam.ac.uk).

## Publications
**Christopher Leung**, Charlotte Houldcroft, Aylwyn Scally. *Statistical inference of viral ancestral recombination graphs* (2026). *In preparation.*

## Current projects

{% for p in site.data.projects %}{% if p.current %}{% include project-card.html project=p %}{% endif %}{% endfor %}
