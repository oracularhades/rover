import './css/device_component.css';
import { useRouter } from 'next/router';
import Link from 'next/link';

export default function Device_Component(props) {
    const router = useRouter();

    if (!props.data) {
        return <p>Device component here, but no data was passed.</p>
    }
    const data = props.data;

    const Right_Button = ((props) => {
        return (
            <button onClick={() => { router.push(props.href); }} className='right_button'>
                <img src={props.icon}/>
            </button>
        )
    });

    return (
        <div className='device_component secondary_element shade'>
            <Link href={`/device/${data.id}`} className='device_component_left'>
                <img className='device_component_icon' src="/icons/computer.svg"/>
                <div className='device_component_metadata'>
                    <p className='device_component_metadata_alias'>{data.alias}</p>
                    <p className='device_component_metadata_device_type'>{data.os_type} ({data.os_version}) • {new Date(data.created).toLocaleDateString()}</p>
                </div>
            </Link>

            {props.hide_right_buttons != true && <div className='device_component_right'>
                <Right_Button href={`/device/${data.id}`} icon="/icons/device_logs.svg"/>
                <Right_Button href={`/device/${data.id}`} icon="/icons/pencil_border.svg"/>
                <Right_Button icon="/icons/trash.svg"/>
            </div>}
        </div>
    )
}