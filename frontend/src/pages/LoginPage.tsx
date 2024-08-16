import { useRef, useState } from "react";
import { Eye, EyeOffIcon } from "lucide-react";
import { Button } from "../components/ui/button";
import { Link, useNavigate } from "react-router-dom";
import { cn } from "../lib/utils";
import { useMutation } from "@tanstack/react-query";
import axios from "axios";

axios.defaults.withCredentials = true;

export default function LoginPage() {
    const [type, setType] = useState("password")
    const usernameRef = useRef<HTMLInputElement>(null);
    const passwordRef = useRef<HTMLInputElement>(null);
    const navigate = useNavigate();

    const { mutate: handleSubmit, isPending } = useMutation({
        mutationFn: async (e: any) => {
            e.preventDefault();
            if (!usernameRef.current || !passwordRef.current) {
                return
            }
            const username = usernameRef.current?.value
            const password = passwordRef.current?.value
            const payload = {
                username,
                password
            }

            const { data } = await axios.post("http://localhost:3000/login", payload);
            console.log(data);
            return data;
        },
        onSuccess: () => {
            // navigate("/");
        },
        onError: (error) => {
            console.log(error)
            alert(error);
        }
    })

    return (
        <div className={cn("w-screen h-screen overflow-hidden flex justify-center items-center font-spaceGrotesk",
            "bg-gradient-to-r from-green-300 via-blue-500 to-purple-600"
        )}>
            <div className="rounded-xl w-[400px] md:w-[550px] h-[600px] md:h-[600px] grid grid-rows-4 border-2 border-sky-400 bg-gray-50">
                <div className="row-span-1 flex items-center justify-center text-4xl md:text-5xl text-zinc-900 font-semibold">Login</div>
                <form onSubmit={handleSubmit} className="row-span-3 flex flex-col px-5 gap-y-14 mt-5">
                    <section className="flex flex-col gap-y-1 border-b-[2px] border-zinc-900 pb-2 mx-5">
                        <input
                            id="username"
                            ref={usernameRef}
                            autoComplete="off"
                            placeholder="Username"
                            className="bg-transparent border-none text-lg focus:outline-none placeholder-zinc-900"
                        />
                        <label htmlFor="username"></label>
                    </section>
                    <section className="flex flex-col gap-y-1 border-b-[2px] border-zinc-900 pb-2 mx-5">
                        <div className="w-full grid grid-cols-[_15fr_1fr]">
                            <input
                                id="username"
                                ref={passwordRef}
                                type={type}
                                autoComplete="off"
                                placeholder="Password"
                                className="bg-transparent text-lg focus:outline-none placeholder-zinc-900"
                            />
                            <button
                                type="button"
                                className=""
                                onClick={() => {
                                    setType(prev => prev == "password" ? "text" : "password")
                                }}
                            >
                                {type === "password" ?
                                    <Eye /> :
                                    <EyeOffIcon />

                                }
                            </button>
                        </div>
                        <label htmlFor="password"></label>
                    </section>
                    <section className="flex flex-col gap-y-1 mx-5">
                        <Button
                            type="submit"
                            variant={'default'}
                            className="bg-gradient-to-r from-purple-300 via-purple-400 to-purple-800 font-semibold"
                        >
                            Login
                        </Button>
                        <label htmlFor="username"></label>
                    </section>
                    <section className="flex gap-x-1 pb-2 mx-5 items-center justify-center">
                        <span className="text-lg font-semibold">Don't have an account?</span>
                        <Link to={"/sign-up"} className="text-blue-500 text-lg font-semibold">Sign-Up</Link>
                    </section>
                </form>
            </div>
        </div>
    )
}
