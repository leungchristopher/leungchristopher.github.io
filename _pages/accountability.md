---
permalink: /accountability/
title: "Accountability Log"
layout: single
author_profile: true
---

<style>
table {
  border-collapse: collapse;
}

table, th, td {
  border: 1px solid rgba(0, 0, 0, 0.1);
}

th {
  text-align: left;
  font-weight: 600;
}

td {
  padding: 8px 12px;
}
</style>

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

<table>
  <thead>
    <tr>
      <th>Date</th>
      <th>Project</th>
      <th>Hours</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
{% assign current_date = nil %}
{% assign date_row_count = 0 %}

{% for entry in site.data.accountability.entries %}
  {% if entry.date != current_date %}
    {% comment %} Count entries for this new date {% endcomment %}
    {% assign date_row_count = 0 %}
    {% for e in site.data.accountability.entries %}
      {% if e.date == entry.date %}
        {% assign date_row_count = date_row_count | plus: 1 %}
      {% endif %}
    {% endfor %}
    {% assign current_date = entry.date %}
    {% assign date_shown = false %}
  {% endif %}

  <tr>
    {% unless date_shown %}
      <td rowspan="{{ date_row_count }}"><strong>{{ current_date }}</strong></td>
      {% assign date_shown = true %}
    {% endunless %}
    <td>{{ entry.project }}</td>
    <td>{{ entry.hours }}</td>
    <td>{{ entry.description }}</td>
  </tr>
{% endfor %}
  </tbody>
</table>
