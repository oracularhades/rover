import Home1 from "@/components/home/home";
import Device_Component from "@/components/internal_components/devices/device_component";
import "./../../../styles/global.css";
import No_results from "@/components/tip/no_results";

export default function Devices() {
    return (
        <div className="frame_div">
            <Home1 className="home_padding align_items_center">
                {/* <Device_Component/> */}
                <No_results tip="Enroll a device" tip_href="https://github.com/oracularhades/rover/wiki/Enroll-a-device"/>
            </Home1>
        </div>
    )
}