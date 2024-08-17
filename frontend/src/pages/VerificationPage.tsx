import { useState } from "react";
import {
    InputOTP,
    InputOTPGroup,
    InputOTPSeparator,
    InputOTPSlot,
} from "../components/ui/input-otp"
import { cn } from "../lib/utils";
import { useNavigate } from "react-router-dom";
import axios from "axios";
import { useMutation } from "@tanstack/react-query";
import { useAuth } from "../lib/AuthProvider";


export default function VerificationPage() {

    const [code, setCode] = useState("");
    const currentDate = new Date();

    const date = currentDate.toLocaleDateString('en-GB');

    const navigate = useNavigate();

    const { userId } = useAuth();

    if (userId) {
        navigate("/");
    }

    const { mutate: handleVerification, isPending } = useMutation({
        mutationFn: async () => {

            const { data } = await axios.post("http://localhost:3000/verify", {
                verification_code: code
            });
            return data;
        },
        onSuccess: () => {
            navigate("/login");
        },
        onError: (error) => {
            console.log(error)
            alert(error);
        }
    })

    return (
        <div className="w-screen h-screen overflow-hidden flex justify-center ">
            <div className={cn("bg-gradient-to-t from-gray-700 via-gray-900 to-black h-screen max-w-[680px] mx-auto p-[45px_30px_60px] font-poppins text-sm text-[#434343] ",
                "flex flex-col justify-center"
            )}>
                <header className="">
                    <div className="flex justify-between items-center">
                        <span className="font-spaceGrotesk font-semibold text-white text-4xl">Drawing App</span>
                        <span className="text-white text-base leading-[30px]">{date}</span>
                    </div>
                </header>

                <main className="mt-[70px] bg-white p-[92px_30px_115px] rounded-[30px] text-center">
                    <div className="max-w-[489px] mx-auto flex flex-col gap-y-2 items-center">
                        <h1 className="text-[24px] font-medium text-[#1f1f1f]">Enter Verification Code</h1>
                        <p className="mt-[17px] font-medium tracking-[0.56px] mb-5">
                            A verification code had been sent to your email address. Your Account will only be created once you have verified your email using the verification code. The code is valid only for{' '}
                            <span className="font-semibold text-[#1f1f1f]">15 minutes</span>. You will not be able to login without verification.
                        </p>
                        <InputOTP
                            autoFocus
                            maxLength={6}
                            value={code}
                            onComplete={handleVerification}
                            onChange={(val) => setCode(val)}
                        >
                            <InputOTPGroup>
                                <InputOTPSlot index={0} />
                                <InputOTPSlot index={1} />
                                <InputOTPSlot index={2} />
                            </InputOTPGroup>
                            <InputOTPSeparator />
                            <InputOTPGroup>
                                <InputOTPSlot index={3} />
                                <InputOTPSlot index={4} />
                                <InputOTPSlot index={5} />
                            </InputOTPGroup>
                        </InputOTP>
                    </div>
                </main>
            </div>
        </div>
    );
};

