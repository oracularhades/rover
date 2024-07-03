import { to_table } from '@/global';
import './table1.css';

export default function Table1(props) {
    if (!props.data) {
        return;
    }

    const data = to_table(props.data);
    const keys = Object.keys(data);
    // const keys = ["id", "first_name", "last_name", "email", "admin_permission_flags"];

    const column_headers = keys.map((key) => {
        return (
            <th>{key}</th>
        )
    });

    const columns = props.data.map((data) => {
        let content = keys.map((key) => {
            return (
                <td>{data[key]}</td>
            )
        });

        return (
            <tr>
                {content}
            </tr>
        );
    });

    return (
        <table className='table1'>
            <tbody>
                <tr>
                    {column_headers}
                </tr>

                {columns}
            </tbody>
        </table>
    )
}