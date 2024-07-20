import Sidebar_Base from '@/components//home/sidebars/sidebar-base';
import Sidebar_Section from '@/components/home/sidebars/sidebar-section';
import './css/policy_sidebar.css';

export default function Policy_Sidebar(props) {
    const Policy_button = ((props) => {
        return (
            <button className='policy_button'>
                {props.children}
            </button>
        )
    });
    
    return (
        <Sidebar_Base className="policy_sidebar secondary_element">
            <Sidebar_Section header="Networking">
                <Policy_button>Network Config</Policy_button>
                <Policy_button>Firewall</Policy_button>
                <Policy_button>Connectivity</Policy_button>
            </Sidebar_Section>

            <Sidebar_Section header="On-device">
                <Policy_button>Browser</Policy_button>
                <Policy_button>Application</Policy_button>
                <Policy_button>Email</Policy_button>
            </Sidebar_Section>

            <Sidebar_Section header="Detection">
                <Policy_button>EDR</Policy_button>
            </Sidebar_Section>

            <Sidebar_Section header="More">
                <Policy_button>Registry (Custom)</Policy_button>
            </Sidebar_Section>

            {/* ^ Gotta add a way to block applications too */}
            {/* <Policy_button>MS Defender</Policy_button> */}
            {/* <Policy_button>Custom</Policy_button> */}
        </Sidebar_Base>
    )
}