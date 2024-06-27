import './css/sidebar-base.css';

export default function Sidebar_Base(props) {
    return (
        <div className={`sidebar_base ${props.className}`} style={props.style}>
            {props.children}
        </div>
    )
}