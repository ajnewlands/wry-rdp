import ConfigModal from './config-modal';
import React from 'react';
import ReactDOM from 'react-dom';
import { createRoot } from 'react-dom/client';
import { Provider } from 'react-redux'
import { RootState, store } from './store';
import { useSelector } from 'react-redux';

import { ConnectionStatus, rdp_shutdown } from './reducers/rdpSlice';
import { connectWebsocket, WEBSOCKET } from './reducers/wsSlice';



//@ts-ignore typescript doesn't know about the webview extensions
window.ipc.postMessage('make-visible');

//@ts-ignore this is injected by the loader process.
connectWebsocket(WEBSOCKETADDRESS);

function updateCanvas(data: ArrayBuffer) {
    let canvas = document.getElementById('rdp-canvas') as HTMLCanvasElement;
    // Right now for test data === the entire screen
    // And maybe that would be best going forwards too - buffer in the Rust side,
    // apply diffs there and present JS with the whole screen.

    // For now we have hardcoded the size as 1024x768
    let ctx = canvas.getContext('2d');
    var idata = ctx.createImageData(1024, 768);
    idata.data.set(new Uint8Array(data));
    ctx.putImageData(idata, 0, 0);
}


const App = () => {
    const state = useSelector((state: RootState) => state);

	React.useEffect(() => {
        WEBSOCKET.onmessage = (event) => {
            if (event.data instanceof ArrayBuffer) {
                updateCanvas(event.data);
            } else {
                const obj = JSON.parse(event.data);
                switch (obj.kind) {
                    case "rdp_close":
                        store.dispatch(rdp_shutdown());
                        break;
                    default:
                        console.log(`unhandled socket message, kind is ${obj.kind}`);
                }
            }
        }
	}, [])

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

const keyUpOrDown = (ev: KeyboardEvent, down: boolean) => {
    WEBSOCKET.send(JSON.stringify({ KeyboardEvent: { action: down ? "down" : "up", key: ev.key}}));
}

canvas.onmousedown = (ev) => {
    mouseUpOrDown(ev, true)
    // The canvas needs to have focus in order to receive key events.
    canvas.focus();
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

canvas.onkeydown = (ev) => {
    keyUpOrDown(ev, true);
    ev.preventDefault();
}

canvas.onkeyup = (ev) => {
    keyUpOrDown(ev, false);
    ev.preventDefault();
}

