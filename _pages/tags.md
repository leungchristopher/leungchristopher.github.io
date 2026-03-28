---
permalink: /tags/
title: Tags
---

{% assign all_tags = site.posts | map: "tags" | flatten | uniq | sort %}
{% for tag in all_tags %}
<p class="tag-group-heading" id="{{ tag | slugify }}">{{ tag }}</p>
<ul class="post-list">
  {% for post in site.posts %}
    {% if post.tags contains tag %}
    <li class="post-list-item">
      <a href="{{ post.url }}" class="post-list-title">{{ post.title }}</a>
      <time class="post-list-date" datetime="{{ post.date | date_to_xmlschema }}">{{ post.date | date: "%b %-d, %Y" }}</time>
    </li>
    {% endif %}
  {% endfor %}
</ul>
{% endfor %}
