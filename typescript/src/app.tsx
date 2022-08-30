import ConfigModal from './config-modal';
import React from 'react';
import ReactDOM from 'react-dom';
import { createRoot } from 'react-dom/client';
import { Provider } from 'react-redux'
import { RootState, store } from './store';
import { useSelector } from 'react-redux';

import { createBootstrapComponent } from 'react-bootstrap/esm/ThemeProvider';
import { ConnectionStatus } from './reducers/rdpSlice';
import { connectWebsocket, WEBSOCKET } from './reducers/wsSlice';



//@ts-ignore this is injected by the loader process.
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

/* Set up handlers on the canvas element */
const canvas = document.getElementById('rdp-canvas') as HTMLCanvasElement;

const mouseUpOrDown = (ev: MouseEvent, down: boolean) => {
    const cwidth = canvas.scrollWidth;
    const cheight = canvas.scrollHeight;
    const x = Math.floor((ev.offsetX / cwidth) * 1024);
    const y = Math.floor((ev.offsetY / cheight) * 764);

    let button;
    switch(ev.button) {
        case 0:
            button = 'left';
            break;
        case 1:
            button = 'middle';
            break;
        case 2:
            button = 'right';
            break;
        default:
            console.log(`Ignoring unhandled button press ${ev.button}`);
        
    }

    WEBSOCKET.send(JSON.stringify({ MouseEvent: { action: down ? "down" : "up", button: button, x: x, y: y}}));

}

canvas.onmousedown = (ev) => {
    mouseUpOrDown(ev, true)
    ev.preventDefault();
};
canvas.onmouseup = (ev) => {
    mouseUpOrDown(ev, false)
    ev.preventDefault();
};
canvas.onmousemove = (ev) => {
    if (ev.buttons !== 0) {
        const cwidth = canvas.scrollWidth;
        const cheight = canvas.scrollHeight;
        const x = Math.floor((ev.offsetX / cwidth) * 1024);
        const y = Math.floor((ev.offsetY / cheight) * 764);
        WEBSOCKET.send(JSON.stringify({ MouseEvent: { action: "move", button: "", x: x, y: y}}));
    }
}
canvas.oncontextmenu = () => false;

