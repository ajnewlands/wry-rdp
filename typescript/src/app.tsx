import ConfigModal from './config-modal';
import React from 'react';
import ReactDOM from 'react-dom';
import { createRoot } from 'react-dom/client';
import { Provider } from 'react-redux'
import { RootState, store } from './store';
import { useSelector } from 'react-redux';

import { createBootstrapComponent } from 'react-bootstrap/esm/ThemeProvider';
import { ConnectionStatus } from './reducers/rdpSlice';

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

//@ts-ignore
connectWebsocket(WEBSOCKETADDRESS);
//@ts-ignore typescript doesn't know about the webview extensions
window.ipc.postMessage('make-visible');

const App = () => {
    const state = useSelector((state: RootState) => state);

    return (<>
        {state.rdp.status === ConnectionStatus.NotConnected && <ConfigModal />}
    </>);
}

let root = createRoot(document.getElementById('app-root'));
root.render(<Provider store={store}><App /></Provider>);