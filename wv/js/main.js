function connectWebsocket(address) {
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
        console.error(`Internal socket threw an error: ${ev.error}`);
    }

    ws.onclose = (ev) => {
        console.log(`Internal socket closed: ${ev.code}`);
    }
}

window.ipc.postMessage('make-visible');