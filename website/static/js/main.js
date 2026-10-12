// Taskee Web HUD & Interactivity Scripts

document.addEventListener('DOMContentLoaded', () => {
  initMobileNav();
  initCopyButtons();
  initTabs();
  initHudSimulator();
});

// Mobile Navigation Toggle
function initMobileNav() {
  const toggleBtn = document.getElementById('mobileNavToggle');
  const navMenu = document.getElementById('navMenu');
  
  if (toggleBtn && navMenu) {
    toggleBtn.addEventListener('click', () => {
      navMenu.classList.toggle('open');
      const isOpen = navMenu.classList.contains('open');
      toggleBtn.setAttribute('aria-expanded', isOpen);
    });
  }
}

// Copy to Clipboard
function initCopyButtons() {
  const copyButtons = document.querySelectorAll('.btn-copy');
  
  copyButtons.forEach(btn => {
    btn.addEventListener('click', async () => {
      const targetId = btn.getAttribute('data-copy-target');
      let textToCopy = '';
      
      if (targetId) {
        const targetEl = document.getElementById(targetId);
        if (targetEl) {
          textToCopy = targetEl.innerText.trim();
        }
      } else {
        textToCopy = btn.getAttribute('data-copy-text') || '';
      }
      
      if (textToCopy) {
        try {
          await navigator.clipboard.writeText(textToCopy);
          const originalHtml = btn.innerHTML;
          btn.classList.add('copied');
          btn.innerHTML = `
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="20 6 9 17 4 12"></polyline>
            </svg>
            <span>COPIED</span>
          `;
          
          setTimeout(() => {
            btn.classList.remove('copied');
            btn.innerHTML = originalHtml;
          }, 2000);
        } catch (err) {
          console.error('Clipboard copy failed:', err);
        }
      }
    });
  });
}

// OS Switcher Tabs on Install Page
function initTabs() {
  const tabButtons = document.querySelectorAll('.tab-btn');
  const tabContents = document.querySelectorAll('.tab-content');
  
  if (!tabButtons.length) return;
  
  tabButtons.forEach(btn => {
    btn.addEventListener('click', () => {
      const targetTab = btn.getAttribute('data-tab');
      
      tabButtons.forEach(b => b.classList.remove('active'));
      tabContents.forEach(c => c.classList.remove('active'));
      
      btn.classList.add('active');
      const activeContent = document.getElementById(`tab-${targetTab}`);
      if (activeContent) {
        activeContent.classList.add('active');
      }
    });
  });
}

