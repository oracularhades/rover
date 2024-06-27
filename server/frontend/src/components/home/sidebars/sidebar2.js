import ProfilePic from "@/components/user/profile_pic";
import "./css/sidebar2.css";
import SidebarButton1 from "./sidebar-buttons/sidebarbutton1";
import UserCard1 from "@/components/user/user_cards/user_card1";

export default function Sidebar2(props) {
    return (
        <div className="sidebar2">
            <ProfilePic hover={<UserCard1 user={{ name: "Example user", email: "user@example.com" }}>
                <button>Logout</button>
            </UserCard1>}/>
            <SidebarButton1 alias="Home" href="/home" icon="/icons/home.svg"/>
            <SidebarButton1 alias="Users" href="/users" icon="/icons/people.svg"/>
            <SidebarButton1 alias="Devices" href="/devices" icon="/icons/devices.svg"/>
            <SidebarButton1 alias="Network" href="/network" icon="/icons/lan.svg"/>
            <SidebarButton1 alias="Device Policies" href="/device-policies" icon="/icons/device-settings.svg"/>
            <SidebarButton1 alias="Browser Isolation" href="/browser-isolation" icon="/icons/cube.svg"/>
            <SidebarButton1 alias="Processes" href="/processes" icon="/icons/monitor-heart.svg"/>
            <SidebarButton1 alias="Settings" href="/settings" icon="/icons/settings.svg"/>
        </div>
    )
}