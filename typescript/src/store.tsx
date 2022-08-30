import { configureStore } from '@reduxjs/toolkit'

import rdpReducer, { rdp_shutdown } from './reducers/rdpSlice';
import wsReducer from './reducers/wsSlice';

export const store = configureStore({
    reducer: { 
        rdp: rdpReducer,
        ws: wsReducer,
     }
})

export type RootState = ReturnType<typeof store.getState>
export type AppDispatch = typeof store.dispatch

export function rdpShutdown() {
    store.dispatch(rdp_shutdown);
}