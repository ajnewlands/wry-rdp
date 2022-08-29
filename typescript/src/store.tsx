import { configureStore } from '@reduxjs/toolkit'

import rdpReducer from './reducers/rdpSlice';

export const store = configureStore({
    reducer: { rdp: rdpReducer }
})

export type RootState = ReturnType<typeof store.getState>
export type AppDispatch = typeof store.dispatch