import { getCreds } from "./index.js";

function getRoverApiURL() {
    const creds = getCreds();
    if (creds.host) {
        let host = new URL(creds.host);
        host.search = '';

        return host;
    }

    if (typeof window == "undefined") {
        throw "You must specify credentials.host. If you were in a browser, we'd just automatically infer the host from your pathname, but it doesn't look like you're in a browser."
    }

    let url = new URL(window.location.href);
    url.port = window.location.port;
    url.pathname = "/api";

    if (window.location.hostname == "localhost" || window.location.hostname == "127.0.0.1") {
        if (typeof localStorage != "undefined") {
            const custom_api_endpoint = localStorage.getItem("custom_api_endpoint");
            if (custom_api_endpoint) {
                url.href = custom_api_endpoint;
            }
        }
    } else {
        url.protocol = "https:";
    }

    url.search = '';

    return url.href;
}

export { getRoverApiURL };