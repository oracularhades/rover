import './css/customization.css';
import './css/settings.css';
import Home1 from "@/components/home/home";
import Input_with_header from '../../components/input/input_with_header';
import Selector1 from '../../components/media/selectors/selector1';

export default function Settings_Customization() {
    return (
        <div className="frame_div">
            <Home1 className="home_padding settings">
                <Selector1 header="Logo" src={null} type="clubs_logo" onImageId={(image_id) => {  }}/>
                <Input_with_header header="Alias" value={"placeholder"}/>
                <Input_with_header header="Description" value={"placeholder"}/>
                <button>Save changes</button>
            </Home1>
        </div>
    )
}