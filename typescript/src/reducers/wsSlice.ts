import { createSlice } from '@reduxjs/toolkit';
import type { PayloadAction } from '@reduxjs/toolkit';
import type { RDPConfiguration } from './rdpSlice';

export var WEBSOCKET: WebSocket;

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

export function connectWebsocket(address) {
    console.log(`creating new socket connection to ${address}`);
    WEBSOCKET = new WebSocket(`ws://${address}`);
    WEBSOCKET.binaryType = 'arraybuffer';
    WEBSOCKET.onopen = (event) => {
        console.log(`Internal socket connection established`);
    };

    WEBSOCKET.onmessage = (event) => {
        if (event.data instanceof ArrayBuffer) {
            updateCanvas(event.data);
        } else {
            console.log(`WS received: ${event.data}`);
        }
    }

    WEBSOCKET.onerror = (ev) => {
        console.error(`Internal socket threw an error: ${ev}`);
        WEBSOCKET = null;
    }

    WEBSOCKET.onclose = (ev) => {
        console.log(`Internal socket closed: ${ev.code}`);
        WEBSOCKET = null;
    }
}

export function requestRDPConnection(cfg: RDPConfiguration) {
    if (WEBSOCKET) {
        WEBSOCKET.send(JSON.stringify({ "RDPConnect": cfg }));
    }
}

const wsSlice = createSlice({
    name: 'ws',
    initialState: {},
    reducers: {
    }
})

export default wsSlice.reducer