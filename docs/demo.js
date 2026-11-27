(function () {
  // Mobile nav toggle shared with main site
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
    header.querySelectorAll('.main-nav a').forEach(function (a) {
      a.addEventListener('click', closeMenu);
    });
  }

  var THEMES = {
    ObsidianSlate: {
      label: 'Obsidian Slate',
      font: 'Inter',
      bg: '#090D16',
      surface: '#0F172A',
      surfaceAlt: '#1E293B',
      primary: '#FFFFFF',
      primaryText: '#0F172A',
      text: '#F8FAFC',
      muted: '#94A3B8',
      border: '#334155',
      radius: 10
    },
    PureEditorial: {
      label: 'Pure Editorial',
      font: 'Plus Jakarta Sans',
      bg: '#FFFFFF',
      surface: '#F8FAFC',
      surfaceAlt: '#F1F5F9',
      primary: '#0F172A',
      primaryText: '#FFFFFF',
      text: '#0F172A',
      muted: '#475569',
      border: '#E2E8F0',
      radius: 8
    },
    MidnightMono: {
      label: 'Midnight Mono',
      font: 'Space Grotesk',
      bg: '#0B0F17',
      surface: '#111827',
      surfaceAlt: '#1F2937',
      primary: '#F9FAFB',
      primaryText: '#111827',
      text: '#F3F4F6',
      muted: '#9CA3AF',
      border: '#374151',
      radius: 6
    },
    NordicSteel: {
      label: 'Nordic Steel',
      font: 'DM Sans',
      bg: '#F1F5F9',
      surface: '#FFFFFF',
      surfaceAlt: '#E2E8F0',
      primary: '#1E293B',
      primaryText: '#FFFFFF',
      text: '#0F172A',
      muted: '#64748B',
      border: '#CBD5E1',
      radius: 12
    },
    WarmAlabaster: {
      label: 'Warm Alabaster',
      font: 'Instrument Serif',
      bg: '#FAF9F6',
      surface: '#FFFFFF',
      surfaceAlt: '#F3F1EC',
      primary: '#18181B',
      primaryText: '#FAFAFA',
      text: '#18181B',
      muted: '#71717A',
      border: '#E4E4E7',
      radius: 8
    },
    CarbonInk: {
      label: 'Carbon Ink',
      font: 'Syne',
      bg: '#05070B',
      surface: '#0D1117',
      surfaceAlt: '#161B22',
      primary: '#FFFFFF',
      primaryText: '#05070B',
      text: '#F8FAFC',
      muted: '#8B949E',
      border: '#30363D',
      radius: 4
    }
  };

  function makeId() {
    return 'el-' + Math.random().toString(36).slice(2, 9);
  }

  function buildTemplate(key, themeKey) {
    var t = THEMES[themeKey] || THEMES.ObsidianSlate;
    if (key === 'meridian') {
      return {
        name: 'Meridian Cloud',
        base_url: 'https://meridian.example.com',
        themeKey: themeKey,
        pages: [
          {
            id: 'page-home',
            title: 'Home',
            slug: 'index',
            sections: [
              {
                id: 'sec-hero',
                name: 'Meridian Hero',
                altSurface: false,
                elements: [
                  { id: makeId(), kind: 'Badge', text: 'MERIDIAN FABRIC v4', fontSize: 12, radius: 20, pad: 8 },
                  { id: makeId(), kind: 'Heading', text: 'Deterministic edge compute for distributed teams.', fontSize: 38, radius: 4, pad: 4 },
                  { id: makeId(), kind: 'Paragraph', text: 'Deploy reproducible workloads across 42 bare-metal regions with hardware-isolated networking and sub-15ms routing.', fontSize: 16, radius: 4, pad: 4 },
                  { id: makeId(), kind: 'Button', text: 'View Pricing', fontSize: 14, radius: t.radius, pad: 12 }
                ]
              },
              {
                id: 'sec-features',
                name: 'Core Architecture',
                altSurface: true,
                elements: [
                  { id: makeId(), kind: 'Heading', text: 'Engineered for predictable uptime', fontSize: 28, radius: 4, pad: 4 },
                  { id: makeId(), kind: 'Card', title: 'Zero-Jitter Edge', text: 'Dedicated NVMe instances with deterministic packet scheduling.', fontSize: 15, radius: t.radius, pad: 16 },
                  { id: makeId(), kind: 'Card', title: 'Immutable Deploys', text: 'Cryptographic artifact manifests ensure byte-for-byte parity.', fontSize: 15, radius: t.radius, pad: 16 }
                ]
              }
            ]
          },
          {
            id: 'page-pricing',
            title: 'Pricing',
            slug: 'pricing',
            sections: [
              {
                id: 'sec-pricing',
                name: 'Pricing Tiers',
                altSurface: false,
                elements: [
                  { id: makeId(), kind: 'Heading', text: 'Simple, transparent pricing', fontSize: 32, radius: 4, pad: 4 },
                  { id: makeId(), kind: 'Card', title: 'Starter — $0 / month', text: '5 static sites, 100 GB edge bandwidth, automated TLS.', fontSize: 15, radius: t.radius, pad: 16 },
                  { id: makeId(), kind: 'Card', title: 'Studio — $29 / month', text: 'Unlimited sites, custom domains, team seats & CI hooks.', fontSize: 15, radius: t.radius, pad: 16 }
                ]
              }
            ]
          },
          {
            id: 'page-contact',
            title: 'Contact',
            slug: 'contact',
            sections: [
              {
                id: 'sec-contact',
                name: 'Contact Strip',
                altSurface: true,
                elements: [
                  { id: makeId(), kind: 'Heading', text: 'Connect with Meridian Engineering', fontSize: 30, radius: 4, pad: 4 },
                  { id: makeId(), kind: 'Paragraph', text: 'Email architecture@meridian.example.com or launch a cluster today.', fontSize: 15, radius: 4, pad: 4 },
                  { id: makeId(), kind: 'Button', text: 'Send Inquiry', fontSize: 14, radius: t.radius, pad: 12 }
                ]
              }
            ]
          }
        ]
      };
    }
    if (key === 'portfolio') {
      return {
        name: 'Slate Portfolio',
        base_url: 'https://example.com',
        themeKey: themeKey,
        pages: [
          {
            id: 'page-home',
            title: 'Home',
            slug: 'index',
            sections: [
              {
                id: 'sec-hero',
                name: 'Hero Split',
                altSurface: false,
                elements: [
                  { id: makeId(), kind: 'Badge', text: 'INDEPENDENT PRACTICE', fontSize: 12, radius: 20, pad: 8 },
                  { id: makeId(), kind: 'Heading', text: 'Interfaces shaped by clarity, contrast, and speed.', fontSize: 36, radius: 4, pad: 4 },
                  { id: makeId(), kind: 'Button', text: 'Start a Project', fontSize: 14, radius: t.radius, pad: 12 }
                ]
              }
            ]
          }
        ]
      };
    }
    if (key === 'saas') {
      return {
        name: 'Kestrel SaaS',
        base_url: 'https://example.com',
        themeKey: themeKey,
        pages: [
          {
            id: 'page-home',
            title: 'Home',
            slug: 'index',
            sections: [
              {
                id: 'sec-hero',
                name: 'Hero Centered',
                altSurface: false,
                elements: [
                  { id: makeId(), kind: 'Badge', text: 'MINIMALIST DIGITAL STUDIO', fontSize: 12, radius: 20, pad: 8 },
                  { id: makeId(), kind: 'Heading', text: 'Build crisp websites with quiet architectural precision.', fontSize: 36, radius: 4, pad: 4 },
                  { id: makeId(), kind: 'Button', text: 'Get Started', fontSize: 14, radius: t.radius, pad: 12 }
                ]
              }
            ]
          }
        ]
      };
    }
    return {
      name: 'Untitled Site',
      base_url: 'https://example.com',
      themeKey: themeKey,
      pages: [
        {
          id: 'page-home',
          title: 'Home',
          slug: 'index',
          sections: [
            {
              id: 'sec-blank',
              name: 'Hero Strip',
              altSurface: false,
              elements: [
                { id: makeId(), kind: 'Heading', text: 'Your Custom Website', fontSize: 36, radius: 4, pad: 4 },
                { id: makeId(), kind: 'Paragraph', text: 'Add elements from the left panel or edit typography on the right.', fontSize: 16, radius: 4, pad: 4 }
              ]
            }
          ]
        }
      ]
    };
  }

  var state = {
    selectedTheme: 'ObsidianSlate',
    project: null,
    activePageIndex: 0,
    selectedElementId: null,
    viewport: 'desktop',
    showGrid: true
  };

  var homeView = document.getElementById('demo-home-view');
  var editorView = document.getElementById('demo-editor-view');
  var resumeBtn = document.getElementById('demo-resume-btn');
  var styleGrid = document.getElementById('demo-style-grid');
  var templateGrid = document.getElementById('demo-template-grid');
  var toastEl = document.getElementById('demo-toast');
  var toastTimer = null;

  function notify(msg) {
    if (!toastEl) return;
    toastEl.textContent = msg;
    toastEl.classList.add('is-shown');
    if (toastTimer) clearTimeout(toastTimer);
    toastTimer = setTimeout(function () {
      toastEl.classList.remove('is-shown');
    }, 2600);
  }

  function renderHomeStyles() {
    if (!styleGrid) return;
    styleGrid.innerHTML = '';
    Object.keys(THEMES).forEach(function (k) {
      var t = THEMES[k];
      var btn = document.createElement('button');
      btn.type = 'button';
      btn.className = 'demo-style-card' + (state.selectedTheme === k ? ' is-selected' : '');
      btn.innerHTML =
        '<div class="demo-swatch-bar">' +
        '  <span style="background:' + t.bg + '"></span>' +
        '  <span style="background:' + t.surface + '"></span>' +
        '  <span style="background:' + t.primary + '"></span>' +
        '  <span style="background:' + t.text + '"></span>' +
        '</div>' +
        '<strong>' + t.label + '</strong>' +
        '<span>' + t.font + '</span>';
      btn.addEventListener('click', function () {
        state.selectedTheme = k;
        renderHomeStyles();
      });
      styleGrid.appendChild(btn);
    });
  }

  function renderHomeTemplates() {
    if (!templateGrid) return;
    var list = [
      { key: 'meridian', title: 'Meridian Cloud (Example Site)', badge: 'DEMO', desc: 'Complete 3-page cloud infrastructure showcase matching the v1.3.2 tutorial.' },
      { key: 'portfolio', title: 'Minimalist Slate Portfolio', badge: '', desc: 'Asymmetric architectural hero, studio work strip & clean contact flow.' },
      { key: 'saas', title: 'SaaS Product Landing', badge: '', desc: 'Centered launch hero, capabilities grid, and conversion CTA.' },
      { key: 'blank', title: 'Blank Canvas', badge: '', desc: 'Minimalist starting page ready for custom elements and strips.' }
    ];
    templateGrid.innerHTML = '';
    list.forEach(function (item) {
      var card = document.createElement('div');
      card.className = 'demo-tmpl-card' + (item.badge ? ' is-featured' : '');
      card.innerHTML =
        '<div class="demo-tmpl-head"><h3>' + item.title + '</h3>' +
        (item.badge ? '<span class="demo-tmpl-badge">' + item.badge + '</span>' : '') +
        '</div>' +
        '<p>' + item.desc + '</p>' +
        '<div><button class="demo-btn-white demo-btn-sm" type="button">Open in Studio &rarr;</button></div>';
      card.querySelector('button').addEventListener('click', function () {
        openTemplate(item.key);
      });
      templateGrid.appendChild(card);
    });
  }

  function openTemplate(key) {
    state.project = buildTemplate(key, state.selectedTheme);
    state.activePageIndex = 0;
    state.selectedElementId = null;
    enterEditor();
    notify('Loaded ' + state.project.name + ' (' + THEMES[state.selectedTheme].label + ')');
  }

  function enterEditor() {
    homeView.classList.add('is-hidden');
    editorView.classList.remove('is-hidden');
    if (resumeBtn) resumeBtn.classList.remove('is-hidden');
    renderEditorAll();
  }

  function enterHome() {
    editorView.classList.add('is-hidden');
    homeView.classList.remove('is-hidden');
  }

  function currentPage() {
    if (!state.project || !state.project.pages.length) return null;
    return state.project.pages[state.activePageIndex] || state.project.pages[0];
  }

  function findSelectedElement() {
    var page = currentPage();
    if (!page || !state.selectedElementId) return null;
    for (var i = 0; i < page.sections.length; i++) {
      var sec = page.sections[i];
      for (var j = 0; j < sec.elements.length; j++) {
        if (sec.elements[j].id === state.selectedElementId) {
          return { sec: sec, el: sec.elements[j] };
        }
      }
    }
    return null;
  }

  function renderEditorAll() {
    var nameInput = document.getElementById('demo-site-name');
    var pageSelect = document.getElementById('demo-page-select');
    if (nameInput && state.project) nameInput.value = state.project.name;
    if (pageSelect && state.project) {
      pageSelect.innerHTML = '';
      state.project.pages.forEach(function (p, idx) {
        var opt = document.createElement('option');
        opt.value = String(idx);
        opt.textContent = '/' + (p.slug === 'index' ? 'index.html' : p.slug + '.html');
        if (idx === state.activePageIndex) opt.selected = true;
        pageSelect.appendChild(opt);
      });
    }
    renderEditorThemes();
    renderArtboard();
    renderInspector();
    renderStatusBar();
  }

  function renderEditorThemes() {
    var host = document.getElementById('demo-editor-themes');
    if (!host || !state.project) return;
    host.innerHTML = '';
    Object.keys(THEMES).forEach(function (k) {
      var t = THEMES[k];
      var b = document.createElement('button');
      b.type = 'button';
      b.className = 'demo-theme-pill' + (state.project.themeKey === k ? ' is-active' : '');
      b.textContent = t.label;
      b.addEventListener('click', function () {
        state.project.themeKey = k;
        renderEditorAll();
        notify('Applied ' + t.label);
      });
      host.appendChild(b);
    });
  }

  function renderArtboard() {
    var artboard = document.getElementById('demo-artboard');
    if (!artboard || !state.project) return;
    var t = THEMES[state.project.themeKey] || THEMES.ObsidianSlate;
    artboard.className = 'demo-artboard vp-' + state.viewport;
    artboard.style.backgroundColor = t.bg;
    artboard.style.color = t.text;
    artboard.innerHTML = '';

    var page = currentPage();
    if (!page) return;

    page.sections.forEach(function (sec) {
      var secEl = document.createElement('div');
      secEl.className = 'demo-sec';
      secEl.style.backgroundColor = sec.altSurface ? t.surface : t.bg;
      secEl.style.borderBottomColor = t.border;

      var head = document.createElement('div');
      head.className = 'demo-sec-head';
      head.textContent = sec.name;
      secEl.appendChild(head);

      var list = document.createElement('div');
      list.className = 'demo-sec-elements';

      sec.elements.forEach(function (el) {
        var box = document.createElement('div');
        box.className = 'demo-el-item' + (el.id === state.selectedElementId ? ' is-selected' : '');

        var tag = document.createElement('span');
        tag.className = 'demo-el-tag';
        tag.textContent = el.kind.toUpperCase();
        box.appendChild(tag);

        if (el.kind === 'Heading') {
          var h = document.createElement('h2');
          h.textContent = el.text;
          h.style.margin = '0';
          h.style.fontSize = el.fontSize + 'px';
          h.style.lineHeight = '1.15';
          h.style.color = el.colorOverride || t.text;
          box.appendChild(h);
        } else if (el.kind === 'Paragraph') {
          var p = document.createElement('p');
          p.textContent = el.text;
          p.style.margin = '0';
          p.style.fontSize = el.fontSize + 'px';
          p.style.color = el.colorOverride || t.muted;
          box.appendChild(p);
        } else if (el.kind === 'Button') {
          var b = document.createElement('span');
          b.textContent = el.text;
          b.style.display = 'inline-block';
          b.style.padding = el.pad + 'px ' + (el.pad + 10) + 'px';
          b.style.borderRadius = el.radius + 'px';
          b.style.fontSize = el.fontSize + 'px';
          b.style.fontWeight = '600';
          b.style.backgroundColor = el.bgOverride || t.primary;
          b.style.color = el.colorOverride || t.primaryText;
          box.appendChild(b);
        } else if (el.kind === 'Badge') {
          var bg = document.createElement('span');
          bg.textContent = el.text;
          bg.style.display = 'inline-block';
          bg.style.padding = '4px 12px';
          bg.style.borderRadius = '9999px';
          bg.style.border = '1px solid ' + t.border;
          bg.style.backgroundColor = el.bgOverride || t.surfaceAlt;
          bg.style.color = el.colorOverride || t.text;
          bg.style.fontSize = el.fontSize + 'px';
          bg.style.fontWeight = '600';
          box.appendChild(bg);
        } else if (el.kind === 'Card') {
          var card = document.createElement('div');
          card.style.backgroundColor = el.bgOverride || t.surfaceAlt;
          card.style.border = '1px solid ' + t.border;
          card.style.borderRadius = el.radius + 'px';
          card.style.padding = el.pad + 'px';
          var ct = document.createElement('div');
          ct.textContent = el.title || 'Feature Card';
          ct.style.fontWeight = '700';
          ct.style.fontSize = (el.fontSize + 2) + 'px';
          ct.style.marginBottom = '6px';
          var cb = document.createElement('div');
          cb.textContent = el.text;
          cb.style.fontSize = el.fontSize + 'px';
          cb.style.color = el.colorOverride || t.muted;
          card.appendChild(ct);
          card.appendChild(cb);
          box.appendChild(card);
        } else {
          var div = document.createElement('hr');
          div.style.border = '0';
          div.style.borderTop = '1px solid ' + t.border;
          div.style.margin = '6px 0';
          box.appendChild(div);
        }

        box.addEventListener('click', function (ev) {
          ev.stopPropagation();
          state.selectedElementId = el.id;
          renderArtboard();
          renderInspector();
          renderStatusBar();
        });
        list.appendChild(box);
      });

      secEl.appendChild(list);
      artboard.appendChild(secEl);
    });
  }

  function renderInspector() {
    var body = document.getElementById('demo-inspector-body');
    if (!body) return;
    var found = findSelectedElement();
    if (!found) {
      body.innerHTML = '<p class="demo-muted">Click any element on the canvas to edit text, styling, radius, or padding.</p>';
      return;
    }
    var el = found.el;
    body.innerHTML =
      '<div class="demo-field"><label>Element Type</label><input type="text" value="' + el.kind + '" disabled></div>' +
      (el.kind === 'Card'
        ? '<div class="demo-field"><label>Card Title</label><input id="insp-title" type="text"></div>'
        : '') +
      (el.kind !== 'Divider'
        ? '<div class="demo-field"><label>Text / Copy</label><textarea id="insp-text" rows="3"></textarea></div>'
        : '') +
      '<div class="demo-field"><label>Font Size (px)</label><input id="insp-size" type="number" min="10" max="72"></div>' +
      '<div class="demo-field"><label>Corner Radius (px)</label><input id="insp-radius" type="number" min="0" max="40"></div>' +
      '<div class="demo-field"><label>Padding (px)</label><input id="insp-pad" type="number" min="0" max="48"></div>' +
      '<div class="demo-insp-actions">' +
      '  <button class="demo-tool-btn" id="insp-dup-btn" type="button">Duplicate</button>' +
      '  <button class="demo-tool-btn" id="insp-del-btn" type="button" style="color:#f87171;">Delete</button>' +
      '</div>';

    var titleIn = document.getElementById('insp-title');
    var textIn = document.getElementById('insp-text');
    var sizeIn = document.getElementById('insp-size');
    var radiusIn = document.getElementById('insp-radius');
    var padIn = document.getElementById('insp-pad');

    if (titleIn) {
      titleIn.value = el.title || '';
      titleIn.addEventListener('input', function () {
        el.title = titleIn.value;
        renderArtboard();
      });
    }
    if (textIn) {
      textIn.value = el.text || '';
      textIn.addEventListener('input', function () {
        el.text = textIn.value;
        renderArtboard();
      });
    }
    if (sizeIn) {
      sizeIn.value = String(el.fontSize || 16);
      sizeIn.addEventListener('input', function () {
        el.fontSize = Math.max(10, Math.min(72, Number(sizeIn.value) || 16));
        renderArtboard();
      });
    }
    if (radiusIn) {
      radiusIn.value = String(el.radius || 0);
      radiusIn.addEventListener('input', function () {
        el.radius = Math.max(0, Math.min(40, Number(radiusIn.value) || 0));
        renderArtboard();
      });
    }
    if (padIn) {
      padIn.value = String(el.pad || 0);
      padIn.addEventListener('input', function () {
        el.pad = Math.max(0, Math.min(48, Number(padIn.value) || 0));
        renderArtboard();
      });
    }

    document.getElementById('insp-dup-btn').addEventListener('click', function () {
      var clone = JSON.parse(JSON.stringify(el));
      clone.id = makeId();
      found.sec.elements.push(clone);
      state.selectedElementId = clone.id;
      renderEditorAll();
      notify('Duplicated ' + el.kind);
    });
    document.getElementById('insp-del-btn').addEventListener('click', function () {
      found.sec.elements = found.sec.elements.filter(function (x) {
        return x.id !== el.id;
      });
      state.selectedElementId = null;
      renderEditorAll();
      notify('Deleted ' + el.kind);
    });
  }

  function renderStatusBar() {
    var left = document.getElementById('demo-status-left');
    var right = document.getElementById('demo-status-right');
    var page = currentPage();
    var count = 0;
    if (page) {
      page.sections.forEach(function (s) {
        count += s.elements.length;
      });
    }
    var found = findSelectedElement();
    if (left && page) {
      left.textContent =
        '/' + (page.slug === 'index' ? 'index.html' : page.slug + '.html') +
        '  •  ' + count + ' elements' +
        (found ? '  •  Selected: ' + found.el.kind : '');
    }
    if (right && state.project) {
      var t = THEMES[state.project.themeKey] || THEMES.ObsidianSlate;
      right.textContent = t.label + '  •  ' + state.viewport.toUpperCase();
    }
  }

  // Export portable .weysko envelope file compatible with the desktop Rust app
  function exportWeyskoFile() {
    if (!state.project) return;
    var t = THEMES[state.project.themeKey] || THEMES.ObsidianSlate;
    var slugName = state.project.name.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '') || 'weysko-site';
    var desktopPages = state.project.pages.map(function (p) {
      return {
        id: p.id,
        title: p.title,
        slug: p.slug,
        meta_description: '',
        custom_head_html: '',
        background_override: null,
        sections: p.sections.map(function (s) {
          var yCursor = 40;
          return {
            id: s.id,
            name: s.name,
            anchor_id: '',
            height: Math.max(280, s.elements.length * 90 + 100),
            full_bleed: false,
            sticky: false,
            background: { Solid: { color: s.altSurface ? t.surface : t.bg } },
            border_top: false,
            border_bottom: true,
            mobile_layout: 'AutoStack',
            elements: s.elements.map(function (el, idx) {
              var h = el.kind === 'Heading' ? 64 : el.kind === 'Card' ? 120 : el.kind === 'Divider' ? 4 : 48;
              var y = yCursor;
              yCursor += h + 20;
              return {
                id: el.id,
                name: el.kind,
                kind: el.kind === 'Card' ? 'Card' : el.kind,
                rect: { x: 60, y: y, w: el.kind === 'Button' || el.kind === 'Badge' ? 220 : 880, h: h },
                tablet_rect: null,
                mobile_rect: null,
                full_width: false,
                visible_desktop: true,
                visible_tablet: true,
                visible_mobile: true,
                locked: false,
                hidden: false,
                z_index: idx + 1,
                content: {
                  text: el.text || '',
                  heading_level: 2,
                  href: '#top',
                  link_target_page_slug: null,
                  open_in_new_tab: false,
                  asset_id: null,
                  alt_text: '',
                  object_fit: 'Cover',
                  nav_brand: state.project.name.toUpperCase(),
                  nav_links: [],
                  nav_cta_label: 'Contact',
                  nav_cta_href: '#top',
                  list_items: [],
                  list_style: 'Bullet',
                  card_eyebrow: '01',
                  card_title: el.title || 'Feature Card',
                  card_body: el.text || '',
                  card_meta: 'Pure Rust',
                  card_cta: '',
                  card_href: '',
                  accordion_items: [],
                  form_title: '',
                  form_submit_label: 'Send',
                  form_action_mode: 'ToastDemo',
                  form_action_url: '',
                  form_success_message: '',
                  form_fields: [],
                  video_url: '',
                  video_autoplay: false,
                  video_muted: true,
                  custom_html: '',
                  footer_tagline: '',
                  footer_copyright: '',
                  footer_columns: [],
                  social_links: []
                },
                style: {
                  bg_type: el.kind === 'Button' || el.kind === 'Badge' || el.kind === 'Card' ? 'Solid' : 'None',
                  bg_color: el.kind === 'Button' ? t.primary : t.surfaceAlt,
                  bg_gradient_end: t.surface,
                  bg_gradient_angle: 135,
                  text_color: el.kind === 'Button' ? t.primaryText : t.text,
                  font_family: '',
                  font_size: el.fontSize || 16,
                  font_weight: el.kind === 'Heading' ? 700 : 400,
                  line_height: 1.4,
                  letter_spacing: 0,
                  text_align: 'Left',
                  padding: [el.pad || 0, el.pad || 0, el.pad || 0, el.pad || 0],
                  border_width: el.kind === 'Card' || el.kind === 'Badge' ? 1 : 0,
                  border_style: 'Solid',
                  border_color: t.border,
                  border_radius: [el.radius || 0, el.radius || 0, el.radius || 0, el.radius || 0],
                  opacity: 1,
                  shadow: 'None'
                },
                hover: 'None',
                animation: 'None',
                custom_class: '',
                custom_css: '',
                html_id: ''
              };
            })
          };
        })
      };
    });

    var envelope = {
      format: 'weysko-project',
      format_version: 1,
      app_version: '1.3.2',
      exported_at_unix: Math.floor(Date.now() / 1000),
      project: {
        schema_version: 1,
        name: state.project.name,
        base_url: state.project.base_url || 'https://example.com',
        favicon_asset_id: null,
        theme: {
          preset: state.project.themeKey,
          palette: {
            background: t.bg,
            surface: t.surface,
            surface_alt: t.surfaceAlt,
            primary: t.primary,
            primary_text: t.primaryText,
            accent: '#CBD5E1',
            text: t.text,
            text_muted: t.muted,
            border: t.border
          },
          typography: {
            heading_font: t.font,
            body_font: 'Inter',
            mono_font: 'JetBrains Mono',
            use_google_fonts: true,
            base_font_size: 16,
            line_height: 1.6,
            heading_weight: 700,
            letter_spacing: -0.02
          },
          content_max_width: 1200,
          border_radius: t.radius,
          smooth_scroll: true,
          custom_global_css: ''
        },
        assets: [],
        pages: desktopPages
      }
    };

    var blob = new Blob([JSON.stringify(envelope, null, 2)], { type: 'application/json' });
    var url = URL.createObjectURL(blob);
    var a = document.createElement('a');
    a.href = url;
    a.download = slugName + '.weysko';
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
    notify('Exported ' + slugName + '.weysko (compatible with Weysko Desktop)');
  }

  // Wire toolbar & Home controls
  document.getElementById('demo-start-scratch-btn').addEventListener('click', function () {
    openTemplate('blank');
  });
  document.getElementById('demo-open-meridian-btn').addEventListener('click', function () {
    openTemplate('meridian');
  });
  if (resumeBtn) {
    resumeBtn.addEventListener('click', enterEditor);
  }
  document.getElementById('demo-go-home-btn').addEventListener('click', enterHome);

  document.getElementById('demo-import-input').addEventListener('change', function (e) {
    var file = e.target.files && e.target.files[0];
    if (!file) return;
    var reader = new FileReader();
    reader.onload = function () {
      try {
        var parsed = JSON.parse(String(reader.result));
        var proj = parsed.format === 'weysko-project' && parsed.project ? parsed.project : parsed;
        if (!proj.pages || !proj.pages.length) throw new Error('Missing pages');
        var themeKey = (proj.theme && proj.theme.preset && THEMES[proj.theme.preset]) ? proj.theme.preset : 'ObsidianSlate';
        state.selectedTheme = themeKey;
        state.project = {
          name: proj.name || file.name,
          base_url: proj.base_url || 'https://example.com',
          themeKey: themeKey,
          pages: proj.pages.map(function (p) {
            return {
              id: p.id || makeId(),
              title: p.title || 'Page',
              slug: p.slug || 'index',
              sections: (p.sections || []).map(function (s, idx) {
                return {
                  id: s.id || makeId(),
                  name: s.name || ('Section ' + (idx + 1)),
                  altSurface: idx % 2 === 1,
                  elements: (s.elements || []).map(function (el) {
                    var c = el.content || {};
                    var st = el.style || {};
                    return {
                      id: el.id || makeId(),
                      kind: el.kind === 'Card' ? 'Card' : (['Heading', 'Paragraph', 'Button', 'Badge', 'Divider'].indexOf(el.kind) >= 0 ? el.kind : 'Paragraph'),
                      title: c.card_title || 'Feature Card',
                      text: c.card_body || c.text || el.name || 'Element',
                      fontSize: st.font_size || 16,
                      radius: (st.border_radius && st.border_radius[0]) || 6,
                      pad: (st.padding && st.padding[0]) || 8
                    };
                  })
                };
              })
            };
          })
        };
        state.activePageIndex = 0;
        state.selectedElementId = null;
        enterEditor();
        notify('Imported ' + file.name);
      } catch (err) {
        notify('Could not parse .weysko file: ' + err.message);
      }
    };
    reader.readAsText(file);
  });

  document.getElementById('demo-site-name').addEventListener('input', function (e) {
    if (state.project) state.project.name = e.target.value;
  });

  document.getElementById('demo-page-select').addEventListener('change', function (e) {
    state.activePageIndex = Number(e.target.value) || 0;
    state.selectedElementId = null;
    renderEditorAll();
  });

  document.querySelectorAll('.demo-vp-btn').forEach(function (btn) {
    btn.addEventListener('click', function () {
      document.querySelectorAll('.demo-vp-btn').forEach(function (x) {
        x.classList.remove('is-active');
      });
      btn.classList.add('is-active');
      state.viewport = btn.getAttribute('data-vp') || 'desktop';
      renderArtboard();
      renderStatusBar();
    });
  });

  document.getElementById('demo-grid-toggle').addEventListener('click', function (e) {
    state.showGrid = !state.showGrid;
    e.currentTarget.classList.toggle('is-active', state.showGrid);
    document.getElementById('demo-canvas-wrap').classList.toggle('is-grid', state.showGrid);
  });

  document.querySelectorAll('.demo-add-btn').forEach(function (btn) {
    btn.addEventListener('click', function () {
      var page = currentPage();
      if (!page || !page.sections.length) return;
      var kindKey = btn.getAttribute('data-add');
      var t = THEMES[state.project.themeKey] || THEMES.ObsidianSlate;
      var map = {
        heading: { kind: 'Heading', text: 'New Section Heading', fontSize: 30, radius: 4, pad: 4 },
        paragraph: { kind: 'Paragraph', text: 'Deterministic copy generated cleanly without external AI.', fontSize: 16, radius: 4, pad: 4 },
        button: { kind: 'Button', text: 'Action CTA', fontSize: 14, radius: t.radius, pad: 12 },
        badge: { kind: 'Badge', text: 'NEW RELEASE', fontSize: 12, radius: 20, pad: 8 },
        card: { kind: 'Card', title: 'Modular Block', text: 'Customize typography, radius, and padding in the Inspector.', fontSize: 15, radius: t.radius, pad: 16 },
        divider: { kind: 'Divider', text: '', fontSize: 14, radius: 0, pad: 0 }
      };
      var tpl = map[kindKey] || map.paragraph;
      var newEl = JSON.parse(JSON.stringify(tpl));
      newEl.id = makeId();
      page.sections[0].elements.push(newEl);
      state.selectedElementId = newEl.id;
      renderEditorAll();
      notify('Added ' + newEl.kind);
    });
  });

  document.getElementById('demo-export-weysko-btn').addEventListener('click', exportWeyskoFile);

  renderHomeStyles();
  renderHomeTemplates();
})();
