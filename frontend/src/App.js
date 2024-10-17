import {Route, Routes} from "react-router-dom";
import Navbar from "./features/navbar/Navbar";
import MainPage from "./features/main-page/MainPage";
import SatellitePage from "./features/satellite-tracking/SatellitePage";
import PassesSearchingPage from "./features/passes-searching/PassesSearchingPage";
import PassViewingPage from "./features/passes-viewing/PassViewingPage";
import "./index.css";

const App = () => {
    return <>
        <Navbar/>
        <Routes>
            <Route path="" element={<MainPage/>}/>
            <Route path="/satellite" element={<SatellitePage/>}/>
            <Route path="/passes" element={<PassesSearchingPage/>}/>
            <Route path="/pass" element={<PassViewingPage/>}/>
        </Routes>
    </>;
};

export default App;
export const baseURL = "http://127.0.0.1:8080";
