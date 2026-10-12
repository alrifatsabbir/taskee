pub fn render_page(title: &str, description: &str, active_route: &str, content: &str) -> String {
    let is_home = if active_route == "home" { "active" } else { "" };
    let is_about = if active_route == "about" { "active" } else { "" };
    let is_install = if active_route == "install" { "active" } else { "" };
    let is_contact = if active_route == "contact" { "active" } else { "" };

    let html = r##"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>__TITLE__</title>
  <meta name="description" content="__DESCRIPTION__">
  <meta name="theme-color" content="#0D0E10">
  <meta property="og:title" content="__TITLE__">
  <meta property="og:description" content="__DESCRIPTION__">
  <meta property="og:type" content="website">
  <meta property="og:image" content="/static/images/logo.jpg">
  <link rel="icon" type="image/svg+xml" href="/favicon.svg">
  <link rel="stylesheet" href="/static/css/style.css">
</head>
<body>
  <!-- Header / Navigation Bar -->
  <header class="navbar" id="siteHeader">
    <div class="container nav-container">
      <a href="/" class="brand-link" id="navBrandLink" aria-label="Taskee Home">
        <img src="/static/images/logo.jpg" alt="Taskee Cyber HUD Logo" class="brand-logo-img" id="navBrandLogo">
        <div class="brand-text-wrap">
          <div class="brand-title">TASKEE<span class="brand-accent">.RS</span></div>
          <div class="brand-subtitle">Workstation Telemetry</div>
        </div>
      </a>

      <!-- Desktop & Mobile Nav Menu -->
      <nav aria-label="Primary Navigation">
        <ul class="nav-menu" id="navMenu">
          <li><a href="/" class="nav-item-link __IS_HOME__" id="navLinkHome">HOME</a></li>
          <li><a href="/about" class="nav-item-link __IS_ABOUT__" id="navLinkAbout">ABOUT</a></li>
          <li><a href="/install" class="nav-item-link __IS_INSTALL__" id="navLinkInstall">INSTALL</a></li>
          <li><a href="/contact" class="nav-item-link __IS_CONTACT__" id="navLinkContact">CONTACT</a></li>
        </ul>
      </nav>

      <div class="nav-actions">
        <div class="nav-badge-status" id="navStatusBadge" title="Native Rust Engine Active">
          <span class="pulse-dot"></span>
          <span>100% RUST</span>
        </div>
        <a href="/install" class="btn btn-primary btn-sm" id="navCtaDownload">DOWNLOAD</a>
        <button class="mobile-toggle" id="mobileNavToggle" aria-label="Toggle navigation menu" aria-expanded="false">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="3" y1="12" x2="21" y2="12"></line>
            <line x1="3" y1="6" x2="21" y2="6"></line>
            <line x1="3" y1="18" x2="21" y2="18"></line>
          </svg>
        </button>
      </div>
    </div>
  </header>

  <!-- Main Page Content -->
  <main id="mainContent">
    __CONTENT__
  </main>

  <!-- Site Footer -->
  <footer id="siteFooter">
    <div class="container">
      <div class="footer-grid">
        <div class="footer-col">
          <div class="brand-title" style="margin-bottom: 0.5rem;">TASKEE<span class="brand-accent">.RS</span></div>
          <p style="font-size: 0.88rem; max-width: 320px;">
            Lightweight, high-density desktop system resource monitor and mini task manager engineered in pure Rust.
          </p>
          <div style="margin-top: 0.8rem; display: flex; gap: 0.5rem; flex-wrap: wrap;">
            <span class="hud-tag hud-tag-emerald">RUST 2024</span>
            <span class="hud-tag">ZERO WEBVIEW</span>
            <span class="hud-tag">ARTCRAFT HUD</span>
          </div>
        </div>

        <div class="footer-col">
          <h4>Navigation</h4>
          <ul>
            <li><a href="/" id="footerLinkHome">Home Overview</a></li>
            <li><a href="/about" id="footerLinkAbout">Architecture & Philosophy</a></li>
            <li><a href="/install" id="footerLinkInstall">Download & Installation</a></li>
            <li><a href="/contact" id="footerLinkContact">Developer Channels</a></li>
          </ul>
        </div>

        <div class="footer-col">
          <h4>Telemetry Stack</h4>
          <ul>
            <li><a href="https://github.com/emilk/egui" target="_blank" rel="noopener">egui 0.36</a></li>
            <li><a href="https://github.com/emilk/egui/tree/master/crates/eframe" target="_blank" rel="noopener">eframe Native</a></li>
            <li><a href="https://github.com/GuillaumeGomez/sysinfo" target="_blank" rel="noopener">sysinfo 0.39</a></li>
            <li><a href="https://github.com/tokio-rs/axum" target="_blank" rel="noopener">axum 0.7</a></li>
          </ul>
        </div>

        <div class="footer-col">
          <h4>Telemetry Health</h4>
          <div style="background: #0E1013; border: 1px solid var(--border-subtle); padding: 0.75rem; border-radius: 4px; font-family: var(--font-mono); font-size: 0.75rem;">
            <div style="color: var(--text-dim); margin-bottom: 0.25rem;">SERVER TELEMETRY</div>
            <div style="color: var(--status-ok); font-weight: 600;">STATUS: OPERATIONAL</div>
            <div style="color: var(--text-muted); margin-top: 0.25rem;">LATENCY: &lt;1ms (Rust SSR)</div>
          </div>
        </div>
      </div>

      <div class="footer-bottom">
        <div>&copy; 2026 Taskee Project. Engineered with 100% Pure Rust. Open-Source.</div>
        <div class="footer-bottom-links">
          <a href="/about">Philosophy</a>
          <a href="/install">Get Started</a>
          <a href="/contact">Support</a>
        </div>
      </div>
    </div>
  </footer>

  <script src="/static/js/main.js"></script>
</body>
</html>"##;

    html.replace("__TITLE__", title)
        .replace("__DESCRIPTION__", description)
        .replace("__IS_HOME__", is_home)
        .replace("__IS_ABOUT__", is_about)
        .replace("__IS_INSTALL__", is_install)
        .replace("__IS_CONTACT__", is_contact)
        .replace("__CONTENT__", content)
}
