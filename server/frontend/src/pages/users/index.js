import Home1 from "@/components/home/home";
import "./../../../styles/global.css";
import "./../../../styles/flags.css";
import Table1 from "@/components/tables/table1/table1";
import Link from "next/link";
import "./css/users.css";
import { creds, to_table } from "../../global";
import TopbarPage1 from "@/components/internal_components/topbar/page/topbar-page1";
import UserCreate1 from "@/components/internal_components/user/dialog/user-create1";
import { useEffect, useRef, useState } from "react";
import { Rover } from "@oracularhades/rover";

export default function Users() {
    const should_run = useRef(true);
    const [data, set_data] = useState([]);

    useEffect(() => {
        if (should_run.current != true) {
            return;
        }
        should_run.current = false;

        get_users();
    });

    async function get_users() {
        const response = await Rover(creds()).user.list();
        set_data(response.data);
    }

    return (
        <div className="frame_div">
            <Home1 className="home_padding align_items_center">
                <UserCreate1 id="user_create_1"/>
                <TopbarPage1>
                    <p></p>
                    <button onClick={() => { document.getElementById("user_create_1").showModal() }}>Create user</button>
                </TopbarPage1>
                <Table1 data={to_table(data)}/>
            </Home1>
        </div>
    )
}