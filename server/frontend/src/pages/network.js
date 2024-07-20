import "./../../styles/global.css";
import "./../../styles/flags.css";
import Home1 from "@/components/home/home";
import Table1 from "../components/tables/table1/table1";
import No_results from '@/components/tip/no_results';
import { creds, to_table } from "../global";
import LoadingSpinner from "@/components/miscellaneous/loadingspinner";
import { useEffect, useRef, useState } from "react";
import { Rover } from "@oracularhades/rover";

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
                set_network(response.data);
                set_loading(false);
            }
        } catch (error) {
            alert(error.message);
            return;
        }
    }

    if (loading == true) {
        return (
            <div className="frame_div">
                <Home1 className="home_padding align_items_center">
                    <LoadingSpinner speed="600ms" style={{ width: 15, height: 15 }}/>
                </Home1>
            </div>
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
            <Home1 className="home_padding align_items_center">
                {/* <Network_traffic_Component/> */}
                {network.length > 0 && <Table1 data={network}/>}
                {network.length == 0 && <div>
                    <No_results tip="Setup network logging" tip_href="https://github.com/oracularhades/rover/wiki/Setup-network-logging"/>
                </div>}
            </Home1>
        </div>
    )
}