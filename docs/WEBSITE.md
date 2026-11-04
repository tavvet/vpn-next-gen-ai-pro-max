# Marketing Website — Operations Manual

> **Safety note:** This is a satirical landing page for a nonfunctional VPN. It does not process payments, connect to networks, collect form submissions, or offer real security.

## Source files

The standalone static website lives under the [site directory](../site/):

- [index.html](../site/index.html) — responsive marketing page, with clear satire disclosures.
- [styles.css](../site/styles.css) — neon corporate design, no third-party fonts or external scripts.
- [app.js](../site/app.js) — harmless client-side buttons that render fictional terminal output.
- [favicon.svg](../site/favicon.svg) — zero-packet corporate logo.

**No build system or npm dependencies.** The page operates entirely client-side. Plan buttons do not create accounts or charges. Regional simulation buttons do not inspect your IP or geolocation and do not restrict access.

## Local preview

From the project root:

~~~sh
cd site
python3 -m http.server 8080
~~~

Open http://localhost:8080. This is a local development server, not a VPN.

## Publish via GitHub Pages

The GitHub Actions workflow [.github/workflows/deploy-site.yml](../.github/workflows/deploy-site.yml) publishes the static site folder.

1. In the GitHub repository, open **Settings → Pages**.
2. Choose **GitHub Actions** as the **Build and deployment → Source**.
3. Open **Actions → Publish marketing website (parody)** and select **Run workflow**, or push a change in site/.
4. Check the Pages deployment result and use the URL GitHub supplies.

Deployment requires Pages to be enabled and Actions to be permitted. These account settings cannot be configured with the current connector.

## Configure your own subdomain

You can use any subdomain you control, such as **vpn.example.com**.

Create a DNS record:

| Type | Host | Value |
| --- | --- | --- |
| CNAME | vpn | tavvet.github.io |

The host field is an example; replace vpn with whichever label you want. The destination is **tavvet.github.io** (no protocol, path or port).

Then open **GitHub → Repository Settings → Pages → Custom domain**, enter your actual full hostname, save, wait for verification, and enable **Enforce HTTPS** once the option becomes available.

**We do not yet create a CNAME file, because no specific hostname has been chosen.** The site does not depend on any particular domain.

## Marketing and security disclaimers

- This is a parody, not a functioning VPN.
- No subscriptions, payments, real regional restrictions, carrier partnerships, measured VPN performance, or actual security services.
- Fictional regional checks are a deterministic local string lookup, not geolocation.
- The only real features are the static website and text-only demo.

[← Documentation Center](README.md) · [Project README](../README.md)
