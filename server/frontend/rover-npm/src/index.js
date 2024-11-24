import fetch_wrapper from "./fetcher.js";
import general from "./general.js";
import user from "./user.js";
import device from "./device.js";
import network from "./network.js";
import processes from "./process.js";

let creds = {};

function getCreds() {
    const pemHeader = "-----BEGIN PRIVATE KEY-----";
    const pemFooter = "-----END PRIVATE KEY-----";

    return {
        host: creds.host,
        device_id: creds.device_id,
        private_key: pemHeader+creds.private_key+pemFooter,
        additional_data: creds.additional_data,
        type: creds.type
    };
}

function Rover(credsObject) {
    if (credsObject) {
        creds = credsObject;
    } else {
        console.warn("You need to specify a credentials object when initalizing Rover(). E.g Rover({ deviceID \"myawesomedeviceid\", \"privatekey\":\"awesomeprivatekey\"})");
    }

    return {
        fetch_wrapper: fetch_wrapper,
        general: general,
        user: user,
        device: device,
        network: network,
        process: processes
    };
}

export { Rover, getCreds }