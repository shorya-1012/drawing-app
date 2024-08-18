import { useEffect, useRef, useState } from "react"
import { Button } from "../ui/button";
import { GradientPicker } from "../ui/colorPicker";
import { io } from 'socket.io-client'
import { useParams } from "react-router-dom";
import { useAuth } from "../../lib/AuthProvider";

const socket = io('http://localhost:3000/socket');

// socket.on('disconnect', (reason) => {
//     console.log(`disconnected from server: ${reason}`, null);
// });

type DrawLine = {
    ctx: CanvasRenderingContext2D,
    prevPoint: Point | null,
    currPoint: Point,
    color: string
};

type Point = {
    x: number,
    y: number,
}

export default function Canvas() {
    const { roomid } = useParams();

    const canvasRef = useRef<HTMLCanvasElement>(null);
    const prevPoint = useRef<Point | null>(null);
    const [color, setColor] = useState("#000");
    const [isDrawing, setIsDrawing] = useState(false);
    const { userId } = useAuth();

    useEffect(() => {
        const context = canvasRef.current?.getContext('2d');

        socket.emit("client-connect", {
            roomCode: roomid,
            user_id: userId!
        });

        socket.on("get-client-state", () => {
            console.log("recieve req for state")
            const client_state = canvasRef.current?.toDataURL();
            if (!client_state) return
            socket.emit("client-state", {
                state: client_state,
                roomCode: roomid
            });
        })

        socket.on("server-state", (data) => {
            let img = new Image();
            img.src = data;
            img.onload = () => {
                context?.drawImage(img, 0, 0);
            }
        })

        socket.on("clear-canvas", () => {
            clearCanvas();
        })

        socket.on("get-draw-line", (data: DrawLine & string) => {
            if (!context) return;
            drawLine({ ctx: context, prevPoint: data.prevPoint, currPoint: data.currPoint, color: data.color })
        })

        return () => {
            socket.off("get-client-state");
            socket.off("get-draw-line");
            socket.off("clear-canvas");
            socket.off("server-state");
            socket.disconnect();
        }
    }, [])

    useEffect(() => {
        const mouseMoveHandler = (e: MouseEvent) => {
            if (!isDrawing) return
            const context = canvasRef.current?.getContext('2d');
            const currPoint = calculateMouseInCanvas(e);
            if (!currPoint || !context) return;
            paintCanvas({ ctx: context, prevPoint: prevPoint.current, currPoint, color: color });
            prevPoint.current = currPoint;
        }

        const calculateMouseInCanvas = (e: MouseEvent) => {
            const rect = canvasRef.current?.getBoundingClientRect();
            if (!rect) return;
            return {
                x: e.clientX - rect.x,
                y: e.clientY - rect.y,
            } as Point;
        }

        const handleMouseUp = () => {
            setIsDrawing(false);
            prevPoint.current = null;
        }

        // add event listener
        canvasRef.current?.addEventListener('mousemove', mouseMoveHandler);
        window.addEventListener('mouseup', handleMouseUp);

        // remove event listener
        return () => {
            canvasRef.current?.removeEventListener('mousemove', mouseMoveHandler);
            window.removeEventListener('mouseup', handleMouseUp);
        }

    }, [isDrawing]);

    const emitClearCanvas = () => {
        socket.emit("clear-canvas", {
            roomCode: roomid,
            user_id: userId
        });
        clearCanvas();
    }

    const clearCanvas = () => {
        if (!canvasRef) return;
        const context = canvasRef.current?.getContext('2d');
        if (!context) return;
        context.clearRect(0, 0, canvasRef.current!.width, canvasRef.current!.height);
    }

    const paintCanvas = ({ ctx, prevPoint, currPoint }: DrawLine) => {
        socket.emit("draw-line", {
            prevPoint: prevPoint,
            currPoint,
            roomCode: roomid,
            color
        })
        drawLine({ ctx, prevPoint, currPoint, color })
    }

    const drawLine = ({ ctx, prevPoint, currPoint, color }: DrawLine) => {

        const { x: currX, y: currY } = currPoint;
        const lineColor = color;
        const lineWidth = 5;

        const start = prevPoint ?? currPoint;

        ctx.beginPath()
        ctx.lineWidth = lineWidth;
        ctx.strokeStyle = lineColor;
        ctx.lineCap = "round"


        ctx.moveTo(start.x, start.y);
        ctx.lineTo(currX, currY);
        ctx.stroke();
    }

    return (
        <div className="w-full h-full flex items-center gap-x-5 justify-center">
            <div className="flex flex-col items-center gap-y-2">
                <GradientPicker background={color} setBackground={setColor} />
                <Button
                    onClick={emitClearCanvas}
                    variant={'destructive'}
                >
                    Clear Canvas
                </Button>
            </div>
            <canvas
                ref={canvasRef}
                width={850}
                height={850}
                onMouseDown={() => setIsDrawing(true)}
                className="border border-black rounded-xl"
            />
        </div>
    )
}
