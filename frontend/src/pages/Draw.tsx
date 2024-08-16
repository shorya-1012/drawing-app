import { useNavigate, useParams } from "react-router-dom";
import Canvas from "../components/drawing-page-ui/Canvas";
import { useEffect } from "react";
import Cookies from "js-cookie";
import axios from "axios";

export default function DrawingPage() {
    const { roomid } = useParams();
    const navigate = useNavigate();

    useEffect(() => {
        const checkRoom = async () => {
            try {
                const username = Cookies.get("username");
                Cookies.remove("username");
                if (!username) {
                    throw new Error("Username not provided");
                }
                const { data: _ } = await axios.post("http://localhost:3000/check-room", {
                    roomCode: roomid,
                    username: username
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
