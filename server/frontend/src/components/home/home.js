import Base from "../base";
import Sidebar2 from "./sidebars/sidebar2";
import './../global.css';
import "./css/home.css"
import Layout_Topbar from "../layout/layout_topbar";
import Switcher from "../miscellaneous/switcher";
import DropdownOption1 from "../dropdown/options/DropdownOption1";

export default function Home1(props) {
    return (
        <Base className="home1">
            <Sidebar2/>
            <div className="home1_children">
                <Layout_Topbar>
                    <Switcher header={`rover.internal.motionfans.com`}>
                        <DropdownOption1 checked={true}>ok</DropdownOption1>
                    </Switcher>
                </Layout_Topbar>
                <div className={props.className} style={props.style}>
                    {props.children}
                </div>
            </div>
        </Base>
    )
}