import { SiExcalidraw } from 'react-icons/si'
import { Shapes } from 'lucide-react'
import { Button } from "../components/ui/button"
import { useEffect, useState } from "react"
import { gsap, Expo } from 'gsap'
import { Vortex } from "../components/ui/vortex"
import { CreateRoomButton } from "../components/ui/createRoomButton"

export default function HomePage() {

    const [loadingCounter, setLoadingCounter] = useState(0);

    useEffect(() => {
        if (loadingCounter == 100) {
            introAnimation();
            return;
        }
        if (loadingCounter < 100) {
            const interval = setInterval(() => {
                setLoadingCounter(loadingCounter + 1);
            }, 10);
            return () => clearInterval(interval);
        }
    }, [loadingCounter]);

    const introAnimation = () => {
        const t1 = gsap.timeline({
            // onComplete: () => console.log("compelete")
        });
        t1.to("#follow", {
            width: "100%",
            ease: Expo.easeInOut,
            duration: 1.2,
            delay: 0.5
        })
            .to(".hide", {
                opacity: 0,
                duration: 0.3
            })
            .to(".hide", { display: "none", duration: 0.3 })
            .to("#follow", {
                height: "100%",
                color: "#fff",
                ease: Expo.easeInOut,
                duration: 0.7,
                delay: 0.1
            })
            .to("#content", {
                width: "100%",
                ease: Expo.easeInOut,
                duration: 0.7,
                delay: 0.5
            })
            .to(".title-line", {
                display: "flex",
                duration: 0.1
            })
            .to(".title-line", {
                opacity: 1,
                stagger: 0.15,
                ease: Expo.easeInOut,
                duration: 0.6
            })
    }

    return (
        <div className="h-screen w-screen relative font-spaceGrotesk">
            <div id="loading" className="h-full w-full bg-[#121212] flex justify-center items-center absolute top:0">
                <div id="follow" className="absolute bg-red-500 h-[4px] w-[0] left-0 z-[2] flex justify-center items-center text-2xl text-red-500 overflow-hidden">
                    <div className="flex flex-col gap-y-1 items-center justify-center text-3xl">
                        <SiExcalidraw />
                        <span className="font-semibold">
                            DoodlePalace
                        </span>
                    </div>
                </div>
                <div
                    id="progress-bar"
                    style={{ width: `${loadingCounter}%`, transition: "0.4s ease-out" }}
                    className="hide absolute left-0 bg-[#fff] h-[4px] w-0"
                >
                </div>
                <p
                    id="counter"
                    style={{ transform: "translateY(-15px)" }}
                    className="hide absolute text-[130px] text-[#fff]"
                >
                    {loadingCounter}
                </p>
            </div>
            <div id="content"
                className="h-full w-0 absolute left-0 top-0 p-[auto] z-[2] flex flex-col items-center justify-center bg-zinc-950 text-gray-50 gap-y-5 overflow-hidden"
            >
                <Vortex
                    backgroundColor="black"
                    rangeY={800}
                    particleCount={100}
                    baseHue={120}
                    className="flex items-center flex-col justify-center px-2 md:px-10  py-4 w-full h-full gap-y-5"
                >
                    <div className="title-line opacity-0 hidden text-3xl md:text-7xl font-bold dark:text-white text-nowrap">
                        Welcome to DoodlePalace
                    </div>
                    <div className="title-line opacity-0 hidden font-extralight text-base md:text-4xl dark:text-neutral-200 py-4 text-center">
                        Sketch, share, and collaborate on a live canvas with friends and colleagues.
                    </div>
                    <div className="title-line opacity-0 hidden gap-x-5 text-2xl">
                        <Button variant={"secondary"} className="flex items-center gap-x-3 text-gray-950">
                            <Shapes />
                            Join
                        </Button>
                        <CreateRoomButton />
                    </div>
                </Vortex>
            </div >
        </div>
    )
}
