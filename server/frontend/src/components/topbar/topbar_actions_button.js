import './css/topbar_actions_button.css';

export default function Topbar_Actions_Button(props) {
    return (
        <button className='topbar_actions_button'>
            <img src={props.icon} alt={props.alt}/>
            {props.children}
        </button>
    )
}