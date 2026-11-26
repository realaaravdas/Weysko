/// Generates zero-dependency vanilla JS for mobile navigation, scroll-reveal, forms, and optional SPA page switching.
pub fn generate_js(single_file_mode: bool) -> String {
    let mut out = String::from(
        r#"(function () {
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
"#,
    );

    if single_file_mode {
        out.push_str(
            r#"
  // Single-file / Google Sites multi-page router
  function activatePage(slug) {
    var views = document.querySelectorAll('[data-ws-page-view]');
    var found = false;
    views.forEach(function (view) {
      if (view.getAttribute('data-ws-page-view') === slug) {
        view.classList.add('is-active');
        found = true;
      } else {
        view.classList.remove('is-active');
      }
    });
    if (!found && views.length > 0) {
      views[0].classList.add('is-active');
    }
    document.querySelectorAll('.ws-nav.is-open').forEach(function (nav) {
      nav.classList.remove('is-open');
    });
    window.scrollTo({ top: 0, behavior: 'smooth' });
  }

  document.querySelectorAll('[data-ws-page]').forEach(function (link) {
    link.addEventListener('click', function (e) {
      var slug = link.getAttribute('data-ws-page');
      if (slug) {
        e.preventDefault();
        activatePage(slug);
        if (window.history && window.history.replaceState) {
          window.history.replaceState(null, '', '#page-' + slug);
        }
      }
    });
  });

  if (window.location.hash && window.location.hash.indexOf('#page-') === 0) {
    activatePage(window.location.hash.slice(6));
  }
"#,
        );
    }

    out.push_str("})();\n");
    out
}
