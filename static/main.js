document.addEventListener("DOMContentLoaded", () => {
  fetch("/year")
    .then((res) => res.text())
    .then((y) => {
      const el = document.getElementById("year");
      if (el) el.textContent = y;
    });
  initCounters();
  initParticles();
});

function initCounters() {
  const counters = document.querySelectorAll(".number");
  const options = { threshold: 0.6 };
  const observer = new IntersectionObserver(onEntry, options);
  counters.forEach((c) => observer.observe(c));

  function onEntry(entries, obs) {
    entries.forEach((entry) => {
      if (entry.isIntersecting) {
        const el = entry.target;
        const target = +el.getAttribute("data-target");
        let count = 0;
        const step = Math.ceil(target / 100);
        const timer = setInterval(() => {
          count += step;
          if (count >= target) {
            el.textContent = target;
            clearInterval(timer);
          } else {
            el.textContent = count;
          }
        }, 20);
        obs.unobserve(el);
      }
    });
  }
}

function initParticles() {
  const canvas = document.getElementById("particles-canvas");
  if (!canvas) return;
  const ctx = canvas.getContext("2d");
  let width = (canvas.width = window.innerWidth);
  let height = (canvas.height = window.innerHeight);

  const particles = [];
  const numParticles = 120;
  for (let i = 0; i < numParticles; i++) {
    particles.push({
      x: Math.random() * width,
      y: Math.random() * height,
      r: Math.random() * 1.2 + 0.3,
      speed: Math.random() * 0.3 + 0.1
    });
  }
  function update() {
    ctx.clearRect(0, 0, width, height);
    ctx.fillStyle = "rgba(255,255,255,0.6)";
    particles.forEach((p) => {
      ctx.beginPath();
      ctx.arc(p.x, p.y, p.r, 0, Math.PI * 2);
      ctx.fill();

      // movimiento suave hacia abajo y algo de deriva horizontal
      p.y += p.speed;
      p.x += Math.random() * 0.4 - 0.2;

      // reiniciar cuando salga del viewport
      if (p.y > height + p.r) {
        p.y = -p.r;
        p.x = Math.random() * width;
      }
      if (p.x > width) p.x = 0;
      if (p.x < 0) p.x = width;
    });
    requestAnimationFrame(update);
  }

  update();

  window.addEventListener("resize", () => {
    width = canvas.width = window.innerWidth;
    height = canvas.height = window.innerHeight;
  });
}