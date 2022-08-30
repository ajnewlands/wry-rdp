import { createAsyncThunk, createSlice } from '@reduxjs/toolkit';
import type { PayloadAction } from '@reduxjs/toolkit';
import { requestRDPConnection } from './wsSlice';

export enum ConnectionStatus {
    NotConnected,
    Connecting,
    Connected
}

export type RDPConfiguration = {
    host: string,
    username: string,
    password: string,
    port: number,
};

const rdpSlice = createSlice({
    name: 'rdp',
    initialState: {
        cfg: {
            host: "127.0.0.1",
            username: "",
            password: "",
            port: 3389
        },
        status: ConnectionStatus.NotConnected,
    },
    reducers: {
        setPort: (state, action: PayloadAction<number>) => {
            state.cfg.port = action.payload;
        },
        setUser: (state, action: PayloadAction<string>) => {
            state.cfg.username = action.payload;
        },
        setPass: (state, action: PayloadAction<string>) => {
            state.cfg.password = action.payload;
        },
        setHost: (state, action: PayloadAction<string>) => {
            state.cfg.host = action.payload;
        },
        connect: (state) => {
            state.status = ConnectionStatus.Connecting;
            requestRDPConnection(state.cfg);
        },
        rdp_shutdown: (state) => {
            state.status = ConnectionStatus.NotConnected;
            const canvas = document.getElementById('rdp-canvas') as HTMLCanvasElement;
            canvas.getContext('2d').clearRect(0, 0, canvas.width, canvas.height);
        }
    }
})

export const { setPort, setUser, setHost, setPass, connect, rdp_shutdown } = rdpSlice.actions;
export default rdpSlice.reducer
