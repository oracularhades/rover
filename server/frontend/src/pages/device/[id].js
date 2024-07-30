import Home1 from "@/components/home/home";
import Device_Component from "@/components/internal_components/devices/device_component";
import "@/styles/global.css";
import "./css/device.css";
import Topbar_Actions from "@/components/topbar/topbar_actions";
import Topbar_Actions_Button from "@/components/topbar/topbar_actions_button";
import Backdrop_content from "@/components/rows/backdrop/backdrop_content";
import Grid1 from "../../components/grids/grid1";

export default function Devices() {
    return (
        <div className="frame_div">
            <Home1 className="device_page home_padding">
                <div className="device_topbar">
                    <Device_Component hide_right_buttons={true}/>
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
        </div>
    )
}