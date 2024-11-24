import Home1 from "@/components/home/home";
import Device_Component from "@/components/internal_components/devices/device_component";
import "@/styles/global.css";
import No_results from "@/components/tip/no_results";
import { useEffect, useRef, useState } from "react";
import { Rover } from "@oracularhades/rover";
import { creds } from "@/global";
import LoadingSpinner from "@/components/miscellaneous/loadingspinner";

export default function Devices() {
    const should_run = useRef(true);
    const [devices, set_devices] = useState([]);
    const [loading, set_loading] = useState(true);

    useEffect(() => {
        if (should_run.current != true) {
            return;
        }
        should_run.current = false;

        get_devices();
    });

    async function get_devices() {
        set_loading(true);

        try {
            const response = await Rover(creds()).device.list();
            if (response.ok == true) {
                set_devices(response.data);
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

    const devices_ul = devices.map((data) => {
        return (
            <Device_Component data={data}/>
        )
    });

    return (
        <Home1 className="home_padding default_row_gap">
            <h2>Devices</h2>
            <div className="components_ul">
                {devices.length >= 0 && devices_ul}
            </div>
            {devices.length == 0 && <No_results tip="Enroll a device" tip_href="https://gitlab.com/oracularhades/rover/wiki/Enroll-a-device"/>}
        </Home1>
    )
}