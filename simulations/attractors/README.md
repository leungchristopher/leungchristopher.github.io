# Five attractor brushstrokes

`{{attractors}}` places a five-column panel above the homepage text:
Halvorsen, Lorenz, Aizawa, Chen–Celikovsky, and the original Chua double-scroll. On narrow screens,
the row scrolls horizontally instead of shrinking the drawings to tiny marks.
Each has four independent trajectories and thin gray strokes. Model names
remain in the accessible descriptions; there are no visible labels.

All start together. They begin quickly for two seconds, then share an
exponential slowdown with a six-second decay time, approaching 2% of their
initial speeds. The simulated flow still determines variations in drawing
speed. Strokes remain and trajectories continue without fading or restarting.

## Models

The equations use dimensionless state `(x,y,z)`:

| Model | dx/dt | dy/dt | dz/dt |
| --- | --- | --- | --- |
| Halvorsen | `-1.4x-4y-4z-y²` | `-1.4y-4z-4x-z²` | `-1.4z-4x-4y-x²` |
| Lorenz | `10(y-x)` | `x(28-z)-y` | `xy-(8/3)z` |
| Aizawa | `(z-.7)x-3.5y` | `3.5x+(z-.7)y` | `.6+.95z-z³/3-(x²+y²)(1+.25z)+.1zx³` |
| Chen–Celikovsky | `36(y-x)` | `-xz+17y` | `xy-3z` |
| Chua double-scroll | `15.6(y-x-f(x))` | `x-y+z` | `-28y` |

For Chua, `f(x)=-(5/7)x-(3/14)(abs(x+1)-abs(x-1))`.

Model definitions and parameter references:

- [Halvorsen: TSDynamics implementation documentation](https://el3ssar.github.io/TSDynamics/systems/ode/chaotic-attractors/Halvorsen/).
- [Lorenz and Aizawa: research supplement, equations S57 and S60](https://journals.aps.org/prxlife/supplemental/10.1103/jc9p-m3rn/si_carto.pdf).
- [Chen–Celikovsky: attractors 1.1.1 released source](https://pypi.org/project/attractors/1.1.1/#files), `attractors/utils/base.py` and `attractors/data/params.json`.
- [Chua parameters and equations: Deciphering Dynamical Nonlinearities in Short Time Series Using Recurrent Neural Networks](https://doi.org/10.1038/s41598-019-50625-y).

This panel uses the Chen–Celikovsky equations and parameters in the released
`attractors` implementation. The Chua view retains the original smooth
horizontal compression and rotation that makes its lobes resemble an
infinity sign.

## Generation and playback

The C generator uses fourth order Runge–Kutta with step `0.001`, discarding
30 simulation time units for each perturbed seed. It prepares 24 time units
at sample interval `0.01`, plus exact terminal states. The browser continues
from these states using the same equations and integration step. Projection
matrices are stored in the generated data so both implementations share
identical views. Each view fits its paths with room for continued motion.

Per-model initial playback rates in simulation units per second are 1.8,
2.2, 5.0, 1.8, and 3.75, respectively. This makes the slower Aizawa dynamics
visually comparable to the faster Lorenz-family flows. The slowdown clock
pauses while the panel is off screen, the tab is hidden, or reduced motion
is enabled. Reduced motion and disabled JavaScript display static SVGs.

Each drawing deposits completed segments once onto a fixed 480×420 canvas.
Resizing only scales this unchanged ink onto its display canvas; repeated
resizing preserves both the drawing and playback progress. Display resolution
tracks pixel density. Continued simulation retains only current states and
a fractional segment, so memory use remains bounded.

## Regenerate

From the repository root:

```sh
cc -std=c11 -O2 -Wall -Wextra -Werror simulations/attractors/attractors.c -lm -o /tmp/site-attractors
/tmp/site-attractors --check
/tmp/site-attractors --generate assets/graphics
cargo run --release --manifest-path ssg/Cargo.toml
```

The normal site build uses the checked-in binary and SVGs. `--check` verifies
short-time step refinement, nondegenerate projections, bounded motion, and
viewport coverage for an additional 200 simulation units per trajectory.

`assets/js/attractors.js` controls the shared slowdown through `fastSeconds`,
`decaySeconds`, and `trickleRatio`. The C `models` table controls the view,
seed, and initial playback rate of each attractor.

## Binary layout

All values are little-endian. `ATP1` is followed by uint32 system count,
trajectory count, and sample count, then float32 sample and integration steps.
Each system has uint32 model ID, float32 playback rate, six float64 projection
coefficients, three float64 fit values (scale, center x, center y), flattened
float32 projected `(x,y)` samples, and float64 terminal `(x,y,z)` states.
