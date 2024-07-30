export default function Tab_button1(props) {
    return (
        <button className={`${props.className} ${props.value == props.element && "tab_button1_selected"}`} onClick={() => { props.set_value(props.element); }}>{props.children}</button>
    )
}