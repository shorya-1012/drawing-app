import { useMutation } from "@tanstack/react-query";
import { Button } from "./button"
import axios from "axios";
import { useAuth } from "../../lib/AuthProvider";
import { useNavigate } from "react-router-dom";

export const SignOutButton = () => {
    const { refetchUserId } = useAuth();
    const navigate = useNavigate();

    const { mutate: handleSignOut } = useMutation({
        mutationFn: async () => {
            const { data } = await axios.post("http://localhost:3000/sign-out");
            console.log(data);
            return data;
        },
        onSuccess: async () => {
            await refetchUserId();
            navigate("/");
        },
        onError: (error) => {
            console.log(error)
            alert(error);
        }
    })

    return (
        <Button
            onClick={() => handleSignOut()}
            variant={'destructive'}>
            Sign out
        </Button>
    )
}
