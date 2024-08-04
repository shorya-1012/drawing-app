import { Link } from "react-router-dom"
import { Spotlight } from "../components/ui/Spotlight"

export default function HomePage() {

    return (
        <div className="h-screen w-full rounded-md flex md:items-center md:justify-center bg-black/[0.96] antialiased bg-grid-white/[0.02] relative overflow-hidden">
            <Spotlight
                className="-top-40 left-0 md:left-60 md:-top-20"
                fill="white"
            />
            <div className=" p-4 max-w-7xl  mx-auto relative z-10  w-full pt-20 md:pt-0">
                <h1 className="text-4xl md:text-7xl font-bold text-center bg-clip-text text-transparent bg-gradient-to-b from-neutral-50 to-neutral-400 bg-opacity-50">
                    Welcome to <br /> Drawing App
                </h1>
                <p className="mt-5 font-normal text-base text-neutral-300 max-w-lg text-center mx-auto">
                    This is a app that lets you draw whaatever you want and share it with others
                </p>
                <Link to={'/draw'} className="bg-red-500 p-3 rounded-xl text-white">Begin Drawing</Link>
            </div>
        </div>
    )
}
