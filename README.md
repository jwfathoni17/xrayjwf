# Xray on Cloudflare Workers (English Documentation).

Xray is a lightweight, serverless V2Ray tunnel built on Cloudflare Workers, optimized for the Indonesian network environment. It supports modern mainstream proxy protocols and offers fast, secure, and highly scalable deployment without the need for a traditional VPS.

🌐 **Web UI Demo:** [[https://link-565.pages.dev/](https://link-565.pages.dev/)]
*(Feel free to customize the Web UI according to your needs)*

---

## 🌐 Language / 语言
* **English Version** | [简体中文](./README_CN.md)

---

## 🔧 Features
- ✅ **Multi-Protocol Support**: VMess, VLESS, Trojan, Shadowsocks (SS)
- ✅ **DNS over HTTPS (DoH)**: Encrypts DNS queries to enhance privacy and security.

## 🌐 Endpoints
| Endpoint | Description |
| -------- | --------------------------------- |
| `/`      | Static landing page (Prevents Cloudflare Error 1101) |
| `/link`  | Generate shareable proxy links for VMess, VLESS, Trojan & SS |
| `/sub`   | Static subscription text (Prevents Cloudflare Error 1101)|

---

## 1️⃣ Cloudflare Account Setup
1. Ensure you have a [Cloudflare](https://cloudflare.com) account (Free).  
2. Note your **Account ID**:  
   - Log in to the [Cloudflare Dashboard](https://dash.cloudflare.com/).  
   - On the dashboard overview page, scroll down to the bottom right to find your **Account ID**.  
   - Copy and save this ID. It will be used later in `wrangler.toml` and GitHub Secrets.

---

## 🚀 Deployment Guide (GitHub Actions)

### 2️⃣ Create KV Namespace
This Worker requires Cloudflare KV to store data.
1. In the Cloudflare Dashboard, navigate to **Workers & Pages** → **KV**.  
2. Click **Create namespace** and fill in the name: `YUMI`.  
3. Once created, click on the namespace name and copy its **ID** (e.g., `8ff1913fa986482d967e0a5c7e2b0791`). Save this ID for later use.

### 3️⃣ Create Cloudflare API Token
This token authorizes GitHub Actions deployment scripts to access your Cloudflare account.
1. Go to the [Cloudflare API Tokens page](https://dash.cloudflare.com/profile/api-tokens).  
2. Click **Create Token** → select the **Edit Cloudflare Workers** template.  
3. Click **Continue to summary** → **Create Token**.  

### 4️⃣ GitHub Repository Configuration
1. **Add Secret**: Navigate to GitHub → Your project repository → **Settings** → **Secrets and variables** → **Actions**.
2. Add a new Repository Secret:
   - **Name**: `CLOUDFLARE_API_TOKEN`
   - **Value**: Paste the Cloudflare API token you just generated.
3. **Fork/Clone Repository**: Fork or Clone this repository to your own GitHub account.
4. **Modify `wrangler.toml`**: Open the `wrangler.toml` file in your GitHub repository and update the following configuration fields:

```toml
name = "xray"
main = "build/worker/shim.mjs"
compatibility_date = "2024-05-23"
minify = true

# Replace with your Account ID obtained from Step 1
account_id = "ea1aab81dc8e22018131832d124a54fc"
workers_dev = true

[[kv_namespaces]]
binding = "YUMI"
# Replace with your KV Namespace ID obtained from Step 2
id = "8ff1913fa986482d967e0a5c7e2b0791"

[build]
command = "cargo install worker-build && worker-build --release"

[env.dev]
build = { command = "cargo install worker-build && worker-build --dev" }

[vars]
UUID = "2bcfbfba-b446-4ad5-93ad-72af9e008f61"   # You can customize and change this to your own UUID
