use super::layout::render_page;

pub fn render_contact() -> String {
    let title = "Contact Taskee - Developer Channels & Community Hub";
    let description = "Direct contact channels, GitHub issue trackers, and community hubs for the Taskee desktop system resource monitor project.";

    let content = r##"
    <section class="hero" style="padding: 4rem 0 2rem;">
      <div class="container">
        <div style="max-width: 820px;">
          <span class="hud-tag hud-tag-accent">DEVELOPER COMMUNICATION DIRECTORY</span>
          <h1 style="margin-top: 1rem; margin-bottom: 1.25rem;">Connect with the Taskee Team</h1>
          <p style="font-size: 1.15rem; color: var(--text-muted); line-height: 1.8;">
            Have questions, feedback, or telemetry issues? Reach the core maintainers directly through our dedicated open-source channels, issue trackers, and developer forums.
          </p>
        </div>
      </div>
    </section>

    <!-- Communication Cards Grid (NO FORM) -->
    <section style="padding: 3rem 0; border-top: 1px solid var(--border-subtle);">
      <div class="container">
        <div class="contact-grid">
          <!-- Card 1: GitHub Issues -->
          <div class="contact-card">
            <div class="contact-card-top">
              <span class="contact-channel-badge" style="background: rgba(37, 99, 235, 0.1); color: #60A5FA; border: 1px solid rgba(37, 99, 235, 0.3);">
                BUG TRACKER
              </span>
              <h3 style="color: #FFFFFF; font-size: 1.35rem;">GitHub Issue Tracker</h3>
              <p style="font-size: 0.92rem;">
                Found a bug with calibrated CPU deltas, core detection, or high process overhead? Open an issue on GitHub with your reproduction steps and system information.
              </p>
            </div>
            <div class="contact-card-footer">
              <a href="https://github.com/alrifatsabbir/taskee/issues" target="_blank" rel="noopener" class="btn btn-secondary" style="width: 100%; justify-content: space-between;">
                <span>OPEN AN ISSUE</span>
                <span>&rarr;</span>
              </a>
            </div>
          </div>

          <!-- Card 2: GitHub Discussions -->
          <div class="contact-card">
            <div class="contact-card-top">
              <span class="contact-channel-badge" style="background: rgba(16, 185, 129, 0.1); color: #34D399; border: 1px solid rgba(16, 185, 129, 0.3);">
                COMMUNITY RFCs
              </span>
              <h3 style="color: #FFFFFF; font-size: 1.35rem;">GitHub Discussions</h3>
              <p style="font-size: 0.92rem;">
                Discuss new HUD instrumentation ideas, suggest UI themes, submit architectural RFCs, or share your custom workstation setups with fellow engineers.
              </p>
            </div>
            <div class="contact-card-footer">
              <a href="https://github.com/alrifatsabbir/taskee/discussions" target="_blank" rel="noopener" class="btn btn-secondary" style="width: 100%; justify-content: space-between;">
                <span>JOIN DISCUSSIONS</span>
                <span>&rarr;</span>
              </a>
            </div>
          </div>

          <!-- Card 3: Security & Vulnerability -->
          <div class="contact-card">
            <div class="contact-card-top">
              <span class="contact-channel-badge" style="background: rgba(239, 68, 68, 0.1); color: #F87171; border: 1px solid rgba(239, 68, 68, 0.3);">
                RESPONSIBLE DISCLOSURE
              </span>
              <h3 style="color: #FFFFFF; font-size: 1.35rem;">Security &amp; Vulnerabilities</h3>
              <p style="font-size: 0.92rem;">
                For sensitive security reports regarding process privilege elevation or kernel communication, please contact our core security inbox.
              </p>
            </div>
            <div class="contact-card-footer">
              <div class="terminal-box" style="margin: 0; padding: 0.75rem;">
                <div style="font-family: var(--font-mono); font-size: 0.78rem; color: #E2E8F0; display: flex; justify-content: space-between; align-items: center;">
                  <span id="secEmailText">security@taskee.dev</span>
                  <button class="btn-copy" data-copy-target="secEmailText">COPY</button>
                </div>
              </div>
            </div>
          </div>

          <!-- Card 4: Maintainer Email -->
          <div class="contact-card">
            <div class="contact-card-top">
              <span class="contact-channel-badge" style="background: rgba(139, 92, 246, 0.1); color: #C084FC; border: 1px solid rgba(139, 92, 246, 0.3);">
                DIRECT INQUIRIES
              </span>
              <h3 style="color: #FFFFFF; font-size: 1.35rem;">Maintainer Direct</h3>
              <p style="font-size: 0.92rem;">
                For project sponsorships, licensing inquiries, or enterprise workstation packaging questions, reach the maintainer team directly.
              </p>
            </div>
            <div class="contact-card-footer">
              <div class="terminal-box" style="margin: 0; padding: 0.75rem;">
                <div style="font-family: var(--font-mono); font-size: 0.78rem; color: #E2E8F0; display: flex; justify-content: space-between; align-items: center;">
                  <span id="maintEmailText">maintainers@taskee.dev</span>
                  <button class="btn-copy" data-copy-target="maintEmailText">COPY</button>
                </div>
              </div>
            </div>
          </div>

          <!-- Card 5: Real-Time Chat (Discord & Matrix) -->
          <div class="contact-card">
            <div class="contact-card-top">
              <span class="contact-channel-badge" style="background: rgba(20, 184, 166, 0.1); color: #2DD4BF; border: 1px solid rgba(20, 184, 166, 0.3);">
                REAL-TIME CHAT
              </span>
              <h3 style="color: #FFFFFF; font-size: 1.35rem;">Matrix &amp; Discord</h3>
              <p style="font-size: 0.92rem;">
                Chat with other Rust developers, share egui styling tips, or debug cross-platform graphics issues in our matrix channel <code>#taskee:matrix.org</code>.
              </p>
            </div>
            <div class="contact-card-footer">
              <div style="font-family: var(--font-mono); font-size: 0.78rem; color: var(--text-dim);">
                ROOM: <span style="color: var(--text-main);">#taskee:matrix.org</span>
              </div>
            </div>
          </div>

          <!-- Card 6: Zero Telemetry Privacy Promise -->
          <div class="contact-card">
            <div class="contact-card-top">
              <span class="contact-channel-badge" style="background: rgba(245, 158, 11, 0.1); color: #FBBF24; border: 1px solid rgba(245, 158, 11, 0.3);">
                PRIVACY GUARANTEE
              </span>
              <h3 style="color: #FFFFFF; font-size: 1.35rem;">Zero Analytics &amp; Tracking</h3>
              <p style="font-size: 0.92rem;">
                Taskee does <strong>NOT</strong> collect analytics, telemetry, or network packets. All hardware stats remain strictly inside your local device RAM.
              </p>
            </div>
            <div class="contact-card-footer">
              <span class="hud-tag hud-tag-emerald">100% LOCAL PROCESSING</span>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- FAQ Section -->
    <section style="background: rgba(18, 19, 22, 0.4); border-top: 1px solid var(--border-subtle); padding: 4rem 0;">
      <div class="container">
        <div class="section-header" style="text-align: left; max-width: 100%; margin-bottom: 2.5rem;">
          <span class="hud-tag">FAQ</span>
          <h2>Frequently Asked Questions</h2>
        </div>

        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 1.5rem;">
          <div class="feature-card">
            <h3>How do I contribute code?</h3>
            <p style="font-size: 0.9rem; margin-top: 0.5rem;">
              Fork the repository, create a feature branch, verify your code compiles with <code>cargo check</code>, and open a Pull Request. Please follow standard <code>cargo fmt</code> and <code>cargo clippy</code> conventions.
            </p>
          </div>

          <div class="feature-card">
            <h3>Does Taskee require administrator or root rights?</h3>
            <p style="font-size: 0.9rem; margin-top: 0.5rem;">
              No. General CPU, RAM, disk, and personal process monitoring operates with standard user privileges. Terminating system-owned processes on Windows or Linux requires elevated privileges as enforced by the OS.
            </p>
          </div>

          <div class="feature-card">
            <h3>Is Taskee available for macOS?</h3>
            <p style="font-size: 0.9rem; margin-top: 0.5rem;">
              Yes! <code>sysinfo</code> and <code>eframe</code> support macOS natively. You can clone the repo and compile with <code>cargo run --release</code> on Apple Silicon and Intel Macs.
            </p>
          </div>
        </div>
      </div>
    </section>
    "##;

    render_page(title, description, "contact", content)
}
