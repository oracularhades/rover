import './css/settings.css';
import Home1 from "@/components/home/home";
import Backdrop_content from "@/components/rows/backdrop/backdrop_content";
import Rows_backdrop_row1 from "@/components/rows/backdrop/rows/rows-backdrop-row1";
import { useState } from 'react';

export default function Settings() {
    const [show_full_version, set_show_full_version] = useState(false);

    return (
        <div className="frame_div">
            <Home1 className="home_padding settings">
                <Backdrop_content header="Rover settings" style={{ maxWidth: 600 }}>
                    <Rows_backdrop_row1 icon="/icons/palette.svg" header="Customization" href="/settings/customization"/>
                    <Rows_backdrop_row1 icon="/icons/meshing.svg" header="Meshing" href="/settings/meshing"/>
                    <Rows_backdrop_row1 icon="/icons/post_box.svg" header="SMTP connection (notifications)" href="/settings/smtp"/>
                    <Rows_backdrop_row1 icon="/icons/database.svg" header="SQL connection" href="/settings/sql"/>
                    <Rows_backdrop_row1 icon="/icons/backup.svg" header="Backup & restore" href="/settings/backup-or-restore"/>
                    <p className="rover_version greyText">Rover versions: {show_full_version != true && <a className='underline' onClick={() => { set_show_full_version(true) }}>Show</a>}
                        {show_full_version == true && <div>
                            Rover admin panel: (Canary) 0.0.1
                            <br/>
                            Rover server: (Canary) 0.0.1
                        </div>}
                    </p>
                </Backdrop_content>

                <Backdrop_content header="My admin account" style={{ maxWidth: 600 }}>
                    <Rows_backdrop_row1 icon="/icons/security_hazard.svg" header="Two-factor authentication" href="/settings/me/two-factor"/>
                    <Rows_backdrop_row1 icon="/icons/key.svg" header="My Logins" href="/settings/me/my-logins"/>
                </Backdrop_content>
            </Home1>
        </div>
    )
}