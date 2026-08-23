pub const WEB_UI: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1">

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
        BlinkMacSystemFont,
        "Segoe UI",
        sans-serif;
}

.container {
    width: min(100% - 24px, 620px);
    margin: auto;
    padding: 42px 0 60px;
}

.header {
    text-align: center;
    margin-bottom: 30px;
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

.section-title {
    margin-bottom: 13px;
    font-size: 11px;
    font-weight: 700;
    color: #777;
    text-transform: uppercase;
    letter-spacing: .06em;
}

/* SERVER */

.country {
    margin-bottom: 18px;
}

.country:last-child {
    margin-bottom: 0;
}

.country-name {
    margin-bottom: 8px;
    font-size: 12px;
    font-weight: 700;
}

.server {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    min-height: 42px;
    padding: 8px 10px;
    margin-bottom: 6px;

    background: #fafafa;
    border: 1px solid #e3e3e3;
    border-radius: 8px;

    color: #111;
    text-align: left;

    cursor: pointer;
}

.server:hover {
    border-color: #aaa;
}

.server.selected {
    border-color: #111;
    background: #f0f0f0;
}

.server-left {
    display: flex;
    align-items: center;
    gap: 9px;
    min-width: 0;
}

.radio {
    width: 13px;
    height: 13px;
    flex: 0 0 13px;

    border: 1px solid #aaa;
    border-radius: 50%;
}

.server.selected .radio {
    border: 4px solid #111;
}

.server-address {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;

    font-family:
        "SFMono-Regular",
        Consolas,
        monospace;

    font-size: 11px;
}

.ping {
    flex-shrink: 0;
    margin-left: 10px;

    font-family:
        "SFMono-Regular",
        Consolas,
        monospace;

    font-size: 10px;
    color: #888;
}

/* WC */

select,
input {
    width: 100%;
    height: 44px;
    padding: 0 12px;

    border: 1px solid #d5d5d5;
    border-radius: 8px;

    background: #fff;
    color: #111;

    outline: none;
}

input {
    display: none;
    margin-top: 10px;
}

/* BUTTON */

.create {
    width: 100%;
    height: 44px;

    border: 0;
    border-radius: 8px;

    background: #111;
    color: #fff;

    font-size: 12px;
    font-weight: 700;

    cursor: pointer;
}

.create:hover {
    opacity: .88;
}

.create:disabled {
    opacity: .5;
    cursor: not-allowed;
}

/* CONFIG */

.config {
    display: none;
}

.config.show {
    display: block;
}

.config-title {
    display: flex;
    align-items: center;
    justify-content: space-between;

    margin-bottom: 10px;

    font-size: 11px;
    font-weight: 700;
}

.copy {
    width: auto;
    height: 28px;

    padding: 0 10px;

    border: 0;
    border-radius: 6px;

    background: #eee;
    color: #111;

    font-size: 10px;
    font-weight: 700;

    cursor: pointer;
}

pre {
    margin: 0;
    padding: 12px;

    overflow-x: auto;

    background: #f7f7f7;
    border: 1px solid #e7e7e7;
    border-radius: 8px;

    font-family:
        "SFMono-Regular",
        Consolas,
        monospace;

    font-size: 10px;
    line-height: 1.6;

    white-space: pre-wrap;
    word-break: break-all;
}

.status {
    margin-top: 16px;

    color: #888;

    font-size: 11px;
    text-align: center;
}

.footer {
    margin-top: 28px;

    color: #aaa;

    font-size: 10px;
    text-align: center;
}
</style>
</head>

<body>

<div class="container">

    <div class="header">

        <div class="logo">
            X
        </div>

        <div class="title">
            XRAYJWF
        </div>

        <div class="subtitle">
            Minimal Configuration Panel
        </div>

    </div>


    <!-- SERVER -->

    <div class="card">

        <div class="section-title">
            Server
        </div>

        <div id="servers">
            Loading servers...
        </div>

    </div>


    <!-- WC -->

    <div class="card">

        <div class="section-title">
            WC
        </div>

        <select id="wc">

            <option value="nonwc">
                NON WC
            </option>

            <option value="custom">
                CUSTOM
            </option>

        </select>

        <input
            id="custom"
            type="text"
            placeholder="Enter WC"
            autocomplete="off"
        >

    </div>


    <!-- CREATE -->

    <div class="card">

        <button
            id="create"
            class="create"
            onclick="createAccount()"
            disabled
        >
            CREATE ACCOUNT
        </button>

    </div>


    <!-- VLESS -->

    <div
        id="vlessCard"
        class="card config"
    >

        <div class="config-title">

            <span>
                VLESS
            </span>

            <button
                class="copy"
                onclick="copyText('vless')"
            >
                COPY
            </button>

        </div>

        <pre id="vless"></pre>

    </div>


    <!-- CLASH -->

    <div
        id="clashCard"
        class="card config"
    >

        <div class="config-title">

            <span>
                CLASH
            </span>

            <button
                class="copy"
                onclick="copyText('clash')"
            >
                COPY
            </button>

        </div>

        <pre id="clash"></pre>

    </div>


    <div
        id="status"
        class="status"
    >
        Loading...
    </div>


    <div class="footer">
        Xray Worker
    </div>

</div>


<script>

let selectedServer = null;

let serverData = {};


const serversElement =
    document.getElementById("servers");

const createButton =
    document.getElementById("create");

const statusElement =
    document.getElementById("status");

const wcElement =
    document.getElementById("wc");

const customElement =
    document.getElementById("custom");


// ======================================
// LOAD SERVERS
// ======================================

async function loadServers() {

    try {

        const response =
            await fetch(
                "/api/servers",
                {
                    cache: "no-store"
                }
            );

        if (!response.ok) {
            throw new Error(
                "HTTP " + response.status
            );
        }

        serverData =
            await response.json();

        renderServers();

        statusElement.textContent =
            "Select a server";

    } catch (error) {

        console.error(error);

        serversElement.innerHTML =
            "<div style='font-size:11px;color:#888'>Unable to load servers</div>";

        statusElement.textContent =
            "Failed to load servers";

    }

}


// ======================================
// RENDER
// ======================================

function renderServers() {

    serversElement.innerHTML = "";

    for (
        const location in serverData
    ) {

        const servers =
            serverData[location];

        const country =
            document.createElement("div");

        country.className =
            "country";


        const title =
            document.createElement("div");

        title.className =
            "country-name";

        title.textContent =
            getCountryName(location);


        country.appendChild(title);


        servers.forEach(
            (server, index) => {

                const button =
                    document.createElement(
                        "button"
                    );

                button.className =
                    "server";


                const left =
                    document.createElement(
                        "div"
                    );

                left.className =
                    "server-left";


                const radio =
                    document.createElement(
                        "div"
                    );

                radio.className =
                    "radio";


                const address =
                    document.createElement(
                        "div"
                    );

                address.className =
                    "server-address";

                address.textContent =
                    server;


                left.appendChild(radio);

                left.appendChild(address);


                const ping =
                    document.createElement(
                        "div"
                    );

                ping.className =
                    "ping";

                ping.textContent =
                    "checking...";


                button.appendChild(left);

                button.appendChild(ping);


                button.onclick = () => {

                    selectServer(
                        location,
                        index,
                        button
                    );

                };


                country.appendChild(button);


                /*
                 * Ping dilakukan setelah
                 * elemen dibuat.
                 */

                checkPing(
                    server,
                    ping
                );

            }
        );


        serversElement.appendChild(
            country
        );

    }

}


// ======================================
// SELECT SERVER
// ======================================

function selectServer(
    location,
    index,
    element
) {

    document
        .querySelectorAll(".server")
        .forEach(
            item =>
                item.classList.remove(
                    "selected"
                )
        );

    element.classList.add(
        "selected"
    );


    selectedServer = {
        location: location,
        index: index + 1
    };


    createButton.disabled =
        false;


    statusElement.textContent =
        location +
        " / " +
        (index + 1) +
        " selected";

}


// ======================================
// PING
// ======================================

async function checkPing(
    server,
    element
) {

    /*
     * Endpoint Worker akan kita buat:
     *
     * /api/ping?server=IP:PORT
     *
     */

    try {

        const response =
            await fetch(
                "/api/ping?server=" +
                encodeURIComponent(
                    server
                ),
                {
                    cache: "no-store"
                }
            );

        if (!response.ok) {
            throw new Error();
        }

        const data =
            await response.json();


        if (
            typeof data.ms ===
            "number"
        ) {

            element.textContent =
                data.ms + " ms";

        } else {

            element.textContent =
                "offline";

        }

    } catch {

        /*
         * Endpoint ping belum tersedia
         * atau server tidak merespons.
         */

        element.textContent =
            "—";

    }

}


// ======================================
// WC
// ======================================

wcElement.addEventListener(
    "change",
    () => {

        customElement.style.display =
            wcElement.value === "custom"
            ? "block"
            : "none";

    }
);


// ======================================
// CREATE ACCOUNT
// ======================================

async function createAccount() {

    if (!selectedServer) {

        alert(
            "Please select a server first"
        );

        return;

    }


    createButton.disabled =
        true;

    createButton.textContent =
        "GENERATING...";

    statusElement.textContent =
        "Generating configuration...";


    try {

        let url =
            "/link?" +
            "location=" +
            encodeURIComponent(
                selectedServer.location
            ) +
            "&server=" +
            encodeURIComponent(
                selectedServer.index
            ) +
            "&wc=" +
            encodeURIComponent(
                wcElement.value
            );


        if (
            wcElement.value ===
            "custom"
        ) {

            const value =
                customElement.value.trim();


            if (!value) {

                alert(
                    "Enter custom WC"
                );

                return;

            }


            url +=
                "&custom=" +
                encodeURIComponent(
                    value
                );

        }


        const response =
            await fetch(
                url,
                {
                    cache: "no-store"
                }
            );


        if (!response.ok) {

            throw new Error(
                "HTTP " +
                response.status
            );

        }


        const text =
            await response.text();


        const parts =
            text.split(
                "\n---CLASH---\n"
            );


        document
            .getElementById("vless")
            .textContent =
                parts[0] || "";


        document
            .getElementById("clash")
            .textContent =
                parts[1] || "";


        document
            .getElementById("vlessCard")
            .classList.add("show");


        document
            .getElementById("clashCard")
            .classList.add("show");


        statusElement.textContent =
            "Configuration generated";


    } catch (error) {

        console.error(error);

        statusElement.textContent =
            "Failed to generate configuration";

    } finally {

        createButton.disabled =
            false;

        createButton.textContent =
            "CREATE ACCOUNT";

    }

}


// ======================================
// COPY
// ======================================

async function copyText(id) {

    const text =
        document
            .getElementById(id)
            .textContent;


    try {

        await navigator
            .clipboard
            .writeText(text);


        statusElement.textContent =
            "Copied";

    } catch {

        statusElement.textContent =
            "Copy failed";

    }

}


// ======================================
// COUNTRY NAME
// ======================================

function getCountryName(code) {

    const names = {

        ID: "🇮🇩 Indonesia",

        SG: "🇸🇬 Singapore",

        US: "🇺🇸 United States",

        DE: "🇩🇪 Germany",

        FR: "🇫🇷 France",

        NL: "🇳🇱 Netherlands",

        GB: "🇬🇧 United Kingdom",

        FI: "🇫🇮 Finland",

        IN: "🇮🇳 India",

        RU: "🇷🇺 Russia",

        RO: "🇷🇴 Romania",

        LV: "🇱🇻 Latvia",

        SK: "🇸🇰 Slovakia",

        AM: "🇦🇲 Armenia",

        ZW: "🇿🇼 Zimbabwe"

    };


    return names[code] || code;

}


// ======================================
// START
// ======================================

loadServers();

</script>

</body>
</html>"#;
