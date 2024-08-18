import { useNavigate, useParams } from "react-router-dom";
import Canvas from "../components/drawing-page-ui/Canvas";
import { useEffect } from "react";
import axios from "axios";
import { useAuth } from "../lib/AuthProvider";

export default function DrawingPage() {
    const { userId } = useAuth();
    const { roomid } = useParams();
    const navigate = useNavigate();
    if (!userId) {
        navigate("/login")
    }

    useEffect(() => {
        const checkRoom = async () => {
            try {
                const { data: _ } = await axios.post("http://localhost:3000/check-room", {
                    roomCode: roomid,
                });
            } catch (error) {
                alert(error);
                navigate("/")
            }
        }
        checkRoom();
    }, []);

    return (
        <div className="w-screen h-screen flex items-center  justify-center">
            <Canvas />
        </div>
    )
}
