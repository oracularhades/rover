import fetch_wrapper from "./fetcher.js";
import general from "./general.js";
import auth from './lib.js';
import oauth from "./protocols/oauth.js";
import user from "./user.js";
import network from "./network.js";
import process from "./process.js";

let deviceIDG = null;
let privateKeyG = null;
let typeG = null;
let additional_data = null;

async function getCreds() {
    const pemHeader = "-----BEGIN PRIVATE KEY-----";
    const pemFooter = "-----END PRIVATE KEY-----";

    return {
        deviceid: deviceIDG,
        privatekey: pemHeader+privateKeyG+pemFooter,
        additional_data: additional_data,
        type: typeG
    };
}

function Rover(credsObject) {
    if (credsObject) {
        deviceIDG = credsObject.deviceid;
        privateKeyG = credsObject.privatekey;
        additional_data = credsObject.additional_data;
        typeG = credsObject.type;
    } else {
        console.warn("You need to specify a credentials object when initalizing Rover(). E.g Rover({ deviceID \"myawesomedeviceid\", \"privatekey\":\"awesomeprivatekey\"})");
    }

    return {
        metadata: metadata,
        auth: auth,
        oauth: oauth,
        fetch_wrapper: fetch_wrapper,
        general: general,
        user: user,
        network: network,
        process: process
    };
}

export { Rover, getCreds }