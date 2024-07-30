import './css/tab_row1.css';
import '../button/css/tab_button1.css';

export default function Tab_row1(props) {
    return (
        <div className="tab_row1 tab_button1">
            {props.children}
        </div>
    )
}