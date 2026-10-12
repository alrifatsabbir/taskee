use super::layout::render_page;

pub fn render_not_found(path: &str) -> String {
    let title = "404 Not Found — Taskee";
    let description = "The requested telemetry endpoint could not be located.";

    let template = r##"
    <section class="hero" style="min-height: 65vh; display: flex; align-items: center;">
      <div class="container" style="text-align: center;">
        <div style="max-width: 580px; margin: 0 auto;">
          <span class="hud-tag hud-tag-accent" style="color: var(--accent-red); border-color: rgba(239, 68, 68, 0.4); background: rgba(239, 68, 68, 0.08);">
            SIGNAL DISRUPTED // 404 ERROR
          </span>
          <h1 style="margin: 1.25rem 0 0.75rem; font-size: 3rem;">Page Not Found</h1>
          <p style="margin-bottom: 2rem; font-size: 1.05rem;">
            The endpoint <code>__PATH__</code> does not exist on this Taskee telemetry host.
          </p>
          <div style="display: flex; gap: 1rem; justify-content: center;">
            <a href="/" class="btn btn-primary">RETURN TO HOME</a>
            <a href="/install" class="btn btn-secondary">INSTALLATION</a>
          </div>
        </div>
      </div>
    </section>
    "##;

    let content = template.replace("__PATH__", path);
    render_page(title, description, "404", &content)
}
