---
layout: default
title: Log
permalink: /dlog/
body_class: dlog-page
---
{% include dlog-stats.html %}

<h1>Log</h1>

<p class="lede">
Day {{ dlog_num_days }}, {{ dlog_num_documented_days }} logged ({{ dlog_percentage_documented }}%), {{ dlog_streak }} day streak, {{ dlog_total_hrs }} hr {{ dlog_total_mins }} min total.
</p>
Vannevar Bush (Manhattan Project, NSF, differential analyser) was a giant proponent of basic science. In *Science, the endless frontier*, he argues that basic research is the engine of technological progress.

I try to wear two hats: scientist and computer scientist. By working across the two fields in open-ended intelligence and discovery, I hope to contribute what I can.

This log is here to keep me focused on building what I *want* to build, inspired by my friend, [Kyle Ng](https://doingtheth.ing/)!

My goals for the summer:
1. Become fluent in Rust and C
2. Familiarise myself with the cognitive and computational neuroscience frontier.
3. Be up-to-date with reasoning, world models, and robotics.

{% include dlog-tag-cloud.html %}

## Entries

<ul class="entry-list">
  {% assign entries = site.dlog | sort: "date" | reverse %}
  {% for entry in entries %}
  {% assign entry_tags = entry.sessions | map: "tag" | uniq | join: " " %}
  <li class="entry-item" data-tags="{{ entry_tags }}">
    <a class="entry-item-title" href="{{ entry.url | relative_url }}">{{ entry.title | default: entry.goal }}</a>
    <time class="entry-item-date" datetime="{{ entry.date | date_to_xmlschema }}">{{ entry.date | date: "%-d %B %Y" }}</time>
  </li>
  {% endfor %}
</ul>
