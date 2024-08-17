import axios from "axios";
import { createContext, useContext, useEffect, useState } from "react";

interface AuthContextType {
    userId: string | null,
    refetchUserId: () => Promise<void>
};

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
    const [userId, setUserId] = useState(null);

    const fetchUserId = async () => {
        const { data } = await axios.get("http://localhost:3000/get-user-id");
        setUserId(data)
    }

    useEffect(() => {
        fetchUserId();
    }, [])

    return (
        <AuthContext.Provider value={{
            userId: userId,
            refetchUserId: fetchUserId
        }}>
            {children}
        </AuthContext.Provider>
    )
}

export const useAuth = () => {
    const authContext = useContext(AuthContext);
    if (!authContext) throw new Error("useAuth() must be called within AuthProvider");
    return authContext;
}
