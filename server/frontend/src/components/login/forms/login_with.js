import './css/login_with.css';
import './../../global.css';

export default function Login_With(props) {
    let href = null;
    if (props.disabled != true) {
        href = props.redirect;
    }
    
    return (
        <a href={href} className='login_with'><button disabled={props.disabled}>
            <img src={props.icon}/>
            <p>Login with {props.service}</p>
        </button></a>
    )
}