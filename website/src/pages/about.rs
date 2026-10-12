use super::layout::render_page;

pub fn render_about() -> String {
    let title = "About Taskee — System Architecture & Pure Rust Engineering";
    let description = "Explore the technical architecture, zero-webview philosophy, and immediate-mode GUI engine behind Taskee Desktop System Resource Monitor.";

    let content = r##"
    <section class="hero" style="padding: 4rem 0 2rem;">
      <div class="container">
        <div style="max-width: 820px;">
          <span class="hud-tag hud-tag-accent">SYSTEM PHILOSOPHY & ARCHITECTURE</span>
          <h1 style="margin-top: 1rem; margin-bottom: 1.25rem;">The Engineering Behind Taskee</h1>
          <p style="font-size: 1.15rem; color: var(--text-muted); line-height: 1.8;">
            Modern desktop utilities frequently ship bundled with an entire Chromium browser and Node.js runtime, consuming hundreds of megabytes of RAM just to monitor RAM. <strong>Taskee</strong> was built to restore sanity: 100% native Rust, sub-millisecond execution, and brutalist high-density telemetry.
          </p>
        </div>
      </div>
    </section>

    <!-- Core Philosophy Section -->
    <section style="padding: 3rem 0; border-top: 1px solid var(--border-subtle);">
      <div class="container">
        <div class="section-header" style="text-align: left; margin-bottom: 2.5rem; max-width: 100%;">
          <span class="hud-tag hud-tag-emerald">CORE TENETS</span>
          <h2>Three Foundational Principles</h2>
        </div>

        <div class="features-grid">
          <div class="feature-card">
            <div class="feature-icon-wrap" style="color: var(--status-ok);">01</div>
            <h3>Zero Webview, Zero Runtime</h3>
            <p>
              Taskee contains zero JavaScript, zero Node.js, and zero HTML renderer in the desktop app. It compiles directly to native x86_64 machine code, starting up in under 150 milliseconds.
            </p>
            <div class="feature-badge-row">
              <span class="hud-tag">Pure Rust</span>
              <span class="hud-tag">&lt;15 MB RAM</span>
            </div>
          </div>

          <div class="feature-card">
            <div class="feature-icon-wrap" style="color: var(--accent-blue);">02</div>
            <h3>Immediate-Mode Rendering</h3>
            <p>
              Built using <code>eframe</code> and <code>egui</code>. Unlike retained-mode frameworks that construct heavy DOM node trees, Taskee's interface is freshly rendered on the GPU each frame with zero GC pauses.
            </p>
            <div class="feature-badge-row">
              <span class="hud-tag">egui 0.36</span>
              <span class="hud-tag">Zero Memory Leaks</span>
            </div>
          </div>

          <div class="feature-card">
            <div class="feature-icon-wrap" style="color: var(--accent-purple);">03</div>
            <h3>Brutalist ArtCraft HUD</h3>
            <p>
              Hardware telemetry should be legible, dense, and uncompromising. Taskee uses razor-thin borders, monospace alignment, and an ultra-dark palette engineered for developer workstations.
            </p>
            <div class="feature-badge-row">
              <span class="hud-tag">Monospace HUD</span>
              <span class="hud-tag">OLED Pitch Black</span>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Architecture Pipeline Diagram -->
    <section style="background: rgba(18, 19, 22, 0.4); border-top: 1px solid var(--border-subtle); padding: 4rem 0;">
      <div class="container">
        <div class="section-header" style="text-align: left; margin-bottom: 2.5rem; max-width: 100%;">
          <span class="hud-tag hud-tag-accent">DATA PIPELINE</span>
          <h2>Sub-Millisecond Telemetry Pipeline</h2>
          <p>
            How hardware signals transition from OS kernel APIs to immediate-mode GPU drawing in 1-second calibrated loops.
          </p>
        </div>

        <div class="architecture-diagram">
          <div class="arch-flow">
            <div class="arch-node">
              <span class="hud-tag" style="font-size: 0.65rem; color: #60A5FA;">STAGE 01</span>
              <div class="arch-node-title">Kernel Instrumentation</div>
              <div class="arch-node-desc">
                Queries Windows <code>NtQuerySystemInformation</code> or Linux <code>/proc</code> via <code>sysinfo 0.39</code>.
              </div>
            </div>

            <div class="arch-node">
              <span class="hud-tag" style="font-size: 0.65rem; color: #34D399;">STAGE 02</span>
              <div class="arch-node-title">Delta Calibration</div>
              <div class="arch-node-desc">
                <code>monitor.rs</code> primes measurement windows to eliminate the 0% initial CPU delta flaw.
              </div>
            </div>

            <div class="arch-node">
              <span class="hud-tag" style="font-size: 0.65rem; color: #FBBF24;">STAGE 03</span>
              <div class="arch-node-title">Ring Buffer Aggregator</div>
              <div class="arch-node-desc">
                Maintains rolling 60-second time-series vectors for CPU load, RAM saturation, and throughput.
              </div>
            </div>

            <div class="arch-node">
              <span class="hud-tag" style="font-size: 0.65rem; color: #A78BFA;">STAGE 04</span>
              <div class="arch-node-title">GPU Immediate Rendering</div>
              <div class="arch-node-desc">
                <code>eframe</code> paints HUD telemetry, time-series curves, and process tables at 60 FPS.
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Modular Breakdown -->
    <section style="padding: 4rem 0; border-top: 1px solid var(--border-subtle);">
      <div class="container">
        <div class="section-header" style="text-align: left; margin-bottom: 2.5rem; max-width: 100%;">
          <span class="hud-tag">CRATE STRUCTURE</span>
          <h2>Source Architecture Breakdown</h2>
          <p>
            Clean, modular design adhering to strict idiomatic Rust practices.
          </p>
        </div>

        <div class="terminal-box" style="margin-bottom: 2.5rem;">
          <div class="terminal-header">
            <span class="terminal-title">TASKEE SOURCE TREE (CARGO WORKSPACE)</span>
            <span class="hud-tag" style="font-size: 0.65rem;">RUST EDITION 2024</span>
          </div>
          <pre class="code-block"><code>taskee/
├── Cargo.toml               <span class="code-comment"># Workspace manifest</span>
├── README.md                <span class="code-comment"># Technical documentation</span>
├── assets/
│   └── logo.jpg             <span class="code-comment"># Cyber HUD brand identity</span>
└── src/
    ├── main.rs              <span class="code-comment"># Native runtime setup & ArtCraft styling</span>
    ├── app.rs               <span class="code-comment"># HUD navigation tabs & 1-second update loop</span>
    ├── monitor.rs           <span class="code-comment"># Calibrated sysinfo telemetry engine</span>
    └── ui/
        ├── mod.rs           <span class="code-comment"># UI module definitions</span>
        ├── theme.rs         <span class="code-comment"># ArtCraft ultra-dark color tokens</span>
        ├── overview.rs      <span class="code-comment"># Workstation specs & 4-card HUD</span>
        ├── performance.rs   <span class="code-comment"># Live egui_plot charts & per-core grid</span>
        ├── processes.rs     <span class="code-comment"># Interactive process table & SIGKILL</span>
        └── storage_net.rs   <span class="code-comment"># Disk partitions & network throughput</span></code></pre>
        </div>

        <div class="features-grid">
          <div class="feature-card">
            <h3>monitor.rs — Telemetry Core</h3>
            <p>
              Handles multi-threaded hardware polling. Reads CPU frequencies, individual logical core percentages, total/available physical memory, swap space, disk mount points, and network packet rates.
            </p>
          </div>

          <div class="feature-card">
            <h3>performance.rs — Time Plots</h3>
            <p>
              Leverages <code>egui_plot</code> to draw rolling 60-second curves. Features dynamic bounds, dual plot legends, and the 16-thread visual utilization matrix with auto-scaling.
            </p>
          </div>

          <div class="feature-card">
            <h3>processes.rs — Process Dispatcher</h3>
            <p>
              Implements workstation process table sorting by CPU, Memory, PID, Name, or Runtime. Includes instant keyword substring search and safe process termination signals.
            </p>
          </div>
        </div>
      </div>
    </section>

    <!-- Bottom Navigation CTA -->
    <section style="border-top: 1px solid var(--border-subtle); padding: 3.5rem 0; text-align: center;">
      <div class="container">
        <h2>Ready to Experience Taskee?</h2>
        <p style="margin: 0.75rem 0 1.75rem;">Download the standalone executable or build it directly with Cargo.</p>
        <div style="display: flex; gap: 1rem; justify-content: center; flex-wrap: wrap;">
          <a href="/install" class="btn btn-primary">INSTALLATION INSTRUCTIONS</a>
          <a href="/contact" class="btn btn-secondary">CONTACT DEVELOPERS</a>
        </div>
      </div>
    </section>
    "##;

    render_page(title, description, "about", content)
}
