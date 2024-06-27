import './css/network_traffic_component.css';
import '../../../../styles/flags.css';

export default function Network_traffic_Component(props) {
    const Right_Button = ((props) => {
        return (
            <button className='right_button'>
                <img src={props.icon}/>
            </button>
        )
    });

    return (
        <div className='device_component secondary_element'>
            <div className='device_component_metadata'>
                <p className='device_component_metadata_alias'><span class="flag-nz"/> 127.0.0.1</p>
                <p className='device_component_metadata_device_type'>Macbook • employee_1@example.com • 14/02/2024 01:04pm</p>
            </div>

            {props.hide_right_buttons != true && <div className='device_component_right'>
                <Right_Button icon="/icons/device_logs.svg"/>
                <Right_Button icon="/icons/pencil_border.svg"/>
                <Right_Button icon="/icons/trash.svg"/>
            </div>}
        </div>
    )
}