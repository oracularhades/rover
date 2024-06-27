import { Rover, getCreds } from "./index.js";
import { getRoverApiURL } from "./routing.js";
import general from "./general.js";

async function list(hostname) {
    const response = await Rover(await getCreds()).fetch_wrapper(`${getRoverApiURL()}/network/list?${general().objectToParams({ hostname })}`, {
        method: 'GET', // *GET, POST, PUT, DELETE, etc.
        mode: 'cors', // no-cors, *cors, same-origin
        cache: 'default', // *default, no-cache, reload, force-cache, only-if-cached
        credentials: 'same-origin', // include, *same-origin, omit
        headers: {
            'Content-Type': 'application/json'
        },
        redirect: 'error', // manual, *follow, error
        referrerPolicy: 'no-referrer', // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
    })
    
    const data = response.json();
    
    return data;
}


const network = { list };
export default network;