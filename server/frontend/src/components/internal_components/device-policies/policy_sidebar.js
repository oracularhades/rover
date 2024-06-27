import Sidebar_Base from '../../home/sidebars/sidebar-base';
import './css/policy_sidebar.css';

export default function Policy_Sidebar(props) {
    const Policy_button = ((props) => {
        return (
            <button>
                {props.children}
            </button>
        )
    });
    return (
        <Sidebar_Base className="policy_sidebar secondary_element">
            <Policy_button>DNS</Policy_button>
            <Policy_button>Network policies</Policy_button>
            <Policy_button>Browser</Policy_button>
            <Policy_button>Email</Policy_button>
            <Policy_button>Connectivity</Policy_button>
            <Policy_button>Application</Policy_button>
            {/* ^ Gotta add a way to block applications too */}
            <Policy_button>MS Defender</Policy_button>
            <Policy_button>Custom</Policy_button>
        </Sidebar_Base>
    )
}