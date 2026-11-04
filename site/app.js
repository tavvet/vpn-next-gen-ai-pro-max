/* VPN Next Gen AI Pro Max™ — completely local demo; no network, payments, or data collection. */
"use strict";
(() => {
  const year = document.querySelector("#year");
  if (year) year.textContent = String(new Date().getFullYear());

  const menuButton = document.querySelector(".menu-toggle");
  const menu = document.querySelector("#main-nav");
  if (menuButton && menu) {
    menuButton.addEventListener("click", () => {
      const next = menuButton.getAttribute("aria-expanded") !== "true";
      menuButton.setAttribute("aria-expanded", String(next));
      menu.classList.toggle("open", next);
    });
    menu.querySelectorAll("a").forEach(link => link.addEventListener("click", () => {
      menu.classList.remove("open");
      menuButton.setAttribute("aria-expanded", "false");
    }));
  }

  const output = document.querySelector("#terminal-output");
  const demo = document.querySelector("#demo");
  const simulations = {
    connect: [
      "[ NGT/0 :: CONNECTIVITY PLATFORM ]",
      "Initializing world-class VPN architecture........ OK",
      "AI-native branding module......................... OK",
      "Quantum-adjacent paperwork........................ OK",
      "Resolving non-existent server..................... SKIPPED",
      "",
      "TUNNEL STATUS ................ NOT ESTABLISHED",
      "ENCRYPTION ................... NOT ACTIVE",
      "PACKETS TRANSMITTED ......... 0",
      "NETWORK SETTINGS CHANGED .... NONE",
      "",
      "THIS IS NOT A VPN. Your internet connection remains unchanged."
    ],
    benchmark: [
      "[ ENTERPRISE PERFORMANCE REPORT ]",
      "Test suite ................................. Not executed",
      "Packets transmitted ........................ 0",
      "Packets received ........................... 0",
      "RTT (ms) ................................... UNDEFINED",
      "p99 latency (ms) ........................... UNDEFINED",
      "Loss rate .................................. UNDEFINED (0/0)",
      "Throughput (Mbps) .......................... N/A",
      "Zero-copy ................................. Zero packets copied",
      "",
      "Benchmark is fictional. No connection or test occurred."
    ],
    free: [
      "[ FREE TIER ACTIVATION ]",
      "Account setup .............................. NOT REQUIRED",
      "Credit card ................................ NOT REQUIRED",
      "Tunnel ..................................... NOT IMPLEMENTED",
      "Money charged .............................. $0",
      "",
      "Welcome aboard. This is a joke, not a functioning VPN."
    ],
    pro: [
      "[ PRO PLAN / PREVIEW ]",
      "Monthly price advertised.................... $19.99 (fictional)",
      "Payment processor .......................... NONE",
      "Subscription created ....................... NO",
      "Network tunnels ............................ 0",
      "",
      "Congratulations. Your optimism has been prioritized."
    ],
    max: [
      "[ AI PRO MAX™ ACTIVATION ]",
      "AI in product name .......................... CONFIRMED",
      "Additional networking code .................. NONE",
      "Global regulatory vibes ..................... OPTIMAL",
      "Your money collected ........................ $0",
      "",
      "Pro Max is a fictional marketing tier. No service activated."
    ],
    starman: [
      "[ INTERPLANETARY CONNECTIVITY™ ]",
      "Speed of light in vacuum .................... 299,792,458 m/s",
      "Network throughput to Tesla Roadster ........ N/A",
      "Established spacecraft links ................ 0",
      "Actual packets sent ......................... 0",
      "",
      "Physics is real. This space VPN is not."
    ]
  };

  function render(lines, name) {
    if (!output) return;
    output.replaceChildren();
    const command = document.createElement("p");
    const prompt = document.createElement("span");
    prompt.className = "prompt-symbol";
    prompt.textContent = "➜ ";
    const cmd = document.createElement("span");
    cmd.className = "prompt-command";
    cmd.textContent = "vpn-next-gen-ai-pro-max " + name;
    command.append(prompt, cmd);
    output.append(command);

    lines.forEach(line => {
      const p = document.createElement("p");
      p.textContent = line || "\u00a0";
      if (/NOT |NONE|UNDEFINED|N\/A|not a |NO$|0$|SKIPPED|fictional|joke|not\./i.test(line)) p.className = "terminal-amber";
      if (/OK$|CONFIRMED|OPTIMAL/.test(line)) p.className = "terminal-green";
      output.append(p);
    });
  }

  document.querySelectorAll("[data-demo-action]").forEach(button => {
    button.addEventListener("click", () => {
      const name = button.getAttribute("data-demo-action");
      if (!Object.prototype.hasOwnProperty.call(simulations, name)) return;
      render(simulations[name], name);
      if (demo) demo.scrollIntoView({behavior: window.matchMedia("(prefers-reduced-motion: reduce)").matches ? "auto" : "smooth"});
    });
  });

  const regionOutput = document.querySelector("#region-output");
  const regions = {
    uk: "🇬🇧 UK — FICTIONALLY BLOCKED: tea-based compliance review pending. No real IP detection or restriction.",
    au: "🇦🇺 AUSTRALIA — FICTIONALLY BLOCKED: kangaroo-grade paperwork incomplete. Nothing has been blocked.",
    other: "🌍 REST OF WORLD — FICTIONALLY APPROVED: the product still does not provide a VPN."
  };
  document.querySelectorAll("[data-region]").forEach(button => {
    button.addEventListener("click", () => {
      const region = button.getAttribute("data-region");
      if (!Object.prototype.hasOwnProperty.call(regions, region)) return;
      if (regionOutput) regionOutput.textContent = regions[region];
      document.querySelectorAll("[data-region]").forEach(b => b.setAttribute("aria-pressed", String(b === button)));
    });
  });
})();
