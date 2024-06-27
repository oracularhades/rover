import general from "../general.js";
import { Rover, getCreds } from "../index.js";
import { getRoverApiURL } from "../routing.js";

async function list() {
    const response = await Rover(await getCreds()).fetch_wrapper(`${getRoverApiURL()}/user/list?${general().objectToParams({ host, authentication_method, code })}`, {
        method: 'GET',
        mode: 'cors',
        cache: 'default',
        credentials: 'same-origin',
        edirect: 'error',
        referrerPolicy: 'no-referrer'
    })
    
    const data = response.json();

    if (data.error == true) {
        throw data;
    }
    
    return data;
}

async function get(data) {
    const response = await Rover(await getCreds()).fetch_wrapper(`${getRoverApiURL()}/user/get?${general().objectToParams({ id })}`, {
        method: 'GET',
        mode: 'cors',
        cache: 'default',
        credentials: 'same-origin',
        edirect: 'error',
        referrerPolicy: 'no-referrer'
    })
    
    const data = response.json();

    if (data.error == true) {
        throw data;
    }
    
    return data;
}

async function create(data) {
    data.action = "create";
    return await update(data)
}

async function update(data) {
    const response = await Rover(await getCreds()).fetch_wrapper(`${getRoverApiURL()}/user/update?${general().objectToParams({ id })}`, {
        method: 'POST',
        mode: 'cors',
        cache: 'default',
        credentials: 'same-origin',
        edirect: 'error',
        referrerPolicy: 'no-referrer',
        body: JSON.stringify(data)
    })
    
    const data = response.json();

    if (data.error == true) {
        throw data;
    }
    
    return data;
}

const user = { list, get, create, update };
export default user;