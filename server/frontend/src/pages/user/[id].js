import "./css/user.css";
import "@/styles/global.css";
import "@/styles/flags.css";
import Home1 from "@/components/home/home";
import { creds, to_table } from "../../global";
import UserCreate1 from "@/components/internal_components/user/dialog/user-create1";
import { useEffect, useRef, useState } from "react";
import { Rover } from "@oracularhades/rover";
import No_results from "@/components/tip/no_results";
import LoadingSpinner from "@/components/miscellaneous/loadingspinner";
import User_Boilerplate1 from "../../components/internal_components/user/boilerplate/user-boilerplate1";
import Tab_row1 from "../../components/tab/row/tab_row1";
import Tab_button1 from "../../components/tab/button/tab_button1";
import { useRouter } from "next/router";

export default function User() {
    const router = useRouter();
    const should_run = useRef(true);
    const [user, set_user] = useState([1]);
    const [loading, set_loading] = useState(true);
    const [tab, set_tab] = useState("sign_in_logs");

    useEffect(() => {
        if (should_run.current == router.query.id || !router.query.id) {
            return;
        }
        should_run.current = router.query.id;

        get_users();
    });

    async function get_users() {
        set_loading(true);

        try {
            const response = await Rover(creds()).user.get(router.query.id);
            if (response.ok == true) {
                set_user(response.data[0]);
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
        <Home1 className="home_padding">
            {user == null && <div className="generic_column_div align_items_center">
                <No_results custom_header="User not found"/>
            </div>}

            {user && <div className="generic_column_div">
                <UserCreate1 on_success={user_created} id="user_create_1"/>
                <User_Boilerplate1 data={{ ...user, name: `${user.first_name} ${user.last_name}` }}/>

                {/* <div>
                    <p>Suspended user (until 29-08-2024 [31 days])</p>
                    <p>Vern Dempter: Student downloaded adult content to school computer</p>
                    <button>View all comments</button>
                </div> */}
                <Tab_row1 value={tab} on_change={(value) => { set_tab(value); }}>
                    <Tab_button1 element="sign_in_logs" value={tab} set_value={set_tab}>Sign-in logs</Tab_button1>
                    <Tab_button1 element="devices" value={tab} set_value={set_tab}>Devices</Tab_button1>
                    <Tab_button1 element="processes" value={tab} set_value={set_tab}>Processes</Tab_button1>
                    <Tab_button1 element="network" value={tab} set_value={set_tab}>Network</Tab_button1>
                    <Tab_button1 element="notes" value={tab} set_value={set_tab}>Notes (12)</Tab_button1>
                </Tab_row1>
                {/* <div>
                    <h1>Contact Email</h1>
                    <p>John.doe@example.com</p>

                    <h1>Secondary Contact Email</h1>
                    <p>parent.doe@example.com</p>

                    Custom user values
                    <h1>Birthdate</h1>
                    <p>19-01-99</p>
                </div> */}
                {/* <Grid1 className="device_grid">
                    <Backdrop_content header="Sign-in logs">
                    </Backdrop_content>

                    <Backdrop_content header="Devices">
                    </Backdrop_content>
                    
                    <Backdrop_content header="Processes">
                    </Backdrop_content>

                    <Backdrop_content header="Network">
                    </Backdrop_content>
                </Grid1> */}
            </div>}
        </Home1>
    )
}