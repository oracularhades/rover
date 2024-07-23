import Home1 from "@/components/home/home";
import "./../../../styles/global.css";
import "./../../../styles/flags.css";
import Table1 from "@/components/tables/table1/table1";
import "./css/processes.css";
import No_results from "@/components/tip/no_results";
import { useEffect, useRef, useState } from "react";
import { Rover } from "@oracularhades/rover";
import { creds } from "@/global";
import LoadingSpinner from "@/components/miscellaneous/loadingspinner";

export default function Processes() {
    const should_run = useRef(true);
    const [processes, set_processes] = useState([]);
    const [loading, set_loading] = useState(true);

    useEffect(() => {
        if (should_run.current != true) {
            return;
        }
        should_run.current = false;

        get_processes();
    });

    async function get_processes() {
        set_loading(true);

        try {
            const response = await Rover(creds()).process.list();
            if (response.ok == true) {
                set_processes(response.data);
                set_loading(false);
            }
        } catch (error) {
            alert(error.message);
            return;
        }
    }

    // let data = [
        // {
        //     Device: <Link href="/devices/hi">Josh's MBP</Link>,
        //     "Process": "spotify.exe",
        //     // "Last seen": "24 minutes ago (11:10am 15 Feb 2024)",
        //     "Last seen": "active now",
        //     User: "josh",
        //     "Admin user": "false",
        //     PID: "40494",
        //     Publisher: "Spotify Inc",
        //     Hash: "asd0193ud90u3diojdqoiejdioqej",
        //     Threads: 249,
        //     Size: "499.9MB",
        //     Pathname: "/Applications/Photos",
        //     "View details": <Link href="/processes/hi">Open</Link>,
        // }
    // ]

    if (loading == true) {
        return (
            <div className="frame_div">
                <Home1 className="home_padding align_items_center">
                    <LoadingSpinner speed="600ms" style={{ width: 15, height: 15 }}/>
                </Home1>
            </div>
        )
    }

    return (
        <div className="frame_div">
            <Home1 className="home_padding align_items_center">
                {processes.length > 0 && <Table1 data={processes}/>}
                {processes.length == 0 && <div>
                    <No_results tip="Setup process management" tip_href="https://github.com/oracularhades/rover/wiki/Setup-process-management"/>
                </div>}
            </Home1>
        </div>
    )
}