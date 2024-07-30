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

export default function User() {
    const should_run = useRef(true);
    const [users, set_users] = useState([1]);
    const [loading, set_loading] = useState(false);
    const [tab, set_tab] = useState("sign_in_logs");

    useEffect(() => {
        if (should_run.current != true) {
            return;
        }
        should_run.current = false;

        // get_users();
    });

    function User_details(props) {
        return (
            <a className="user_details_clickable underline gryeText no-text-select" onClick={() => { create_user() }}>Details</a>
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
                        id: user.id,
                        first_name: user.first_name,
                        last_name: user.last_name,
                        email: user.email,
                        permission: user.permission
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
            <div className="frame_div">
                <Home1 className="home_padding align_items_center">
                    <LoadingSpinner speed="600ms" style={{ width: 15, height: 15 }}/>
                </Home1>
            </div>
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
        <div className="frame_div">
            <Home1 className="home_padding">
                {users.length == 0 && <div className="generic_column_div align_items_center">
                    <No_results custom_header="User not found"/>
                </div>}

                {users.length > 0 && <div className="generic_column_div">
                    <UserCreate1 on_success={user_created} id="user_create_1"/>
                    <User_Boilerplate1/>

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
        </div>
    )
}