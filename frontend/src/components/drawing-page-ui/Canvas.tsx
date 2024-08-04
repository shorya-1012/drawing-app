import { useEffect, useRef, useState } from "react"

type DrawLine = {
    ctx: CanvasRenderingContext2D,
    currPoint: Point
};

type Point = {
    x: number,
    y: number,
}

export default function Canvas() {

    const canvasRef = useRef<HTMLCanvasElement>(null);
    const [isDrawing, setIsDrawing] = useState(false);

    useEffect(() => {
        const mouseMoveHandler = (e: MouseEvent) => {
            if (!isDrawing) return
            const context = canvasRef.current?.getContext('2d');
            const currPoint = calculateMouseInCanvas(e);
            if (!currPoint || !context) return;
            drawLine({ ctx: context, currPoint });
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
            canvasRef.current?.getContext('2d')?.beginPath();
        }

        // add event listener
        canvasRef.current?.addEventListener('mousemove', mouseMoveHandler);
        window.addEventListener('mouseup', handleMouseUp);

        // remove event listener
        return () => {
            canvasRef.current?.removeEventListener('mousemove', mouseMoveHandler);
            window.addEventListener('mouseup', handleMouseUp);
        }

    }, [isDrawing]);

    const drawLine = ({ ctx, currPoint }: DrawLine) => {
        const { x: currX, y: currY } = currPoint;
        const lineColor = "#000";
        const lineWidth = 5;

        ctx.lineWidth = lineWidth;
        ctx.strokeStyle = lineColor;
        ctx.lineCap = "round"

        ctx.lineTo(currX, currY);
        ctx.stroke();

        ctx.beginPath();
        ctx.moveTo(currX, currY);
    }

    return (
        <canvas
            ref={canvasRef}
            width={850}
            height={850}
            onMouseDown={() => setIsDrawing(true)}
            className="border border-black rounded-xl"
        />
    )
}
