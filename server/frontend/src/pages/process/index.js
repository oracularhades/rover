import "@/styles/global.css";
import "@/styles/flags.css";
import "./css/processes.css";
import Home1 from "@/components/home/home";
import Table1 from "@/components/tables/table1/table1";
import No_results from "@/components/tip/no_results";
import { useEffect, useRef, useState } from "react";
import { Rover } from "@oracularhades/rover";
import { creds } from "@/global";
import LoadingSpinner from "@/components/miscellaneous/loadingspinner";
import Link from "next/link";

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
                let data = [];
                response.data.forEach(element => {
                    // This forEach was originally for adding the options column, but here we'll just make another object so we can order the keys correctly, without some annoyingly over the top code. It does mean any values returned from the server have to be added here in future versions, but that's a problem for future me to write code to fix.
                    let obj = {
                        // id: user.id,
                        "device name": <Link href={`/device/${element.device.id}`}>{element.device.alias}</Link>,
                        "user": <p><Link href={`/user/${element.user.id}`}>josh@motionfans.com</Link> (unix_example)</p>,
                        process: element.process,
                        pathname: element.pathname,
                        publisher: element.publisher,
                        "is user admin?": element.admin_user,
                        "is process admin?": element.is_admin_process,
                        created: element.created,
                        "last seen": element.last_seen,
                        // hash: element.hash,
                        // size: element.size,
                        // threads: element.threads
                    };

                    data.push(obj);
                });

                set_processes(data);
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
            <Home1 className="home_padding align_items_center">
                <LoadingSpinner speed="600ms" style={{ width: 15, height: 15 }}/>
            </Home1>
        )
    }

    return (
        <Home1 className="home_padding default_row_gap">
            <h2>Processes</h2>
            {processes.length > 0 && <Table1 data={processes}/>}
            {processes.length == 0 && <div>
                <No_results tip="Setup process management" tip_href="https://github.com/oracularhades/rover/wiki/Setup-process-management"/>
            </div>}
        </Home1>
    )
}