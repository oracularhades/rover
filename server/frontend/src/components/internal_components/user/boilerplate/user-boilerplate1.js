import Button_with_icon from '../../../button/image/button_with_icon';
import './css/user-boilerplate1.css';

export default function User_Boilerplate1(props) {
    const user = props.data;
    return (
        <div className='user_boilerplate1'>
            <div className='user_boilerplate1_user_details'>
                <img alt="Your profile picture" style={{ width: 85, height: 85 }} src="https://cdn.bsky.app/img/avatar/plain/did:plc:wtdzzfgzjpirnk5wvpjutqoy/bafkreic5ffd2emwnoeyh5kwwipjrpdaizgbwvkcv45esphgyudvv3jimca@jpeg"/>
                <div className='user_boilerplate1_user_details_metadata'>
                    <h2 className='user_boilerplate1_user_details_metadata_full_name'>{user.name}</h2>
                    <p className='user_boilerplate1_user_details_metadata_email'>{user.email}</p>
                    {/* <p className='user_boilerplate1_user_details_suspended_reason'><b>Suspended [Indefinite]</b>: Adult content on school computers</p> */}
                </div>
            </div>
            <div className='user_boilerplate1_actions'>
                <Button_with_icon icon="/icons/pen-to-square.svg">Update information</Button_with_icon>
                <Button_with_icon icon="/icons/lock-reset.svg">Reset credentials</Button_with_icon>
                <Button_with_icon icon="/icons/block.svg">Suspend</Button_with_icon>
                <Button_with_icon icon="/icons/ellipsis-solid.svg"></Button_with_icon>
            </div>
        </div>
    )
}