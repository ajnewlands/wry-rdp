import ConfigModal from './config-modal';
import React from 'react';
import ReactDOM from 'react-dom';
import { createRoot } from 'react-dom/client';

import { createBootstrapComponent } from 'react-bootstrap/esm/ThemeProvider';

export function connectWebsocket(address) {
    console.log(`creating new socket connection to ${address}`);
    const ws = new WebSocket(`ws://${address}`);
    ws.onopen = (event) => {
        console.log(`Internal socket connection established`);

        ws.send('hello!');
    };

    ws.onmessage = (event) => {
        console.log(`received ${event.data}`);
    }

    ws.onerror = (ev) => {
        console.error(`Internal socket threw an error: ${ev}`);
    }

    ws.onclose = (ev) => {
        console.log(`Internal socket closed: ${ev.code}`);
    }
}
// Make this function available as a global in the browser, so that 
// we can call it easily upon page load.
(window as any).connectWebsocket = connectWebsocket;
//@ts-ignore typescript doesn't know about the webview extensions
window.ipc.postMessage('make-visible');

let root = createRoot(document.getElementById('app-root'));
root.render(<ConfigModal />);