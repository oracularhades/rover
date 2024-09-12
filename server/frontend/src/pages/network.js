import "@/styles/global.css";
import "@/styles/flags.css";
import Home1 from "@/components/home/home";
import Table1 from "../components/tables/table1/table1";
import No_results from '@/components/tip/no_results';
import { creds, to_table } from "../global";
import LoadingSpinner from "@/components/miscellaneous/loadingspinner";
import { useEffect, useRef, useState } from "react";
import { Rover } from "@oracularhades/rover";
import Link from "next/link";

export default function Network() {
    const should_run = useRef(true);
    const [network, set_network] = useState([]);
    const [loading, set_loading] = useState(true);

    useEffect(() => {
        if (should_run.current != true) {
            return;
        }
        should_run.current = false;

        get_network();
    });

    async function get_network() {
        set_loading(true);

        try {
            const response = await Rover(creds()).network.list();
            if (response.ok == true) {
                let data = [];
                response.data.forEach(element => {
                    // This forEach was originally for adding the options column, but here we'll just make another object so we can order the keys correctly, without some annoyingly over the top code. It does mean any values returned from the server have to be added here in future versions, but that's a problem for future me to write code to fix.
                    let obj = {
                        "device name": element.device && <Link href={`/device/${element.device.id}`}>{element.device.alias}</Link> || null,
                        "user": element.user && <p><Link href={`/user/${element.user.id}`}>{element.user.email}</Link> ({element.user.system_user})</p> || null,
                        domain: element.domain,
                        ip_address: element.ip_address,
                        "IP Country": <div><span class={`flag-${element.destination_country.toLowerCase()}`}/> {element.destination_country}</div>,
                        "IP Registrant": element.destination_registrant,
                        protocol: element.protocol,
                        size: element.size,
                        created: element.created,
                        // info: element.info
                    };

                    data.push(obj);
                });

                set_network(data);
                set_loading(false);
            }
        } catch (error) {
            alert(error.message);
            return;
        }
    }

    if (loading == true) {
        return (
            <Home1 className="home_padding align_items_center">
                <LoadingSpinner speed="600ms" style={{ width: 15, height: 15 }}/>
            </Home1>
        )
    }

    // let data = [
        // {
        //     Device: <Link href="/devices/hi">Josh's MBP</Link>,
        //     "Domain": "example.com",
        //     "IP address": "127.0.0.1",
        //     "Destination country": <div>
        //         <span class="flag-nz"/>
        //         New Zealand
        //     </div>,
        //     "Destination registrant": "AS4771 Spark New Zealand Trading Ltd.",
        //     Protocol: "TCP",
        //     Size: "400kb",
        //     info: "62271 → 55152 [SYN] Seq=0 Win=65535 Len=0 MSS=1440 WS=64 TSval=406005761 TSecr=0 SACK_PERM"
        // }
    // ]

    return (
        <div className="frame_div">
            <Home1 className="home_padding default_row_gap">
                {/* <Network_traffic_Component/> */}
                <h2>Network</h2>
                {network.length > 0 && <Table1 data={network}/>}
                {network.length == 0 && <div>
                    <No_results tip="Setup network logging" tip_href="https://gitlab.com/oracularhades/rover/wiki/Setup-network-logging"/>
                </div>}
            </Home1>
        </div>
    )
}