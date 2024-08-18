import { NavigateFunction } from 'react-router-dom';

export const handleRedirect = ({ roomId, navigate }: { roomId: string, navigate: NavigateFunction }) => {
    navigate(`/draw/${roomId}`);
}

