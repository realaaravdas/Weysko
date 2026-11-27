(function () {
  // Mobile navigation drawer
  var header = document.getElementById('site-header');
  var menuBtn = document.getElementById('mobile-menu-btn');
  if (header && menuBtn) {
    function closeMenu() {
      header.classList.remove('is-open');
      menuBtn.setAttribute('aria-expanded', 'false');
    }

    menuBtn.addEventListener('click', function () {
      var open = header.classList.toggle('is-open');
      menuBtn.setAttribute('aria-expanded', open ? 'true' : 'false');
    });
    header.querySelectorAll('.main-nav a').forEach(function (link) {
      link.addEventListener('click', closeMenu);
    });
    document.addEventListener('keydown', function (e) {
      if (e.key === 'Escape' && header.classList.contains('is-open')) {
        closeMenu();
        menuBtn.focus();
      }
    });
    document.addEventListener('click', function (e) {
      if (header.classList.contains('is-open') && !header.contains(e.target)) {
        closeMenu();
      }
    });
    window.addEventListener('resize', function () {
      if (window.innerWidth > 960 && header.classList.contains('is-open')) {
        closeMenu();
      }
    });
  }

  // Full-bleed edge-to-edge ambient vector filaments in hero (with pause/play toggle)
  var canvas = document.getElementById('hero-canvas');
  var pauseBtn = document.getElementById('hero-pause-btn');
  var heroSection = document.querySelector('.hero-section');
  var animRunning = true;
  var phase = 0;

  if (canvas && canvas.getContext) {
    var ctx = canvas.getContext('2d');
    var canvasW = 0;
    var canvasH = 0;
    var rafPending = false;

    function renderHeroCanvas() {
      var w = canvasW;
      var h = canvasH;
      if (w <= 0 || h <= 0) {
        return;
      }
      ctx.clearRect(0, 0, w, h);

      var isCompact = w < 640;

      // Centered full-bleed cobalt/cyan ambient radial wash
      var glowRadius = Math.max(w * 0.55, h * 0.72);
      var grad = ctx.createRadialGradient(
        w * 0.5,
        h * 0.54,
        12,
        w * 0.5,
        h * 0.54,
        glowRadius
      );
      grad.addColorStop(0, isCompact ? 'rgba(0, 188, 235, 0.11)' : 'rgba(0, 188, 235, 0.14)');
      grad.addColorStop(0.38, isCompact ? 'rgba(0, 96, 255, 0.08)' : 'rgba(0, 96, 255, 0.11)');
      grad.addColorStop(0.68, 'rgba(7, 24, 45, 0.06)');
      grad.addColorStop(1, 'rgba(0, 0, 0, 0)');
      ctx.fillStyle = grad;
      ctx.fillRect(0, 0, w, h);

      // Full-width architectural wave filaments fading seamlessly on both horizontal edges
      var strands = 26;
      var step = Math.max(10, Math.round(w / 96));
      var spread = Math.min(h * 0.36, 170);
      var amp1 = Math.min(h * 0.09, 44);
      var amp2 = Math.min(h * 0.06, 28);
      var sweep = Math.min(h * 0.16, 72);
      var opacityScale = isCompact ? 0.65 : 1;

      for (var i = 0; i < strands; i++) {
        var frac = i / (strands - 1);
        ctx.beginPath();
        for (var x = -24; x <= w + 24; x += step) {
          var nx = x / w;
          var wave =
            Math.sin(nx * 4.4 + phase + frac * 1.9) * amp1 +
            Math.cos(nx * 2.5 - phase * 0.7 + frac * 1.4) * amp2;
          var y = h * 0.58 + (0.5 - nx) * sweep + (frac - 0.5) * spread + wave;
          if (x === -24) ctx.moveTo(x, y);
          else ctx.lineTo(x, y);
        }
        var alpha = (0.045 + (1 - Math.abs(frac - 0.5) * 2) * 0.18) * opacityScale;
        var strokeGrad = ctx.createLinearGradient(0, 0, w, 0);
        if (i % 4 === 0) {
          strokeGrad.addColorStop(0, 'rgba(0, 188, 235, 0)');
          strokeGrad.addColorStop(0.18, 'rgba(0, 188, 235, ' + alpha.toFixed(3) + ')');
          strokeGrad.addColorStop(0.82, 'rgba(0, 188, 235, ' + alpha.toFixed(3) + ')');
          strokeGrad.addColorStop(1, 'rgba(0, 188, 235, 0)');
        } else {
          var mutedAlpha = (alpha * 0.85).toFixed(3);
          strokeGrad.addColorStop(0, 'rgba(226, 232, 240, 0)');
          strokeGrad.addColorStop(0.18, 'rgba(226, 232, 240, ' + mutedAlpha + ')');
          strokeGrad.addColorStop(0.82, 'rgba(226, 232, 240, ' + mutedAlpha + ')');
          strokeGrad.addColorStop(1, 'rgba(226, 232, 240, 0)');
        }
        ctx.strokeStyle = strokeGrad;
        ctx.lineWidth = i % 5 === 0 ? 1.35 : 0.85;
        ctx.stroke();
      }

      // Seamless top and bottom black edge fades
      var topFade = ctx.createLinearGradient(0, 0, 0, h * 0.18);
      topFade.addColorStop(0, '#000000');
      topFade.addColorStop(1, 'rgba(0, 0, 0, 0)');
      ctx.fillStyle = topFade;
      ctx.fillRect(0, 0, w, h * 0.18);

      var botFade = ctx.createLinearGradient(0, h * 0.72, 0, h);
      botFade.addColorStop(0, 'rgba(0, 0, 0, 0)');
      botFade.addColorStop(1, '#000000');
      ctx.fillStyle = botFade;
      ctx.fillRect(0, h * 0.72, w, h * 0.28);
    }

    function resizeCanvas() {
      var dpr = Math.min(window.devicePixelRatio || 1, 2);
      var rect = canvas.getBoundingClientRect();
      canvasW = rect.width;
      canvasH = rect.height;
      canvas.width = Math.max(1, Math.floor(canvasW * dpr));
      canvas.height = Math.max(1, Math.floor(canvasH * dpr));
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
      renderHeroCanvas();
    }

    resizeCanvas();
    window.addEventListener('resize', resizeCanvas);
    if (typeof ResizeObserver !== 'undefined' && heroSection) {
      var heroObserver = new ResizeObserver(resizeCanvas);
      heroObserver.observe(heroSection);
    }

    function drawFrame() {
      rafPending = false;
      renderHeroCanvas();
      if (animRunning) {
        phase += 0.008;
        rafPending = true;
        requestAnimationFrame(drawFrame);
      }
    }

    rafPending = true;
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
      if (animRunning && typeof drawFrame === 'function' && !rafPending) {
        rafPending = true;
        requestAnimationFrame(drawFrame);
      }
    });
  }

  // Tabbed Studio Showcase
  var showcaseData = {
    canvas: {
      title: 'Freeform WYSIWYG Canvas with 8px Snap & Smart Guides',
      desc: 'Drag elements, resize via 8-point handles, edit typography inline, and inspect live X/Y coordinates.',
      ctaText: 'Download Installer v1.3.1',
      ctaHref: 'https://github.com/realaaravdas/Weysko/releases/download/v1.3.1/weysko-v1.3.1-windows-x86_64-setup.exe',
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
        'weysko-dist/\n' +
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
        '$ weysko --serve template:saas --port 8080\n' +
        'Weysko live preview serving \'Kestrel SaaS\' at http://127.0.0.1:8080\n\n' +
        'GET /index.html    200 OK  text/html; charset=utf-8\n' +
        'GET /styles.css    200 OK  text/css; charset=utf-8\n' +
        'GET /site.js       200 OK  application/javascript\n'
    },
    cli: {
      title: 'Scriptable Headless CLI & Portable .weysko Bundles',
      desc: 'Initialize .weysko templates, compile static folders, ZIP archives, or Google Sites embeds directly from your terminal.',
      ctaText: 'Open CLI Guide',
      ctaHref: 'guide.html#cli-reference',
      mode: 'code',
      code:
        '# Generate starter .weysko project file\n' +
        'weysko --init-template meridian --out meridian.weysko\n\n' +
        '# Export to Folder, ZIP, Google Sites, or portable .weysko\n' +
        'weysko --export meridian.weysko --out dist/site.zip --format zip\n' +
        'weysko --export meridian.weysko --out dist/copy.weysko --format weysko\n'
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
        '    <div class="studio-mock-brand"><span>WEYSKO STUDIO</span><span class="studio-mock-pill">' + d.badge + '</span></div>' +
        '    <div class="studio-mock-pills">' +
        '      <span class="studio-mock-pill is-secondary">Desktop 1280</span>' +
        '      <span class="studio-mock-pill is-secondary">Grid</span>' +
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
        '          <div class="mock-artboard-heading">' + d.headline + '</div>' +
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
        '    <div class="studio-mock-brand"><span>WEYSKO COMPILER OUTPUT</span></div>' +
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
