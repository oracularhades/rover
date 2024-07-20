import './css/sidebar-section.css';

export default function Sidebar_Section(props) {
    return (
        <div className='Sidebar_Section'>
            <div className='Sidebar_Section top'>
                <p className='header greyText'>{props.header}</p>
                <div className='side_line'/>
            </div>
            <div className='Sidebar_Section_children'>
                {props.children}
            </div>
        </div>
    )
}