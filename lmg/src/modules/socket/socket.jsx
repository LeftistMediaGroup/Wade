import { io } from 'socket.io-client';


const socket = io(`ws://${import.meta.env.VITE_HOST}`, {
    withCredentials: true
});

socket.on("connect", () => {
    console.log(`Socket Connected`);
})

export default socket;