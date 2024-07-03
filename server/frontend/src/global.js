function to_table(data) {
    let table_export = [];
    data.forEach((data) => {
        let keys_to_push = [];
        
        Object.keys(data).map((key) => {
            keys_to_push.push(key);
        });

        let not_found_keys = [];
        Object.keys(table_export).map((key) => {
            if (!data[key]) {
                not_found_keys.push(key);
            }
        });
        
        keys_to_push.forEach((key) => {
            if (!table_export[key]) {
                table_export[key] = [];
            }

            if (data[key]) {
                table_export[key].push(data[key]);
            } else {
                table_export[key].push("data[key]");
            }
        })

        not_found_keys.forEach((key) => {
            if (!table_export[key]) {
                table_export[key] = [];
            }
            
            table_export[key].push("null");
        });
    });

    return table_export;
}

async function redirect_to_login_if_required() {
    if (!await localStorage.getItem("auth")) {
        let host = window.location.hostname;
        if (window.location.port != 443 && window.location.port != 80) {
            host = `${host}:${window.location.port}`;
        }
        
        let params = new URLSearchParams({
            redirect: `${window.location.protocol}//${host}`
        });

        window.location.href = `/guard/frontend/login?${params.toString()}`;
    }
}

function creds() {
    const auth = JSON.parse(localStorage.getItem("auth"));
    return auth;
}

export { to_table, redirect_to_login_if_required, creds };