---
title: "A Rust static site generator"
date: 2026-12-29
---

I got nerd-sniped by [Julian Schrittwieser's blog post](https://www.julian.ac/blog/2024/09/12/custom-static-site-generator/) on writing a static site generator in Rust - here it is!
Most of the wins came from optimising images from JPG to WEBP then finally to AVIF, lazy-loading, and opt-in MathJax.
Writing the site generator was fun apart from handling the edge cases in Markdown. Instead of using Jinja for template parsing, I directly wrote some Rust functions to return formatted HTML strings - giving compiler errors instead of runtime template-syntax mistakes from Jinja.

Overall, I'm pretty happy: the frontpage now loads in 78KB (with the pure HTML/CSS coming around 20KB!
