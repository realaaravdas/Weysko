(function () {
  // Mobile navbar toggles
  document.querySelectorAll('[data-ws-nav-toggle]').forEach(function (btn) {
    btn.addEventListener('click', function () {
      var nav = btn.closest('.ws-nav');
      if (nav) nav.classList.toggle('is-open');
    });
  });

  // Scroll-reveal animations
  var animEls = document.querySelectorAll('.ws-anim');
  if ('IntersectionObserver' in window && animEls.length > 0) {
    var obs = new IntersectionObserver(function (entries) {
      entries.forEach(function (entry) {
        if (entry.isIntersecting) {
          entry.target.classList.add('ws-anim-visible');
          obs.unobserve(entry.target);
        }
      });
    }, { threshold: 0.12 });
    animEls.forEach(function (el) { obs.observe(el); });
  } else {
    animEls.forEach(function (el) { el.classList.add('ws-anim-visible'); });
  }

  // Toast helper
  function showToast(msg) {
    var toast = document.getElementById('ws-toast');
    if (!toast) return;
    toast.textContent = msg;
    toast.classList.add('is-shown');
    setTimeout(function () { toast.classList.remove('is-shown'); }, 3200);
  }

  // Form submission handlers
  document.querySelectorAll('.ws-form[data-ws-form-mode]').forEach(function (form) {
    form.addEventListener('submit', function (e) {
      var mode = form.getAttribute('data-ws-form-mode');
      if (mode === 'demo') {
        e.preventDefault();
        form.reset();
        showToast('Message received. Thanks for reaching out!');
      } else if (mode === 'mailto') {
        e.preventDefault();
        var to = form.getAttribute('data-ws-mailto') || '';
        var fields = [];
        form.querySelectorAll('input, textarea').forEach(function (inp) {
          if (inp.name) fields.push(inp.name + ': ' + inp.value);
        });
        var body = encodeURIComponent(fields.join('\n'));
        window.location.href = 'mailto:' + encodeURIComponent(to) + '?subject=Website%20Inquiry&body=' + body;
        showToast('Opening your email client...');
      }
    });
  });
})();
