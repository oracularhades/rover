import "./../../styles/global.css";
import Home1 from "@/components/home/home";
import No_results from "@/components/tip/no_results";

export default function Home() {
    return (
        <div className="frame_div">
            <Home1 className="home_padding align_items_center">
                <No_results/>
            </Home1>
        </div>
    )
}