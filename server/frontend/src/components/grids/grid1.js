import './../global.css';
import "./css/grid1.css"

export default function Grid1(props) {
    return (
        <div className={`grid1 ${props.className}`}>
            {props.children}
        </div>
    )
}