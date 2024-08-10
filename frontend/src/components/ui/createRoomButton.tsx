import { CirclePlus, Copy } from 'lucide-react'
import { useNavigate } from "react-router-dom"
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
    DialogClose,
} from './dialog'
import { Input } from './input'
import axios from "axios"
import { Button, buttonVariants } from './button'
import { cn } from '../../lib/utils'
import { useState } from 'react'

export const CreateRoomButton = () => {
    const navigate = useNavigate();
    const [roomId, setRoomId] = useState("");
    const [username, setUsername] = useState("");

    const createRoomHandler = async () => {
        try {
            const { data } = await axios.post("http://localhost:3000/create-room", {
                username: username
            });
            setRoomId(data.roomCode);
        } catch (error) {
            document.getElementById('close-button')?.click();
            alert(error);
        }
    }

    const handleRedirect = () => {
        navigate(`/draw/${roomId}`);
    }

    const handleCopy = (e: any) => {
        e.preventDefault();
        navigator.clipboard.writeText(roomId).then(() => console.log("Copied to clipboard"));
    }

    return (
        <Dialog>
            <DialogTrigger
                className={cn(buttonVariants({ variant: 'secondary' }), 'flex gap-x-2')}
            >
                <CirclePlus />
                Create
            </DialogTrigger>
            <DialogContent aria-describedby={"Room Form"} className="w-96 f">
                <DialogHeader>
                    <DialogTitle className="flex items-center">
                        {roomId ? "Join Room" : "Create Room"}
                    </DialogTitle>
                    <DialogDescription className="w-full flex items-center justify-start">
                        {roomId ? "Share this room Id with others to let them join" : "Generate a room Id to share with friends"}
                    </DialogDescription>
                </DialogHeader>
                <form className="space-y-6 w-full">
                    <div className="grid w-full items-center gap-1.5">
                        <label htmlFor="username">Username</label>
                        <section className={cn("gap-2 w-full items-center cursor-pointer", `${!roomId ? "flex" : "hidden"}`)}>
                            <Input
                                type="text"
                                id="username"
                                className="w-full"
                                value={username}
                                placeholder='Enter your username'
                                onChange={e => setUsername(e.target.value)}
                            />
                            <DialogClose id='close-button'>
                            </DialogClose>
                        </section>
                        <section className={cn("gap-2 w-full items-center cursor-pointer", `${roomId ? "flex" : "hidden"}`)}>
                            <Input
                                type="text"
                                id="roomId"
                                readOnly
                                className="w-72"
                                value={roomId}
                            />
                            <Button
                                size="sm"
                                className="px-3"
                                onClick={handleCopy}
                            >
                                <span className="sr-only">Copy</span>
                                <Copy className="h-4 w-4" />
                            </Button>
                            <DialogClose id='close-button'>
                            </DialogClose>
                        </section>
                    </div>
                </form>
                <DialogFooter className=''>
                    {roomId ?
                        <Button onClick={handleRedirect} className="ml-auto mt-3 px-3 py-5 border-[1px] rounded-xl">Join Room</Button>
                        :
                        <Button disabled={username.length < 3} onClick={createRoomHandler} className="ml-auto mt-3 px-3 py-5 border-[1px] rounded-xl">Create Room</Button>
                    }
                </DialogFooter>
            </DialogContent>
        </Dialog >
    )
}
