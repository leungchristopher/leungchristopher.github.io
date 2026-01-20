---
permalink: /accountability/
title: "Accountability Log"
layout: single
author_profile: true
---

## Summary

{% assign total_hours = 0 %}
{% assign projects = "" | split: "" %}

{% for entry in site.data.accountability.entries %}
  {% assign total_hours = total_hours | plus: entry.hours %}
  {% unless projects contains entry.project %}
    {% assign projects = projects | push: entry.project %}
  {% endunless %}
{% endfor %}

**Total Hours Worked**: {{ total_hours | round: 1 }} hours

### Hours by Project

{% for project in projects %}
  {% assign project_hours = 0 %}
  {% for entry in site.data.accountability.entries %}
    {% if entry.project == project %}
      {% assign project_hours = project_hours | plus: entry.hours %}
    {% endif %}
  {% endfor %}
  - **{{ project }}**: {{ project_hours | round: 1 }} hours
{% endfor %}

---

## Work Log

{% assign sorted_entries = site.data.accountability.entries | reverse %}

{% for entry in sorted_entries %}
### {{ entry.date }}

**Project**: {{ entry.project }}
**Hours**: {{ entry.hours }}
**Notes**: {{ entry.description }}

---

{% endfor %}