// Interactive Live Telemetry HUD Simulator
function initHudSimulator() {
  const canvas = document.getElementById('hudTelemetryCanvas');
  if (!canvas) return;
  
  const ctx = canvas.getContext('2d');
  const maxPoints = 50;
  
  // Data history
  const cpuHistory = Array.from({ length: maxPoints }, () => 18 + Math.random() * 12);
  const ramHistory = Array.from({ length: maxPoints }, () => 35 + Math.random() * 4);
  
  // Resize canvas according to device pixel ratio
  function resizeCanvas() {
    const rect = canvas.getBoundingClientRect();
    const dpr = window.devicePixelRatio || 1;
    canvas.width = rect.width * dpr;
    canvas.height = rect.height * dpr;
    ctx.scale(dpr, dpr);
  }
  
  resizeCanvas();
  window.addEventListener('resize', resizeCanvas);
  
  // HUD Elements
  const cpuDisplay = document.getElementById('hudCpuPercent');
  const cpuBar = document.getElementById('hudCpuFill');
  const ramDisplay = document.getElementById('hudRamPercent');
  const ramBar = document.getElementById('hudRamFill');
  const rxRate = document.getElementById('hudRxRate');
  const txRate = document.getElementById('hudTxRate');
  const uptimeEl = document.getElementById('hudUptime');
  
  let uptimeSec = 14285; // Initial mock uptime
  
  // Thread matrix bars
  const coreBars = [];
  const coreVals = [];
  for (let i = 0; i < 16; i++) {
    const bar = document.getElementById(`coreBar-${i}`);
    const val = document.getElementById(`coreVal-${i}`);
    if (bar && val) {
      coreBars.push(bar);
      coreVals.push(val);
    }
  }
  
  function updateTelemetryData() {
    // New CPU target with occasional burst
    const lastCpu = cpuHistory[cpuHistory.length - 1];
    let delta = (Math.random() - 0.48) * 12;
    if (Math.random() < 0.08) delta += 25; // occasional spike
    let nextCpu = Math.max(8, Math.min(94, lastCpu + delta));
    
    // Smooth RAM slow drift
    const lastRam = ramHistory[ramHistory.length - 1];
    let nextRam = Math.max(30, Math.min(48, lastRam + (Math.random() - 0.5) * 1.5));
    
    cpuHistory.shift();
    cpuHistory.push(nextCpu);
    
    ramHistory.shift();
    ramHistory.push(nextRam);
    
    // Update numerical displays
    if (cpuDisplay) cpuDisplay.textContent = `${nextCpu.toFixed(1)}%`;
    if (cpuBar) cpuBar.style.width = `${nextCpu}%`;
    if (ramDisplay) ramDisplay.textContent = `${nextRam.toFixed(1)}%`;
    if (ramBar) ramBar.style.width = `${nextRam}%`;
    
    // Update network rates
    if (rxRate) rxRate.textContent = `${(1.2 + Math.random() * 2.8).toFixed(2)} MB/s`;
    if (txRate) txRate.textContent = `${(120 + Math.random() * 450).toFixed(0)} KB/s`;
    
    // Uptime ticker
    uptimeSec += 1;
    if (uptimeEl) {
      const hrs = Math.floor(uptimeSec / 3600);
      const mins = Math.floor((uptimeSec % 3600) / 60);
      const secs = uptimeSec % 60;
      uptimeEl.textContent = `${hrs}h ${mins}m ${secs < 10 ? '0' : ''}${secs}s`;
    }
    
    // Update per-core load cells
    coreBars.forEach((bar, idx) => {
      const coreLoad = Math.max(4, Math.min(99, nextCpu + (Math.random() - 0.5) * 35));
      bar.style.height = `${coreLoad}%`;
      if (coreVals[idx]) {
        coreVals[idx].textContent = `${Math.round(coreLoad)}%`;
      }
      
      // Color-code heavy load
      if (coreLoad > 75) {
        bar.style.backgroundColor = '#EF4444';
      } else if (coreLoad > 50) {
        bar.style.backgroundColor = '#F59E0B';
      } else {
        bar.style.backgroundColor = '#2563EB';
      }
    });
  }
  
  function drawTelemetryWave() {
    const rect = canvas.getBoundingClientRect();
    const w = rect.width;
    const h = rect.height;
    
    ctx.clearRect(0, 0, w, h);
    
    // Background HUD grid lines
    ctx.strokeStyle = 'rgba(34, 37, 44, 0.45)';
    ctx.lineWidth = 1;
    
    for (let y = 0; y < h; y += 22) {
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(w, y);
      ctx.stroke();
    }
    
    for (let x = 0; x < w; x += 40) {
      ctx.beginPath();
      ctx.moveTo(x, 0);
      ctx.lineTo(x, h);
      ctx.stroke();
    }
    
    const step = w / (maxPoints - 1);
    
    // Draw RAM plot line (#10B981 emerald)
    drawCurve(ramHistory, '#10B981', 'rgba(16, 185, 129, 0.12)', w, h, step);
    
    // Draw CPU plot line (#2563EB indigo)
    drawCurve(cpuHistory, '#2563EB', 'rgba(37, 99, 235, 0.22)', w, h, step);
  }
  
  function drawCurve(data, strokeColor, fillColor, w, h, step) {
    ctx.beginPath();
    ctx.moveTo(0, h - (data[0] / 100) * (h - 10));
    
    for (let i = 1; i < data.length; i++) {
      const x = i * step;
      const y = h - (data[i] / 100) * (h - 10);
      ctx.lineTo(x, y);
    }
    
    ctx.strokeStyle = strokeColor;
    ctx.lineWidth = 2;
    ctx.stroke();
    
    // Gradient fill below curve
    ctx.lineTo(w, h);
    ctx.lineTo(0, h);
    ctx.closePath();
    ctx.fillStyle = fillColor;
    ctx.fill();
  }
  
  // Animation loop
  setInterval(() => {
    updateTelemetryData();
    drawTelemetryWave();
  }, 1000);
  
  // Initial draw
  drawTelemetryWave();
  
  // Interactive process kill simulation in preview table
  const killButtons = document.querySelectorAll('.badge-kill');
  killButtons.forEach(btn => {
    btn.addEventListener('click', (e) => {
      const row = e.target.closest('tr');
      const pid = btn.getAttribute('data-pid');
      const name = btn.getAttribute('data-name');
      
      btn.textContent = 'SIGKILL';
      btn.style.backgroundColor = '#EF4444';
      btn.style.color = '#FFFFFF';
      
      setTimeout(() => {
        if (row) {
          row.style.transition = 'opacity 0.4s ease, transform 0.4s ease';
          row.style.opacity = '0';
          row.style.transform = 'translateX(20px)';
          setTimeout(() => {
            row.style.opacity = '1';
            row.style.transform = 'none';
            btn.textContent = 'KILL';
            btn.style.backgroundColor = '';
            btn.style.color = '';
          }, 3500);
        }
      }, 300);
    });
  });
}
