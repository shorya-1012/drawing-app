import { createRoutesFromElements, createBrowserRouter, Route } from "react-router-dom";
import HomePage from "../pages/HomePage";
import DrawingPage from "../pages/Draw";

export const router = createBrowserRouter(
    createRoutesFromElements(
        <>
            <Route path="/" element={< HomePage />} />
            <Route path="/draw" element={< DrawingPage />} />
        </>
    )
);
