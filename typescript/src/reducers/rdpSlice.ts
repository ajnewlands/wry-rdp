import { createSlice } from '@reduxjs/toolkit';
import type { PayloadAction } from '@reduxjs/toolkit';

const rdpSlice = createSlice({
    name: 'rdp',
    initialState: {
        host: "127.0.0.1",
        username: "",
        password: "",
        port: 3389,
    },
    reducers: {
        setPort: (state, action: PayloadAction<number>) => {
            state.port = action.payload;
        }
    }
})

export const { setPort } = rdpSlice.actions;
export default rdpSlice.reducer