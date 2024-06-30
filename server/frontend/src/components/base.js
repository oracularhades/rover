import { redirect_to_login_if_required } from '@/global';
import { Roboto } from 'next/font/google';
import { useEffect, useRef } from 'react';

const roboto = Roboto({
    subsets: ['latin'],
    weight: ['400', '700']
})

export default function Base(props) {
    const should_run = useRef(true);
    useEffect(() => {
        if (should_run.current != true) {
            return;
        }
        should_run.current = false;
        redirect_to_login_if_required();
    });

    return (
        <div className={`${props.className} ${roboto.className}`} style={props.style}>
            {props.children}
        </div>
    )
}