---
permalink: /accountability/
title: Accountability Log
---

A record of how I spend my productive time.

## Summary

{% assign total_hours = 0 %}
{% assign projects = "" | split: "" %}
{% for entry in site.data.accountability.entries %}
  {% assign total_hours = total_hours | plus: entry.hours %}
  {% unless projects contains entry.project %}
    {% assign projects = projects | push: entry.project %}
  {% endunless %}
{% endfor %}

**Total hours logged:** {{ total_hours | round: 1 }}

<ul class="log-summary">
{% for project in projects %}
  {% assign project_hours = 0 %}
  {% for entry in site.data.accountability.entries %}
    {% if entry.project == project %}
      {% assign project_hours = project_hours | plus: entry.hours %}
    {% endif %}
  {% endfor %}
  <li><span>{{ project }}</span><span>{{ project_hours | round: 1 }}h</span></li>
{% endfor %}
</ul>

---

## Log

<div class="log-entries">
{% for entry in site.data.accountability.entries %}
<div class="log-entry">
  <span class="log-date">{{ entry.date }}</span>
  <span class="log-proj">{{ entry.project }}</span>
  <span class="log-hours">{{ entry.hours }}h</span>
  <span class="log-desc">{{ entry.description }}</span>
</div>
{% endfor %}
</div>
