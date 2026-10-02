/* C supplies prepared paths and terminal states. All five drawings share a
 * clock that eases into a trickle, then continue their own autonomous flows. */
(function () {
  'use strict';
  var panel = document.currentScript.previousElementSibling;
  var figures = Array.from(panel.querySelectorAll('.attractor'));
  var motion = window.matchMedia('(prefers-reduced-motion: reduce)');
  var drawings = [], elapsed = 0, previous = null, frame = 0;
  var visible = false, loading = false, ready = false, density;
  var fastSeconds = 2, decaySeconds = 6, trickleRatio = .02;

  function playbackTime() {
    var slowed = Math.max(0, elapsed - fastSeconds);
    return Math.min(elapsed, fastSeconds) + trickleRatio * slowed
      + (1 - trickleRatio) * decaySeconds * -Math.expm1(-slowed / decaySeconds);
  }
  function derivative(id, x, y, z, out) {
    switch (id) {
      case 0:
        out[0] = -1.4 * x - 4 * y - 4 * z - y * y;
        out[1] = -1.4 * y - 4 * z - 4 * x - z * z;
        out[2] = -1.4 * z - 4 * x - 4 * y - x * x;
        break;
      case 1:
        out[0] = 10 * (y - x);
        out[1] = x * (28 - z) - y;
        out[2] = x * y - 8 / 3 * z;
        break;
      case 2:
        out[0] = (z - .7) * x - 3.5 * y;
        out[1] = 3.5 * x + (z - .7) * y;
        out[2] = .6 + .95 * z - z * z * z / 3
          - (x * x + y * y) * (1 + .25 * z) + .1 * z * x * x * x;
        break;
      case 3:
        out[0] = 36 * (y - x);
        out[1] = -x * z + 17 * y;
        out[2] = x * y - 3 * z;
        break;
      case 4:
        var f = -5 / 7 * x - 3 / 14 * (Math.abs(x + 1) - Math.abs(x - 1));
        out[0] = 15.6 * (y - x - f);
        out[1] = x - y + z;
        out[2] = -28 * y;
        break;
    }
  }
  function createDrawing(figure, model, count, samples, dt, h) {
    var canvas = figure.querySelector('canvas');
    var ink = document.createElement('canvas');
    ink.width = 480;
    ink.height = 420;
    var ctx = canvas.getContext('2d'), pen = ink.getContext('2d');
    if (!ctx || !pen) return null;
    var scale, ratio, brushScale, drawn = Array(count).fill(0), tails = [];

    function point(t, i) {
      var offset = (t * samples + i) * 2;
      return [model.points[offset], model.points[offset + 1]];
    }
    function nextState(state) {
      var x = state[0], y = state[1], z = state[2];
      var a = [], b = [], c = [], d = [];
      for (var k = 0; k < Math.round(dt / h); k++) {
        derivative(model.id, x, y, z, a);
        derivative(model.id, x + h / 2 * a[0], y + h / 2 * a[1], z + h / 2 * a[2], b);
        derivative(model.id, x + h / 2 * b[0], y + h / 2 * b[1], z + h / 2 * b[2], c);
        derivative(model.id, x + h * c[0], y + h * c[1], z + h * c[2], d);
        x += h / 6 * (a[0] + 2 * b[0] + 2 * c[0] + d[0]);
        y += h / 6 * (a[1] + 2 * b[1] + 2 * c[1] + d[1]);
        z += h / 6 * (a[2] + 2 * b[2] + 2 * c[2] + d[2]);
      }
      return [x, y, z];
    }
    function project(state) {
      var m = model.matrix, x = state[0], y = state[1], z = state[2];
      if (model.id === 4) {
        y = .36 * (state[0] + state[2] + 2.6 * state[1]);
        x = state[0] - .45 * Math.tanh(state[0] / .6);
        z = 0;
      }
      return [120 + (m[0] * x + m[1] * y + m[2] * z - model.cx) * model.scale,
              105 - (m[3] * x + m[4] * y + m[5] * z - model.cy) * model.scale];
    }
    function stroke(context, a, b, fraction) {
      var speed = Math.hypot(b[0] - a[0], b[1] - a[1]) / dt;
      context.lineWidth = (.65 - .25 * speed / (speed + 90)) / brushScale;
      context.beginPath();
      context.moveTo(a[0], a[1]);
      context.lineTo(a[0] + (b[0] - a[0]) * fraction, a[1] + (b[1] - a[1]) * fraction);
      context.stroke();
    }
    function setup(context) {
      var transform = context === pen ? 2 : ratio * scale;
      context.setTransform(transform, 0, 0, transform, 0, 0);
      context.strokeStyle = getComputedStyle(figure).color;
      context.globalAlpha = .32;
      context.lineCap = 'round';
      context.lineJoin = 'round';
    }
    function paint() {
      if (!scale) return;
      var progress = playbackTime() * model.rate / dt;
      var end = Math.floor(progress), fraction = progress - end;
      for (var t = 0; t < count; t++) {
        for (var i = drawn[t] + 1; i <= end; i++) {
          if (i < samples) {
            stroke(pen, point(t, i - 1), point(t, i), 1);
          } else {
            var tail = tails[t];
            stroke(pen, tail.point, tail.next, 1);
            tail.point = tail.next;
            tail.state = nextState(tail.state);
            tail.next = project(tail.state);
          }
        }
        drawn[t] = end;
      }
      ctx.setTransform(1, 0, 0, 1, 0, 0);
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      ctx.globalAlpha = 1;
      ctx.drawImage(ink, 0, 0, canvas.width, canvas.height);
      setup(ctx);
      if (fraction > 0) for (var t = 0; t < count; t++) {
        if (end < samples - 1) stroke(ctx, point(t, end), point(t, end + 1), fraction);
        else stroke(ctx, tails[t].point, tails[t].next, fraction);
      }
    }
    function resize() {
      var width = figure.querySelector('.attractor-stage').getBoundingClientRect().width;
      if (!width) return;
      ratio = Math.min(window.devicePixelRatio || 1, 2);
      scale = width / 240;
      if (!brushScale) brushScale = scale;
      var pixelWidth = Math.round(width * ratio), pixelHeight = Math.round(width * 210 / 240 * ratio);
      if (canvas.width !== pixelWidth || canvas.height !== pixelHeight) {
        canvas.width = pixelWidth;
        canvas.height = pixelHeight;
      }
      setup(pen);
      paint();
    }
    for (var t = 0; t < count; t++) {
      var next = nextState(model.states[t]);
      tails.push({point: point(t, samples - 1), state: next, next: project(next)});
    }
    return {paint: paint, resize: resize};
  }
  function resize() {
    density = Math.min(window.devicePixelRatio || 1, 2);
    drawings.forEach(function (drawing) { if (drawing) drawing.resize(); });
  }
  function tick(now) {
    frame = 0;
    if (!ready || !visible || document.hidden || motion.matches) return;
    if (density !== Math.min(window.devicePixelRatio || 1, 2)) resize();
    if (previous !== null) elapsed += Math.min((now - previous) / 1000, .06);
    previous = now;
    drawings.forEach(function (drawing) { if (drawing) drawing.paint(); });
    frame = requestAnimationFrame(tick);
  }
  async function load() {
    if (ready || loading) return;
    loading = true;
    try {
      var response = await fetch('/assets/graphics/attractors.bin');
      if (!response.ok) throw new Error('Attractor data unavailable');
      var buffer = await response.arrayBuffer(), view = new DataView(buffer);
      if (view.getUint32(0, false) !== 0x41545031) throw new Error('Invalid attractor data');
      var systems = view.getUint32(4, true), count = view.getUint32(8, true), samples = view.getUint32(12, true);
      var dt = view.getFloat32(16, true), h = view.getFloat32(20, true);
      var block = 80 + count * samples * 8 + count * 24;
      if (systems !== figures.length || buffer.byteLength !== 24 + systems * block) throw new Error('Incomplete attractor data');
      var offset = 24;
      for (var s = 0; s < systems; s++) {
        var model = {id: view.getUint32(offset, true), rate: view.getFloat32(offset + 4, true), matrix: [], states: []};
        if (model.id !== s) throw new Error('Invalid model order');
        for (var j = 0; j < 6; j++) model.matrix.push(view.getFloat64(offset + 8 + j * 8, true));
        model.scale = view.getFloat64(offset + 56, true);
        model.cx = view.getFloat64(offset + 64, true);
        model.cy = view.getFloat64(offset + 72, true);
        offset += 80;
        model.points = new Float32Array(buffer, offset, count * samples * 2);
        offset += count * samples * 8;
        for (var t = 0; t < count; t++) {
          var state = [];
          for (var j = 0; j < 3; j++) state.push(view.getFloat64(offset + t * 24 + j * 8, true));
          model.states.push(state);
        }
        offset += count * 24;
        drawings.push(createDrawing(figures[s], model, count, samples, dt, h));
      }
      ready = true;
      resize();
      update();
    } catch (error) {
      drawings = [];
      ready = false;
      update();
    }
  }
  function update() {
    cancelAnimationFrame(frame);
    frame = 0;
    previous = null;
    figures.forEach(function (figure, i) {
      var still = motion.matches || !ready || !drawings[i];
      figure.querySelector('img').hidden = !still;
      figure.querySelector('canvas').hidden = still;
    });
    if (motion.matches) return;
    if (visible && !ready) load();
    if (ready && visible && !document.hidden) frame = requestAnimationFrame(tick);
  }
  motion.addEventListener('change', update);
  document.addEventListener('visibilitychange', update);
  window.addEventListener('resize', resize);
  if ('ResizeObserver' in window) {
    var sizes = new ResizeObserver(resize);
    figures.forEach(function (figure) { sizes.observe(figure.querySelector('.attractor-stage')); });
  }
  if ('IntersectionObserver' in window) {
    new IntersectionObserver(function (entries) { visible = entries[0].isIntersecting; update(); }).observe(panel);
  } else visible = true;
  update();
})();
