#!/usr/bin/env python3
"""Generate the small, self-contained SVG animations used in the RKHS essay."""

from math import pi, sin, cos
from pathlib import Path
import random

OUT = Path(__file__).resolve().parents[2] / "assets/graphics"
INK = "#59636e"
FAINT = "#d8dce0"
ACCENT = "#687f8c"


def svg(content, title, desc, viewbox="0 0 600 270"):
    return f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="{viewbox}" role="img" aria-labelledby="title desc">
<title id="title">{title}</title><desc id="desc">{desc}</desc>
<rect width="100%" height="100%" fill="#fff"/>
{content}
</svg>\n'''


def path(points):
    return "M" + " L".join(f"{x:.1f},{y:.1f}" for x, y in points)


def xor_feature_map():
    # For x1,x2 in {-1,+1}, phi=(x1,x2,x1*x2); w=(0,0,1)
    # separates XOR points (x1*x2 < 0) from the other class. The point cloud
    # uses continuous inputs in [-1,1]^2 to make the mapping visible.
    bits = []
    bits.append(f'<text x="65" y="36" font-family="serif" font-size="14" fill="{INK}">input</text>')
    bits.append(f'<text x="395" y="36" font-family="serif" font-size="14" fill="{INK}">φ(x) in feature space</text>')
    # Input square, with the two axes marking the XOR quadrant boundaries.
    bits.append(f'<path d="M105 78 V202 M43 140 H167" fill="none" stroke="{FAINT}" stroke-width="1"/>')
    bits.append(f'<text x="169" y="144" font-family="serif" font-size="12" fill="{INK}">x₁</text><text x="108" y="72" font-family="serif" font-size="12" fill="{INK}">x₂</text>')
    # Isometric feature-space axes and the z=0 decision plane.
    def project(x1, x2, z):
        return 420 + 48*x1 + 22*x2, 142 + 17*x1 - 17*x2 - 54*z
    plane = [project(-1,-1,0), project(1,-1,0), project(1,1,0), project(-1,1,0)]
    bits.append(f'<path d="{path(plane)} Z" fill="#f5f6f7" stroke="{FAINT}" stroke-width="1"/>')
    bits.append(f'<path d="M420 142 L468 159 M420 142 L442 125 M420 142 V72" fill="none" stroke="{INK}" stroke-width="1.1"/>')
    bits.append(f'<text x="472" y="165" font-family="serif" font-size="12" fill="{INK}">x₁</text><text x="443" y="121" font-family="serif" font-size="12" fill="{INK}">x₂</text><text x="424" y="69" font-family="serif" font-size="12" fill="{INK}">z</text>')
    bits.append(f'<text x="492" y="122" font-family="serif" font-size="12" fill="{INK}">z = 0</text>')
    bits.append(f'<text x="353" y="240" font-family="serif" font-size="12" fill="{INK}">w = (0, 0, 1)</text>')
    rng = random.Random(29)
    cloud = []
    for quadrant in ((-1,-1),(-1,1),(1,-1),(1,1)):
        for _ in range(18):
            x1 = quadrant[0] * rng.uniform(.12, .98)
            x2 = quadrant[1] * rng.uniform(.12, .98)
            cloud.append((x1, x2))
    for index, (x1, x2) in enumerate(cloud):
        sx, sy = 105 + 48*x1, 140 - 48*x2
        tx, ty = project(x1, x2, x1*x2)
        color = "#4778a8" if x1*x2 < 0 else "#c05a58"
        delay = index * .025
        bits.append(f'<circle cx="{sx:.2f}" cy="{sy:.2f}" r="2.5" fill="{color}" opacity=".88"><animate attributeName="cx" values="{sx:.2f};{tx:.2f};{tx:.2f};{sx:.2f}" keyTimes="0;.32;.68;1" dur="10s" begin="{delay:.3f}s" repeatCount="indefinite"/><animate attributeName="cy" values="{sy:.2f};{ty:.2f};{ty:.2f};{sy:.2f}" keyTimes="0;.32;.68;1" dur="10s" begin="{delay:.3f}s" repeatCount="indefinite"/></circle>')
    return svg("\n".join(bits), "An XOR point cloud mapped into feature space", "A cloud sampled across four XOR quadrants moves under phi of x equals (x1, x2, x1 x2). Blue XOR points and red points from the other class land on opposite sides of the z equals zero plane, the decision boundary for weight vector zero, zero, one.")


def fourier(n, x):
    # Centered unit-height top hat on [-pi/2, pi/2], period 2pi.
    return 0.5 + sum(2 * sin(k*pi/2) * cos(k*x) / (k*pi) for k in range(1, n+1))


def graph_path(fn, x0=-pi, x1=pi, y0=-0.28, y1=1.28, samples=400, box=(55, 30, 490, 190)):
    left, top, width, height = box
    pts=[]
    for i in range(samples+1):
        x=x0+(x1-x0)*i/samples
        y=fn(x)
        pts.append((left+width*i/samples, top+height*(y1-y)/(y1-y0)))
    return path(pts)


def top_hat():
    bits=[f'<path d="M55 220 H545 M55 30 V220" fill="none" stroke="{FAINT}"/>']
    # Thin reference outline, then a sum that grows smoothly in harmonic content.
    bits.append(f'<path d="M55 186 H177 V64 H423 V186 H545" fill="none" stroke="{FAINT}" stroke-width="1" stroke-dasharray="3 4"/>')
    d0=graph_path(lambda x:fourier(1,x))
    bits.append(f'<path d="{d0}" fill="none" stroke="{ACCENT}" stroke-width="2.4" stroke-linecap="round" stroke-linejoin="round">')
    vals=";".join(graph_path(lambda x,n=n:fourier(n,x)) for n in (1,3,7,15,31))
    bits.append(f'<animate attributeName="d" values="{vals}" keyTimes="0;.25;.5;.75;1" dur="8s" repeatCount="indefinite"/></path>')
    bits.append(f'<text x="54" y="245" font-family="serif" font-size="13" fill="{INK}">−π</text><text x="529" y="245" font-family="serif" font-size="13" fill="{INK}">π</text>')
    return svg("\n".join(bits), "Fourier terms building a top-hat function", "A Fourier series accumulates harmonics to approach a rectangular pulse.")


if __name__ == "__main__":
    OUT.mkdir(parents=True, exist_ok=True)
    for name, content in (("rkhs-xor.svg", xor_feature_map()), ("rkhs-tophat.svg", top_hat())):
        (OUT / name).write_text(content, encoding="utf-8")
