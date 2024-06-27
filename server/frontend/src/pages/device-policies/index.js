import Home1 from "@/components/home/home";
import Policy_Sidebar from "../../components/internal_components/device-policies/policy_sidebar";
import "./css/device-policies.css";

export default function Device_policies() {
    return (
        <div className="frame_div">
            <Home1 className="device_policies">
                <Policy_Sidebar/>
            </Home1>
        </div>
    )
}