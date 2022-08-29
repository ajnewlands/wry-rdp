import { createSlice } from '@reduxjs/toolkit';
import type { PayloadAction } from '@reduxjs/toolkit';

export enum ConnectionStatus {
    NotConnected,
    Connecting,
    Connected
}

const rdpSlice = createSlice({
    name: 'rdp',
    initialState: {
        host: "127.0.0.1",
        username: "",
        password: "",
        port: 3389,
        status: ConnectionStatus.NotConnected,
    },
    reducers: {
        setPort: (state, action: PayloadAction<number>) => {
            state.port = action.payload;
        },
        setUser: (state, action: PayloadAction<string>) => {
            state.username = action.payload;
        },
        setPass: (state, action: PayloadAction<string>) => {
            state.password = action.payload;
        },
        setHost: (state, action: PayloadAction<string>) => {
            state.host = action.payload;
        },
        connect: (state) => {
            state.status = ConnectionStatus.Connecting;
        }
    }
})

export const { setPort, setUser, setHost, setPass, connect } = rdpSlice.actions;
export default rdpSlice.reducer