import './css/topbar_actions.css';

export default function Topbar_Actions(props) {
    return (
        <div className="topbar_actions secondary_element">
            {props.children}
        </div>
    )
}