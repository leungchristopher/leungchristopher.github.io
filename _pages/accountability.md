---
permalink: /accountability/
title: "Accountability Log"
layout: single
author_profile: true
---

<script src="https://cdn.plot.ly/plotly-latest.min.js"></script>

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

#workChart {
  width: 100%;
  max-width: 100%;
  height: 500px;
  margin: 20px 0;
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

## Work Distribution Chart

<div id="workChart"></div>

<script>
document.addEventListener('DOMContentLoaded', function() {
  // Extract all entries data
  const entries = [
    {% for entry in site.data.accountability.entries %}
    {
      date: "{{ entry.date }}",
      project: "{{ entry.project }}",
      hours: {{ entry.hours }},
      description: "{{ entry.description | replace: '"', '\"' }}"
    }{% unless forloop.last %},{% endunless %}
    {% endfor %}
  ];

  // Group entries by date and project
  const dateMap = {};
  const projectSet = new Set();

  entries.forEach(entry => {
    if (!dateMap[entry.date]) {
      dateMap[entry.date] = {};
    }
    if (!dateMap[entry.date][entry.project]) {
      dateMap[entry.date][entry.project] = [];
    }
    dateMap[entry.date][entry.project].push(entry);
    projectSet.add(entry.project);
  });

  // Sort dates
  const sortedDates = Object.keys(dateMap).sort().reverse();
  const projectsArray = Array.from(projectSet).sort();

  // Create trace for each project
  const traces = projectsArray.map(project => {
    const xData = sortedDates;
    const yData = sortedDates.map(date => {
      if (dateMap[date][project]) {
        return dateMap[date][project].reduce((sum, e) => sum + e.hours, 0);
      }
      return 0;
    });

    // Create custom data for hover text
    const customData = sortedDates.map(date => {
      if (dateMap[date][project]) {
        const descriptions = dateMap[date][project]
          .map(e => e.description)
          .join('; ');
        const totalHours = dateMap[date][project]
          .reduce((sum, e) => sum + e.hours, 0);
        return `${project}<br>${totalHours} hours<br>${descriptions}`;
      }
      return '';
    });

    // Assign colors to projects for consistency
    const colors = {
      'Part II Project': '#1f77b4',
      'Literature Review': '#ff7f0e',
      'M2': '#2ca02c',
      'Lectures': '#d62728',
      'Preparations': '#9467bd',
      'ARBOx3': '#8c564b'
    };

    return {
      x: xData,
      y: yData,
      name: project,
      type: 'bar',
      marker: { color: colors[project] || '#7f7f7f' },
      customdata: customData,
      hovertemplate: '<b>%{customdata}</b><extra></extra>'
    };
  });

  const layout = {
    title: 'Daily Work Distribution by Project',
    xaxis: {
      title: 'Date',
      automargin: true
    },
    yaxis: {
      title: 'Hours Worked',
      automargin: true
    },
    barmode: 'stack',
    hovermode: 'closest',
    margin: {
      l: 60,
      r: 20,
      t: 60,
      b: 80
    },
    responsive: true
  };

  Plotly.newPlot('workChart', traces, layout, { responsive: true });
});
</script>

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
