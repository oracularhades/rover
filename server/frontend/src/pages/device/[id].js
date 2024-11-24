import Home1 from "@/components/home/home";
import Device_Component from "@/components/internal_components/devices/device_component";
import "@/styles/global.css";
import "./css/device.css";
import Topbar_Actions from "@/components/topbar/topbar_actions";
import Topbar_Actions_Button from "@/components/topbar/topbar_actions_button";
import Backdrop_content from "@/components/rows/backdrop/backdrop_content";
import Grid1 from "../../components/grids/grid1";
import { useRouter } from "next/router";
import { useEffect, useRef, useState } from "react";
import { Rover } from "@oracularhades/rover";
import { creds } from "@/global";

export default function Devices() {
    const router = useRouter();
    const should_run = useRef(true);
    const [device, set_device] = useState([1]);
    const [loading, set_loading] = useState(true);
    const [tab, set_tab] = useState("sign_in_logs");

    useEffect(() => {
        if (should_run.current == router.query.id || !router.query.id) {
            return;
        }
        should_run.current = router.query.id;

        get_device();
    });

    async function get_device() {
        set_loading(true);

        try {
            const response = await Rover(creds()).device.get(router.query.id);
            if (response.ok == true) {
                set_device(response.data[0]);
                set_loading(false);
            }
        } catch (error) {
            alert(error.message);
            return;
        }
    }

    return (
        <Home1 className="device_page home_padding">
            <div className="device_topbar">
                <Device_Component data={device} hide_right_buttons={true} embed={true}/>
                <Topbar_Actions>
                    <Topbar_Actions_Button icon="/icons/save_as_floppy.svg">Update details</Topbar_Actions_Button>
                    <Topbar_Actions_Button icon="/icons/gavel.svg">Suspend</Topbar_Actions_Button>
                    <Topbar_Actions_Button icon="/icons/trash.svg">Remove</Topbar_Actions_Button>
                </Topbar_Actions>
            </div>

            <Grid1 className="device_grid">
                <Backdrop_content header="Audit Logs">
                </Backdrop_content>

                <Backdrop_content header="System Logs">
                </Backdrop_content>

                <Backdrop_content header="Processes">
                </Backdrop_content>

                <Backdrop_content header="Web traffic">
                </Backdrop_content>
                
                <Backdrop_content header="Applied Policies">
                </Backdrop_content>
            </Grid1>
        </Home1>
    )
}