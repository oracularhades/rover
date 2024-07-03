import general from "./general.js";
import { Rover, getCreds } from "./index.js";
import { getRoverApiURL } from "./routing.js";

async function list() {
    const response = await Rover(await getCreds()).fetch_wrapper(`${getRoverApiURL()}/user/list`, {
        method: 'GET',
        mode: 'cors',
        cache: 'default',
        credentials: 'same-origin',
        edirect: 'error',
        referrerPolicy: 'no-referrer'
    })
    
    const response_data = response.json();
    if (response_data.error == true) {
        throw response_data;
    }
    
    return response_data;
}

async function get(id) {
    const response = await Rover(await getCreds()).fetch_wrapper(`${getRoverApiURL()}/user/get?${general().objectToParams({ id })}`, {
        method: 'GET',
        mode: 'cors',
        cache: 'default',
        credentials: 'same-origin',
        edirect: 'error',
        referrerPolicy: 'no-referrer'
    })
    
    const response_data = response.json();
    if (response_data.error == true) {
        throw response_data;
    }
    
    return response_data;
}

async function create(data) {
    data.action = "create";
    return await update(data)
}

async function update(data) {
    const response = await Rover(await getCreds()).fetch_wrapper(`${getRoverApiURL()}/user/update?${general().objectToParams({ id: data.id })}`, {
        method: 'POST',
        mode: 'cors',
        cache: 'default',
        credentials: 'same-origin',
        edirect: 'error',
        referrerPolicy: 'no-referrer',
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(data)
    })
    
    const response_data = response.json();
    if (response_data.error == true) {
        throw response_data;
    }
    
    return response_data;
}

const user = { list, get, create, update };
export default user;