import Home1 from "@/components/home/home";
import "./../../../styles/global.css";
import "./../../../styles/flags.css";
import Table1 from "@/components/tables/table1/table1";
import Link from "next/link";
import "./css/processes.css";
import { to_table } from "../../global";

export default function Processes() {
    let data = [
        {
            Device: <Link href="/devices/hi">Josh's MBP</Link>,
            "Process": "spotify.exe",
            // "Last seen": "24 minutes ago (11:10am 15 Feb 2024)",
            "Last seen": "active now",
            User: "josh",
            "Admin user": "false",
            PID: "40494",
            Publisher: "Spotify Inc",
            Hash: "asd0193ud90u3diojdqoiejdioqej",
            Threads: 249,
            Size: "499.9MB",
            Pathname: "/Applications/Photos",
            "View details": <Link href="/processes/hi">Open</Link>,
        }
    ]

    return (
        <div className="frame_div">
            <Home1 className="home_padding align_items_center">
                <Table1 data={to_table(data)}/>
            </Home1>
        </div>
    )
}