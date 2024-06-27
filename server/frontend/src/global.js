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

            table_export[key].push(data[key]);
        })

        not_found_keys.forEach((key) => {
            if (!table_export[key]) {
                table_export[key] = [];
            }
            
            table_export[key].push(null);
        });
    });

    return table_export;
}

export { to_table };