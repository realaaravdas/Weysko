(function () {
  // Mobile navigation drawer
  var header = document.getElementById('site-header');
  var menuBtn = document.getElementById('mobile-menu-btn');
  if (header && menuBtn) {
    menuBtn.addEventListener('click', function () {
      var open = header.classList.toggle('is-open');
      menuBtn.setAttribute('aria-expanded', open ? 'true' : 'false');
    });
    header.querySelectorAll('.main-nav a').forEach(function (link) {
      link.addEventListener('click', function () {
        header.classList.remove('is-open');
        menuBtn.setAttribute('aria-expanded', 'false');
      });
    });
  }

  // Subtle ambient vector filaments in hero lower-right (with pause/play toggle)
  var canvas = document.getElementById('hero-canvas');
  var pauseBtn = document.getElementById('hero-pause-btn');
  var animRunning = true;
  var phase = 0;

  if (canvas && canvas.getContext) {
    var ctx = canvas.getContext('2d');

    function resizeCanvas() {
      var dpr = Math.min(window.devicePixelRatio || 1, 2);
      var rect = canvas.getBoundingClientRect();
      canvas.width = Math.max(1, Math.floor(rect.width * dpr));
      canvas.height = Math.max(1, Math.floor(rect.height * dpr));
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    }

    resizeCanvas();
    window.addEventListener('resize', resizeCanvas);

    function drawFrame() {
      var rect = canvas.getBoundingClientRect();
      var w = rect.width;
      var h = rect.height;
      ctx.clearRect(0, 0, w, h);

      // Restrained cobalt/cyan lower-right ambient radial wash
      var grad = ctx.createRadialGradient(w * 0.78, h * 0.85, 20, w * 0.72, h * 0.78, w * 0.52);
      grad.addColorStop(0, 'rgba(0, 188, 235, 0.16)');
      grad.addColorStop(0.45, 'rgba(0, 96, 255, 0.10)');
      grad.addColorStop(1, 'rgba(0, 0, 0, 0)');
      ctx.fillStyle = grad;
      ctx.fillRect(0, 0, w, h);

      // Fine architectural wave ribbons anchored on lower-right so left text stays crisp
      var strands = 22;
      for (var i = 0; i < strands; i++) {
        var frac = i / (strands - 1);
        ctx.beginPath();
        var startX = w * 0.34;
        for (var x = startX; x <= w + 20; x += 14) {
          var nx = (x - startX) / (w - startX);
          var wave =
            Math.sin(nx * 4.2 + phase + frac * 1.9) * (36 * nx) +
            Math.cos(nx * 2.4 - phase * 0.7 + frac * 1.4) * (24 * nx);
          var y = h * (0.94 - nx * 0.42) + (frac - 0.5) * 90 + wave;
          if (x === startX) ctx.moveTo(x, y);
          else ctx.lineTo(x, y);
        }
        var alpha = 0.04 + (1 - Math.abs(frac - 0.5) * 2) * 0.18;
        ctx.strokeStyle =
          i % 4 === 0
            ? 'rgba(0, 188, 235, ' + alpha.toFixed(3) + ')'
            : 'rgba(226, 232, 240, ' + (alpha * 0.85).toFixed(3) + ')';
        ctx.lineWidth = i % 5 === 0 ? 1.35 : 0.85;
        ctx.stroke();
      }

      if (animRunning) {
        phase += 0.008;
      }
      requestAnimationFrame(drawFrame);
    }

    requestAnimationFrame(drawFrame);
  }

  if (pauseBtn) {
    pauseBtn.addEventListener('click', function () {
      animRunning = !animRunning;
      pauseBtn.textContent = animRunning ? '||' : '>';
      pauseBtn.setAttribute('aria-pressed', animRunning ? 'false' : 'true');
      pauseBtn.setAttribute(
        'aria-label',
        animRunning ? 'Pause background motion' : 'Play background motion'
      );
    });
  }

  // Tabbed Studio Showcase
  var showcaseData = {
    canvas: {
      title: 'Freeform WYSIWYG Canvas with 8px Snap & Smart Guides',
      desc: 'Drag elements, resize via 8-point handles, edit typography inline, and inspect live X/Y coordinates.',
      ctaText: 'Download Installer v1.1.0',
      ctaHref: 'https://github.com/realaaravdas/Websir/releases/download/v1.1.0/websir-v1.1.0-windows-x86_64-setup.exe',
      mode: 'studio',
      badge: 'Canvas /index.html',
      headline: 'Critical infrastructure for modern web publishing.',
      sub: 'Pure Rust visual positioning paired with clean semantic CSS export.'
    },
    tokens: {
      title: 'Tokenized Slate Themes & Custom CSS Variables',
      desc: 'Switch between 6 curated palettes or fine-tune every CSS variable, font stack, border, and shadow.',
      ctaText: 'View Theme Tokens',
      ctaHref: '#architecture',
      mode: 'code',
      code:
        ':root {\n' +
        '  --ws-bg: #090D16;\n' +
        '  --ws-surface: #0F172A;\n' +
        '  --ws-primary: #FFFFFF;\n' +
        '  --ws-primary-text: #0F172A;\n' +
        '  --ws-border: #334155;\n' +
        '  --ws-font-heading: \'Inter\', sans-serif;\n' +
        '  --ws-max-width: 1200px;\n' +
        '}\n'
    },
    breakpoints: {
      title: 'Desktop, Tablet & Mobile Breakpoint Overrides',
      desc: 'Preview 1280px, 768px, and 390px viewports with optional Auto-Stack mobile ordering or manual overrides.',
      ctaText: 'Explore Responsive Rules',
      ctaHref: '#capabilities',
      mode: 'studio',
      badge: 'Mobile 390px • AutoStack',
      headline: 'Deterministic layout reflow across every viewport.',
      sub: 'Automatic (y, x) flex stacking on mobile or explicit breakpoint geometry.'
    },
    export: {
      title: 'Multi-Target Static Export & Google Sites Bundle',
      desc: 'Compile to multi-page static folders, compressed ZIP archives, or self-contained single-file HTML.',
      ctaText: 'See Deployment Targets',
      ctaHref: '#deployment',
      mode: 'code',
      code:
        'websir-dist/\n' +
        '├── index.html       (semantic HTML5)\n' +
        '├── pricing.html     (multi-page routing)\n' +
        '├── 404.html         (branded fallback)\n' +
        '├── styles.css       (CSS3 variables + media queries)\n' +
        '├── site.js          (zero-dependency vanilla JS)\n' +
        '├── sitemap.xml      (auto-generated)\n' +
        '└── assets/          (decoded local media)\n'
    },
    preview: {
      title: 'Embedded Pure-Rust HTTP Preview Server',
      desc: 'Launch an instant localhost preview with live memory hot-reload whenever you modify your site.',
      ctaText: 'View Architecture',
      ctaHref: '#architecture',
      mode: 'code',
      code:
        '$ websir --serve template:saas --port 8080\n' +
        'Websir live preview serving \'Kestrel SaaS\' at http://127.0.0.1:8080\n\n' +
        'GET /index.html    200 OK  text/html; charset=utf-8\n' +
        'GET /styles.css    200 OK  text/css; charset=utf-8\n' +
        'GET /site.js       200 OK  application/javascript\n'
    },
    cli: {
      title: 'Scriptable Headless CLI for CI/CD Pipelines',
      desc: 'Initialize templates, compile sites, and package deployment bundles directly from your terminal.',
      ctaText: 'Open CLI Reference',
      ctaHref: '#deployment',
      mode: 'code',
      code:
        '# Generate starter project JSON\n' +
        'websir --init-template agency --out studio.websir.json\n\n' +
        '# Compile to ZIP or Google Sites single-file HTML\n' +
        'websir --export studio.websir.json --out dist/site.zip --format zip\n' +
        'websir --export studio.websir.json --out dist/embed.html --format google-sites\n'
    }
  };

  var frameHost = document.getElementById('showcase-frame');
  var captionTitle = document.getElementById('showcase-title');
  var captionDesc = document.getElementById('showcase-desc');
  var captionCta = document.getElementById('showcase-cta');

  function renderShowcase(key) {
    var d = showcaseData[key];
    if (!d || !frameHost) return;

    if (d.mode === 'studio') {
      frameHost.innerHTML =
        '<div class="studio-mock">' +
        '  <div class="studio-mock-top">' +
        '    <div class="studio-mock-brand"><span>WEBSIR STUDIO</span><span class="studio-mock-pill">' + d.badge + '</span></div>' +
        '    <div class="studio-mock-pills">' +
        '      <span class="studio-mock-pill">Desktop 1280</span>' +
        '      <span class="studio-mock-pill">Grid</span>' +
        '      <span class="studio-mock-pill is-white">Export / Publish</span>' +
        '    </div>' +
        '  </div>' +
        '  <div class="studio-mock-body">' +
        '    <div class="mock-sidebar">' +
        '      <div style="color:#fff;font-weight:600;margin-bottom:4px;">SECTION LAYERS</div>' +
        '      <div class="mock-row"><span>Navbar</span><span>z:1</span></div>' +
        '      <div class="mock-row" style="border-color:#fff;"><span>Hero Heading</span><span>H1</span></div>' +
        '      <div class="mock-row"><span>Lead Copy</span><span>P</span></div>' +
        '      <div class="mock-row"><span>Primary CTA</span><span>BTN</span></div>' +
        '    </div>' +
        '    <div class="mock-canvas">' +
        '      <div class="mock-artboard">' +
        '        <div class="mock-selected-box">' +
        '          <span class="mock-selected-tag">H1 • 680 × 84</span>' +
        '          <span class="mock-handle nw"></span><span class="mock-handle ne"></span>' +
        '          <span class="mock-handle sw"></span><span class="mock-handle se"></span>' +
        '          <div style="font-size:20px;font-weight:600;color:#fff;line-height:1.2;">' + d.headline + '</div>' +
        '        </div>' +
        '        <div style="font-size:13px;color:#94a3b8;margin-bottom:16px;">' + d.sub + '</div>' +
        '        <span style="display:inline-block;padding:8px 16px;background:#fff;color:#0f172a;border-radius:6px;font-size:12px;font-weight:600;">Explore Work</span>' +
        '      </div>' +
        '    </div>' +
        '    <div class="mock-inspector">' +
        '      <div style="color:#fff;font-weight:600;margin-bottom:4px;">INSPECTOR • STYLE</div>' +
        '      <div class="mock-row"><span>Font Size</span><span>56 px</span></div>' +
        '      <div class="mock-row"><span>Fill</span><span>#FFFFFF</span></div>' +
        '      <div class="mock-row"><span>Snap Grid</span><span>8 px</span></div>' +
        '      <div class="mock-row"><span>Hover</span><span>Lift</span></div>' +
        '    </div>' +
        '  </div>' +
        '</div>';
    } else {
      frameHost.innerHTML =
        '<div class="studio-mock">' +
        '  <div class="studio-mock-top">' +
        '    <div class="studio-mock-brand"><span>WEBSIR COMPILER OUTPUT</span></div>' +
        '    <div class="studio-mock-pills"><span class="studio-mock-pill is-white">Deterministic Build</span></div>' +
        '  </div>' +
        '  <pre class="mock-code-panel" style="margin:0;"><code>' + escapeHtml(d.code) + '</code></pre>' +
        '</div>';
    }

    if (captionTitle) captionTitle.textContent = d.title;
    if (captionDesc) captionDesc.textContent = d.desc;
    if (captionCta) {
      captionCta.textContent = d.ctaText;
      captionCta.setAttribute('href', d.ctaHref);
    }
  }

  function escapeHtml(str) {
    return str
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;');
  }

  document.querySelectorAll('.showcase-tab').forEach(function (tab) {
    tab.addEventListener('click', function () {
      document.querySelectorAll('.showcase-tab').forEach(function (t) {
        t.classList.remove('is-active');
        t.setAttribute('aria-selected', 'false');
      });
      tab.classList.add('is-active');
      tab.setAttribute('aria-selected', 'true');
      renderShowcase(tab.getAttribute('data-tab'));
    });
  });

  renderShowcase('canvas');

  // Outcomes Accordion
  document.querySelectorAll('.accordion-trigger').forEach(function (btn) {
    btn.addEventListener('click', function () {
      var item = btn.closest('.accordion-item');
      var isOpen = item.classList.contains('is-open');
      document.querySelectorAll('.accordion-item').forEach(function (el) {
        el.classList.remove('is-open');
        var trig = el.querySelector('.accordion-trigger');
        if (trig) trig.setAttribute('aria-expanded', 'false');
        var icon = el.querySelector('.acc-icon');
        if (icon) icon.textContent = 'v';
      });
      if (!isOpen) {
        item.classList.add('is-open');
        btn.setAttribute('aria-expanded', 'true');
        var icon = item.querySelector('.acc-icon');
        if (icon) icon.textContent = '^';
      }
    });
  });

  // Copy code buttons
  document.querySelectorAll('.copy-code-btn').forEach(function (btn) {
    btn.addEventListener('click', function () {
      var block = btn.closest('.code-block');
      var codeEl = block ? block.querySelector('code') : null;
      if (!codeEl || !navigator.clipboard) return;
      navigator.clipboard.writeText(codeEl.textContent).then(function () {
        var old = btn.textContent;
        btn.textContent = 'Copied';
        setTimeout(function () { btn.textContent = old; }, 1600);
      });
    });
  });
})();
