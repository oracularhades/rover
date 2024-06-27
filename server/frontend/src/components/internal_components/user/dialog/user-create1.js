import './css/user-create1.css';
import Dialog_Frame from "@/components/dialogs/dialog_frame";
import Input_with_header from "@/components/input/input_with_header";

export default function UserCreate1(props) {
    return (
        <Dialog_Frame id={props.id} header="Create user" className="user_create_1_dialog">
            <Input_with_header header="First name" placeholder="John"/>
            <Input_with_header header="Last name" placeholder="Doe"/>

            <Input_with_header header="Email name" placeholder="user@example.com"/>

            <p className='user_create_1_note greyText'>Note: Rover does not use passwords. A login email will be sent to user@example.com. Users can also login with external authentication (e.g Microsoft/Google) if an admin chooses.</p>
            <button>Create user</button>
        </Dialog_Frame>
    )
}