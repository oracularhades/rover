import Home1 from "@/components/home/home";
import Device_Component from "@/components/internal_components/devices/device_component";
import "./../../../styles/global.css";

export default function Devices() {
    return (
        <div className="frame_div">
            <Home1 className="home_padding align_items_center">
                <Device_Component/>
            </Home1>
        </div>
    )
}