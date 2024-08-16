import Cookies from 'js-cookie'
import { NavigateFunction } from 'react-router-dom';

export const handleRedirect = ({ roomId, username, navigate }: { roomId: string, username: string, navigate: NavigateFunction }) => {
    Cookies.set("username", username);
    navigate(`/draw/${roomId}`);
}

