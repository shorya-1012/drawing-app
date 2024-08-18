import { Shapes, Clipboard } from 'lucide-react'
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
import { Button, buttonVariants } from './button'
import { cn } from '../../lib/utils'
import { useState } from 'react'
import { handleRedirect } from '../../lib/helpers'
import { useNavigate } from 'react-router-dom'

export const JoinRoomButton = () => {
    const [roomId, setRoomId] = useState("");
    const navigate = useNavigate();

    const handlePaste = (e: any) => {
        e.preventDefault();
        navigator.clipboard.readText().then((text) => setRoomId(text))
    }

    return (
        <Dialog>
            <DialogTrigger
                className={cn(buttonVariants({ variant: 'secondary' }), 'flex gap-x-2')}
            >
                <Shapes />
                Join
            </DialogTrigger>
            <DialogContent aria-describedby={"Room Form"} className="w-96 f font-spaceGrotesk">
                <DialogHeader>
                    <DialogTitle className="flex items-center">
                        Join Room
                    </DialogTitle>
                    <DialogDescription className="w-full flex items-center justify-start">
                    Enter A room Id to join
                    </DialogDescription>
                </DialogHeader>
                <form className="space-y-6 w-full">
                    <div className="grid w-full items-center gap-1.5">
                        <label htmlFor="roomId">Room Id</label>
                        <section className={cn("flex gap-2 w-full items-center cursor-pointer")}>
                            <Input
                                type="text"
                                id="roomId"
                                autoComplete='off'
                                className="w-72"
                                value={roomId}
                                placeholder='Enter room ID'
                                onChange={(e) => setRoomId(e.target.value)}
                            />
                            <Button
                                size="sm"
                                className="px-3"
                                onClick={handlePaste}
                            >
                                <span className="sr-only">Paste</span>
                                <Clipboard className="h-4 w-4" />
                            </Button>
                            <DialogClose id='close-button'>
                            </DialogClose>
                        </section>
                    </div>
                </form>
                <DialogFooter className=''>
                    <Button
                        disabled={!roomId}
                        onClick={() => handleRedirect({ roomId, navigate })}
                        className="ml-auto mt-3 px-3 py-5 border-[1px] rounded-xl">Join Room
                    </Button>
                </DialogFooter>
            </DialogContent>
        </Dialog >
    )
}
