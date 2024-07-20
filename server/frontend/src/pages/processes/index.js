import Home1 from "@/components/home/home";
import "./../../../styles/global.css";
import "./../../../styles/flags.css";
import Table1 from "@/components/tables/table1/table1";
import "./css/processes.css";
import No_results from "@/components/tip/no_results";

export default function Processes() {
    let data = [
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
    ]

    return (
        <div className="frame_div">
            <Home1 className="home_padding align_items_center">
                {data.length > 0 && <Table1 data={data}/>}
                {data.length == 0 && <div>
                    <No_results tip="Setup process management" tip_href="https://github.com/oracularhades/rover/wiki/Setup-process-management"/>
                </div>}
            </Home1>
        </div>
    )
}