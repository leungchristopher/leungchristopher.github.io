---
permalink: /blog/
title: Writing
---

{% assign posts_by_year = site.posts | group_by_exp: "post", "post.date | date: '%Y'" %}
{% for year_group in posts_by_year %}
<p class="archive-year">{{ year_group.name }}</p>
<ul class="post-list">
  {% for post in year_group.items %}
  <li class="post-list-item">
    <a href="{{ post.url }}" class="post-list-title">{{ post.title }}</a>
    <time class="post-list-date" datetime="{{ post.date | date_to_xmlschema }}">{{ post.date | date: "%d/%m" }}</time>
  </li>
  {% endfor %}
</ul>
{% endfor %}
