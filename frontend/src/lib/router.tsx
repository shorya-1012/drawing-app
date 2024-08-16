import { createRoutesFromElements, createBrowserRouter, Route } from "react-router-dom";
import HomePage from "../pages/HomePage";
import DrawingPage from "../pages/Draw";
import LoginPage from "../pages/LoginPage";

export const router = createBrowserRouter(
    createRoutesFromElements(
        <>
            <Route path="/" element={< HomePage />} />
            <Route path="/draw/:roomid" element={< DrawingPage />} />
            <Route path="/login" element={< LoginPage />} />
        </>
    )
);
