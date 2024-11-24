import './css/user_component.css';
import { useRouter } from 'next/router';
import Link from 'next/link';

export default function User_Component(props) {
    const router = useRouter();

    if (!props.data) {
        return <p>User component here, but no data was passed.</p>
    }
    const data = props.data;

    const Right_Button = ((props) => {
        return (
            <button onClick={() => { router.push(props.href); }} className='right_button'>
                <img src={props.icon} {...props}/>
            </button>
        )
    });

    const permission = data && data.permission && data.permission != 0 && data.permission || "standard";

    let users_name = "Unnamed user";
    if (data.first_name || data.last_name) {
        users_name = `${data.first_name} ${data.last_name}`;
    }
    
    return (
        <div className={`user_component secondary_element ${props.embed != true && `shade`} hover`}>
            <Link href={`/user/${data.id}`} className='user_component_left'>
                <img className='user_component_icon' src="/assets/default-pfp.png"/>
                <div className='user_component_metadata'>
                    <p className='user_component_metadata_alias'>{users_name}</p>
                    <p className='user_component_metadata_device_type'>{data.email} • {permission}</p>
                </div>
            </Link>

            {props.hide_right_buttons != true && <div className='user_component_right'>
                <Right_Button alt="A pencil writing on a line" href={`/user/${data.id}`} icon="/icons/pencil_border.svg"/>
                <Right_Button alt="A trashcan" icon="/icons/trash.svg"/>
            </div>}
        </div>
    )
}