import "./css/users.css";
import "@/styles/global.css";
import "@/styles/flags.css";
import Home1 from "@/components/home/home";
import Table1 from "@/components/tables/table1/table1";
import { creds } from "../../global";
import TopbarPage1 from "@/components/internal_components/topbar/page/topbar-page1";
import UserCreate1 from "@/components/internal_components/user/dialog/user-create1";
import { useEffect, useRef, useState } from "react";
import { Rover } from "@oracularhades/rover";
import No_results from "@/components/tip/no_results";
import LoadingSpinner from "@/components/miscellaneous/loadingspinner";

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
            <a className="user_details_clickable no-text-select underline gryeText" onClick={() => { create_user() }}>Details</a>
        )
    }

    async function get_users() {
        set_loading(true);

        try {
            const response = await Rover(creds()).user.list();
            if (response.ok == true) {
                let user_data = [];
                response.data.forEach(user => {
                    // This forEach was originally for adding the options column, but here we'll just make another object so we can order the keys correctly, without some annoyingly over the top code. It does mean any values returned from the server have to be added here in future versions, but that's a problem for future me to write code to fix.
                    let user_obj = {
                        // id: user.id,
                        first_name: user.first_name,
                        last_name: user.last_name,
                        email: user.email,
                        permission: user.permission && user.permission != 0 && user.permission || "standard"
                    };

                    user_obj.options = <User_details data={user}/>

                    user_data.push(user_obj);
                });
                set_users(user_data);
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

    return (
        <Home1 className="default_row_gap home_padding">
            <UserCreate1 on_success={user_created} id="user_create_1"/>
            <TopbarPage1>
                <h2>Users</h2>
                <button onClick={() => { create_user() }}>Create user</button>
            </TopbarPage1>
            {users.length > 0 && <Table1 data={users}/>}
            {/* note to self: need a way to update a user's permisisons, probably adding tabs between user actions and content boxes. So content boxes are under "overview" and you can get more specific. */}
            {users.length == 0 && <div className="align_items_center">
                <No_results/>
            </div>}
        </Home1>
    )
}