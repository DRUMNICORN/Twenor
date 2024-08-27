"use client"

import React from 'react';

export default function ErrorPage() {
    // redirect to home page
    React.useEffect(() => {
        window.location.href = '/';
    }, []);


    return (
        <div
            style={
                {
                    width: '100%',
                    height: '100%',
                }}
        >
            <span
                style={
                    {
                        position: 'absolute',
                        top: '50%',
                        left: '50%',
                        fontSize: '42vh',
                        color: 'var(--color-sub)',
                        cursor: 'pointer',
                        transform: 'translate(-50%, -50%)',
                        msTransform: 'translate(-50%, -50%)',
                        WebkitTransform: 'translate(-50%, -50%)',

                    }
                }
            >?</span>
        </div>
    );
}
