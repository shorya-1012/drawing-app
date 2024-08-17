import { createRoutesFromElements, createBrowserRouter, Route } from "react-router-dom";
import HomePage from "../pages/HomePage";
import DrawingPage from "../pages/Draw";
import LoginPage from "../pages/LoginPage";
import SignUp from "../pages/SignUpPage";
import VerificationPage from "../pages/VerificationPage";

export const router = createBrowserRouter(
    createRoutesFromElements(
        <>
            <Route path="/" element={< HomePage />} />
            <Route path="/draw/:roomid" element={< DrawingPage />} />
            <Route path="/login" element={< LoginPage />} />
            <Route path="/sign-up" element={< SignUp />} />
            <Route path="/verify" element={< VerificationPage />} />
        </>
    )
);
