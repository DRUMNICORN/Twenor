// jukebox will be iframe at first to https://www.riffusion.com/?&prompt=deep,+smooth+synthwave+with+a+dream-like+atmosphere&seed=123&denoising=0.75&seedImageId=og_beat

// https://www.riffusion.com/?&prompt=deep,+smooth+synthwave+with+a+dream-like+atmosphere&seed=123&denoising=0.75&seedImageId=og_beat

"use client"

import React from 'react';

const Jukebox = () => {
    return (
        <div style={{ height: '80vh' }}>
            <iframe
                style={{ width: '100%', height: '100%' }}
                src="https://www.riffusion.com/?&prompt=deep,+smooth+synthwave+with+a+dream-like+atmosphere&seed=123&denoising=0.75&seedImageId=og_beat" width="100%" height="100%" frameBorder="0"></iframe>
        </div>
    );
}

export default Jukebox;