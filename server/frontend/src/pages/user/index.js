import "./css/users.css";
import "@/styles/global.css";
import "@/styles/flags.css";
import Home1 from "@/components/home/home";
import { creds } from "../../global";
import TopbarPage1 from "@/components/internal_components/topbar/page/topbar-page1";
import UserCreate1 from "@/components/internal_components/user/dialog/user-create1";
import { useEffect, useRef, useState } from "react";
import { Rover } from "@oracularhades/rover";
import No_results from "@/components/tip/no_results";
import LoadingSpinner from "@/components/miscellaneous/loadingspinner";
import User_Component from "@/components/internal_components/user/user_component";

export default function Users() {
    const should_run = useRef(true);
    const [users, set_users] = useState([]);
    const [loading, set_loading] = useState(true);

    useEffect(() => {
        if (should_run.current != true) {
            return;
        }
        should_run.current = false;

        get_users();
    });

    function User_details(props) {
        return (
            <a className="user_details_clickable no-text-select underline gryeText" onClick={() => { create_user() }}>details</a>
        )
    }

    async function get_users() {
        set_loading(true);

        try {
            const response = await Rover(creds()).user.list();
            if (response.ok == true) {
                set_users(response.data);
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

    async function user_created() {
        get_users();
        document.getElementById("user_create_1").close();
    }

    function create_user() {
        document.getElementById("user_create_1").showModal();
    }

    const users_ul = users.map((data) => {
        return (
            <User_Component data={data}/>
        )
    });

    return (
        <Home1 className="default_row_gap home_padding">
            <UserCreate1 on_success={user_created} id="user_create_1"/>
            <TopbarPage1>
                <h2>Users</h2>
                <button onClick={() => { create_user() }}>Create user</button>
            </TopbarPage1>
            <div className="components_ul">
                {users.length >= 0 && users_ul}
            </div>
            {/* {users.length > 0 && <Table1 data={users}/>} */}
            {/* note to self: need a way to update a user's permisisons, probably adding tabs between user actions and content boxes. So content boxes are under "overview" and you can get more specific. */}
            {users.length == 0 && <div className="align_items_center">
                <No_results/>
            </div>}
        </Home1>
    )
}