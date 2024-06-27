import Home1 from "@/components/home/home";
import "./../../styles/global.css";
import Table1 from "../components/tables/table1/table1";
import "./../../styles/flags.css";
import Link from "next/link";
import { to_table } from "../global";

export default function Network() {
    let data = [
        {
            Device: <Link href="/devices/hi">Josh's MBP</Link>,
            "Domain": "example.com",
            "IP address": "127.0.0.1",
            "Destination country": <div>
                <span class="flag-nz"/>
                New Zealand
            </div>,
            "Destination registrant": "AS4771 Spark New Zealand Trading Ltd.",
            Protocol: "TCP",
            Size: "400kb",
            info: "62271 → 55152 [SYN] Seq=0 Win=65535 Len=0 MSS=1440 WS=64 TSval=406005761 TSecr=0 SACK_PERM"
        }
    ]

    return (
        <div className="frame_div">
            <Home1 className="home_padding align_items_center">
                {/* <Network_traffic_Component/> */}
                <Table1 data={to_table(data)}/>
            </Home1>
        </div>
    )
}