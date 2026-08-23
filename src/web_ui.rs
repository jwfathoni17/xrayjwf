pub const WEB_UI: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport"
      content="width=device-width,initial-scale=1">

<title>XRAYJWF</title>

<style>
* {
    box-sizing: border-box;
}

body {
    margin: 0;
    background: #f5f5f5;
    color: #111;
    font-family:
        Inter,
        system-ui,
        -apple-system,
        sans-serif;
}

.container {
    width: min(100% - 24px, 620px);
    margin: auto;
    padding: 42px 0 60px;
}

.header {
    text-align: center;
    margin-bottom: 32px;
}

.logo {
    width: 44px;
    height: 44px;
    margin: auto;
    display: grid;
    place-items: center;
    border: 1px solid #111;
    border-radius: 12px;
    font-weight: 800;
}

.title {
    margin: 14px 0 5px;
    font-size: 22px;
}

.subtitle {
    color: #888;
    font-size: 12px;
}

.card {
    background: #fff;
    border: 1px solid #ddd;
    border-radius: 12px;
    padding: 16px;
    margin-bottom: 12px;
}

label {
    display: block;
    margin-bottom: 8px;
    font-size: 11px;
    font-weight: 700;
    color: #777;
    text-transform: uppercase;
}

select,
input {
    width: 100%;
    height: 44px;
    padding: 0 12px;
    border: 1px solid #d5d5d5;
    border-radius: 8px;
    background: #fff;
    outline: none;
}

input {
    margin-top: 10px;
    display: none;
}

button {
    width: 100%;
    height: 44px;
    border: 0;
    border-radius: 8px;
    background: #111;
    color: white;
    font-weight: 700;
    cursor: pointer;
}

.config {
    display: none;
}

.config.show {
    display: block;
}

.config-title {
    display: flex;
    justify-content: space-between;
    margin-bottom: 10px;
    font-size: 11px;
    font-weight: 700;
}

.copy {
    width: auto;
    height: 28px;
    padding: 0 10px;
    background: #eee;
    color: #111;
    font-size: 10px;
}

pre {
    margin: 0;
    padding: 12px;
    overflow-x: auto;
    background: #f7f7f7;
    border-radius: 8px;
    font-size: 11px;
    line-height: 1.55;
    white-space: pre-wrap;
    word-break: break-all;
}

.status {
    text-align: center;
    margin-top: 16px;
    color: #888;
    font-size: 11px;
}
</style>
</head>

<body>

<div class="container">

<div class="header">
    <div class="logo">X</div>
    <div class="title">XRAYJWF</div>
    <div class="subtitle">
        Minimal Configuration Panel
    </div>
</div>

<div class="card">

<label>WC</label>

<select id="wc">
    <option value="nonwc">NON WC</option>
    <option value="custom">CUSTOM</option>
</select>

<input
    id="custom"
    type="text"
    placeholder="Enter your hostname prefix"
>

</div>

<div class="card">

<button onclick="createAccount()">
    CREATE ACCOUNT
</button>

</div>

<div id="vlessCard"
     class="card config">

<div class="config-title">
    <span>VLESS</span>

    <button
        class="copy"
        onclick="copyText('vless')">
        COPY
    </button>
</div>

<pre id="vless"></pre>

</div>

<div id="clashCard"
     class="card config">

<div class="config-title">
    <span>CLASH</span>

    <button
        class="copy"
        onclick="copyText('clash')">
        COPY
    </button>
</div>

<pre id="clash"></pre>

</div>

<div id="status" class="status">
    Ready
</div>

</div>

<script>

const wc =
    document.getElementById("wc");

const custom =
    document.getElementById("custom");

wc.addEventListener("change", () => {

    custom.style.display =
        wc.value === "custom"
        ? "block"
        : "none";

});

async function createAccount() {

    let url =
        "/link?wc=" +
        encodeURIComponent(wc.value);

    if (wc.value === "custom") {

        const value =
            custom.value.trim();

        if (!value) {
            alert("Enter custom value");
            return;
        }

        url +=
            "&custom=" +
            encodeURIComponent(value);
    }

    const status =
        document.getElementById("status");

    status.textContent =
        "Generating...";

    try {

        const response =
            await fetch(url);

        if (!response.ok)
            throw new Error(
                "HTTP " + response.status
            );

        const text =
            await response.text();

        const parts =
            text.split("\n---CLASH---\n");

        document.getElementById("vless")
            .textContent = parts[0];

        document.getElementById("clash")
            .textContent = parts[1] || "";

        document
            .getElementById("vlessCard")
            .classList.add("show");

        document
            .getElementById("clashCard")
            .classList.add("show");

        status.textContent =
            "Account generated";

    } catch (error) {

        console.error(error);

        status.textContent =
            "Failed";

    }
}

async function copyText(id) {

    const text =
        document.getElementById(id)
            .textContent;

    await navigator.clipboard
        .writeText(text);

    document.getElementById("status")
        .textContent = "Copied";

}

</script>

</body>
</html>"#;
