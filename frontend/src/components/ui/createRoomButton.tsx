import { CirclePlus, Copy } from 'lucide-react'
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
import { handleRedirect } from '../../lib/helpers'
import { useNavigate } from 'react-router-dom'
import { useAuth } from '../../lib/AuthProvider'

export const CreateRoomButton = () => {
    const [roomId, setRoomId] = useState("");
    const navigate = useNavigate();
    const { userId } = useAuth();

    const createRoomHandler = async () => {
        try {
            if (!userId) throw new Error("You need to log in to create a room");
            const { data } = await axios.post("http://localhost:3000/create-room");
            setRoomId(data.roomCode);
        } catch (error) {
            document.getElementById('close-button')?.click();
            alert(error);
        }
    }

    const handleCopy = (e: any) => {
        e.preventDefault();
        navigator.clipboard.writeText(roomId).then(() => console.log("Copied to clipboard"));
    }

    return (
        <Dialog>
            <DialogTrigger
                onClick={() => createRoomHandler()}
                className={cn(buttonVariants({ variant: 'secondary' }), 'flex gap-x-2')}
            >
                <CirclePlus />
                Create
            </DialogTrigger>
            <DialogContent aria-describedby={"Room Form"} className="w-96 f font-spaceGrotesk">
                <DialogHeader>
                    <DialogTitle className="flex items-center">
                        Room Created
                    </DialogTitle>
                    <DialogDescription className="w-full flex items-center justify-start">
                        Share this room Id with others to let them join
                    </DialogDescription>
                </DialogHeader>
                <form className="space-y-6 w-full">
                    <div className="grid w-full items-center gap-1.5">
                        <section className={cn("gap-2 w-full items-center cursor-pointer", `flex`)}>
                            <Input
                                type="text"
                                id="roomId"
                                autoComplete='off'
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
                    <Button onClick={() => handleRedirect({ roomId, navigate })} className="ml-auto mt-3 px-3 py-5 border-[1px] rounded-xl">Join Room</Button>
                </DialogFooter>
            </DialogContent>
        </Dialog >
    )
}
