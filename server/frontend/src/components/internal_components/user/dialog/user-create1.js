import './css/user-create1.css';
import Dialog_Frame from "@/components/dialogs/dialog_frame";
import Input_with_header from "@/components/input/input_with_header";
import { creds } from '@/global.js';
import { Rover } from '@oracularhades/rover';
import { useState } from 'react';

export default function UserCreate1(props) {
    const [error, set_error] = useState(null);
    const [first_name, set_first_name] = useState(null);
    const [last_name, set_last_name] = useState(null);
    const [email, set_email] = useState(null);

    async function user_create() {
        const user = {
            first_name: first_name,
            last_name: last_name,
            email: email,
            permission: 0
        };

        try {
            const response = await Rover(creds()).user.create(user);
        } catch (response) {
            set_error(response.message);
            return;
        }

        if (props.on_success) {
            set_first_name(null);
            set_last_name(null);
            set_email(null);
            props.on_success();
        }
    }

    return (
        <Dialog_Frame id={props.id} header="Create user" className="user_create_1_dialog">
            {error && <p className='error_text'>{error}</p>}
            <Input_with_header header="First name" placeholder="John" value={first_name} onChange={(e) => { set_first_name(e.target.value); }}/>
            <Input_with_header header="Last name" placeholder="Doe" value={last_name} onChange={(e) => { set_last_name(e.target.value); }}/>

            <Input_with_header header="Email address" placeholder="user@example.com" value={email} onChange={(e) => { set_email(e.target.value); }}/>

            <p className='user_create_1_note greyText'>Note: Rover doesn't use passwords. A login email will be sent to {email} - External authentication via OAuth (e.g Microsoft/Google) is supported provided you enable it.</p>
            <button onClick={() => { user_create(); }}>Create user</button>
        </Dialog_Frame>
    )
}