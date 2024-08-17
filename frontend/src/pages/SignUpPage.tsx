import { useRef, useState } from "react";
import { Eye, EyeOffIcon } from "lucide-react";
import { Button } from "../components/ui/button";
import { Link, useNavigate } from "react-router-dom";
import { cn } from "../lib/utils";
import { useMutation } from "@tanstack/react-query";
import axios from "axios";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "../components/ui/select"
import { useAuth } from "../lib/AuthProvider";

const genders = [
    "Male",
    "Female",
    "Batman",
    "Gmail",
    "Optimus Prime",
    "B-2 Stealth Bomber",
    "T-Rex",
    "Mechanic",
]


axios.defaults.withCredentials = true;

export default function SignUp() {
    const [type, setType] = useState("password")
    const usernameRef = useRef<HTMLInputElement>(null);
    const emailRef = useRef<HTMLInputElement>(null);
    const passwordRef = useRef<HTMLInputElement>(null);
    const [gender, setGender] = useState(genders[0]);
    const navigate = useNavigate();

    const { userId } = useAuth();

    if (userId) {
        navigate("/");
    }

    const { mutate: handleSubmit, isPending } = useMutation({
        mutationFn: async (e: any) => {
            e.preventDefault();
            const username = usernameRef.current?.value
            const password = passwordRef.current?.value
            const email = emailRef.current?.value

            if (!username || !password || !email) {
                throw new Error("Username or password or email not provided");
            }

            const payload = {
                username,
                email,
                password,
                gender
            }

            const { data } = await axios.post("http://localhost:3000/register", payload);
            return data;
        },
        onSuccess: () => {
            navigate("/verify");
        },
        onError: (error) => {
            console.log(error)
            alert(error);
        }
    })

    return (
        <div className={cn("w-screen h-screen overflow-hidden flex justify-center items-center font-spaceGrotesk",
            "bg-gradient-to-r from-pink-300 via-purple-300 to-indigo-400"
        )}>
            <div className="rounded-xl w-[400px] md:w-[550px] grid grid-rows-4 border-2 border-sky-400 bg-gray-50">
                <div className="row-span-1 flex items-center justify-center text-3xl md:text-3xl text-zinc-900 font-semibold">Create Account</div>
                <form onSubmit={handleSubmit} className="row-span-3 flex flex-col px-5 gap-y-14 mt-5">
                    <section className="flex flex-col gap-y-1 border-b-[2px] border-zinc-900 pb-2 mx-5">
                        <input
                            id="username"
                            ref={usernameRef}
                            autoComplete="off"
                            placeholder="Username"
                            className="bg-transparent border-none text-lg focus:outline-none placeholder-zinc-900"
                        />
                    </section>
                    <section className="flex flex-col gap-y-1 border-b-[2px] border-zinc-900 pb-2 mx-5">
                        <input
                            id="email"
                            type="email"
                            ref={emailRef}
                            autoComplete="off"
                            placeholder="Email"
                            className="bg-transparent border-none text-lg focus:outline-none placeholder-zinc-900"
                        />
                    </section>
                    <section className="flex flex-col gap-y-1 border-b-[2px] border-zinc-900 pb-2 mx-5">
                        <div className="w-full grid grid-cols-[_15fr_1fr]">
                            <input
                                id="password"
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
                    </section>
                    <section className="flex flex-col gap-y-1 border-b-[2px] border-zinc-900 pb-2 mx-5">
                        <Select onValueChange={(val) => setGender(val)}>
                            <SelectTrigger >
                                <SelectValue placeholder="Gender" className="text-xl font-semibold" />
                            </SelectTrigger>
                            <SelectContent
                                className="text-lg"
                            >
                                {
                                    genders.map((g, i) => {
                                        return (
                                            <SelectItem key={i} value={g}>{g}</SelectItem>
                                        )
                                    })
                                }
                            </SelectContent>
                        </Select>
                    </section>
                    <section className="flex flex-col gap-y-1 mx-5 ">
                        <Button
                            type="submit"
                            disabled={isPending}
                            variant={'default'}
                            className="bg-gradient-to-r from-purple-300 via-purple-400 to-purple-800 font-semibold"
                        >
                            {isPending ? "Loading" : "Create Account"}
                        </Button>
                    </section>
                    <section className="flex gap-x-1 pb-2 mx-5 items-center justify-center mb-5">
                        <span className="text-lg font-semibold">Already have an account?</span>
                        <Link to={"/login"} className="text-blue-500 text-lg font-semibold">Login</Link>
                    </section>
                </form>
            </div>
        </div>
    )
}
