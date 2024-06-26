"use client"
import './css/login1.css';
import FormStyle_1 from '../forms/form_style1';

import Input_with_header from '@/components/input/input_with_header';
import Login_With from '@/components/login/forms/login_with';
import Or_Bar from '@/components/login/forms/or_bar';
import Frame_AestheticMetadataPanel from '@/components/miscellaneous/frame_aesthetic_metadata_panel';
import './../../global.css';

export default function Login1() {
  return (
    <Frame_AestheticMetadataPanel>
      <FormStyle_1>
        <div className='FormStyle_1_div'>
          <Input_with_header header="Email" placeholder={`name@${process.env.NEXT_PUBLIC_email_placeholder_domain}`}></Input_with_header>
          <button className='FormStyle_1_div_login_button'>Login</button>
        </div>
      </FormStyle_1>
    </Frame_AestheticMetadataPanel>
  );
}
