---
permalink: /blog/
title: Writing
---

<input class="tag-search" id="tagFilter" type="search" placeholder="Filter by title/tag (research, books, curiosities...)" autocomplete="off" spellcheck="false">

{% assign posts_by_year = site.posts | group_by_exp: "post", "post.date | date: '%Y'" %}
{% for year_group in posts_by_year %}
<div class="archive-group">
  <p class="archive-year">{{ year_group.name }}</p>
  <ul class="post-list">
    {% for post in year_group.items %}
    <li class="post-list-item" data-tags="{{ post.tags | join: ' ' | downcase }}" data-title="{{ post.title | downcase }}">
      <a href="{{ post.url }}" class="post-list-title">{{ post.title }}</a>
      <time class="post-list-date" datetime="{{ post.date | date_to_xmlschema }}">{{ post.date | date: "%d/%m" }}</time>
    </li>
    {% endfor %}
  </ul>
</div>
{% endfor %}

<script>
(function() {
  var input = document.getElementById('tagFilter');
  if (!input) return;
  input.addEventListener('input', function() {
    var q = this.value.trim().toLowerCase();
    document.querySelectorAll('.archive-group').forEach(function(group) {
      var anyVisible = false;
      group.querySelectorAll('.post-list-item').forEach(function(item) {
        var match = !q || item.getAttribute('data-tags').indexOf(q) !== -1 || item.getAttribute('data-title').indexOf(q) !== -1;
        item.style.display = match ? '' : 'none';
        if (match) anyVisible = true;
      });
      group.style.display = anyVisible ? '' : 'none';
    });
  });
})();
</script>
