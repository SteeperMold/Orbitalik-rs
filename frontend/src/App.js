import {Route, Routes} from "react-router-dom";
import MainPage from "./pages/MainPage/MainPage";
import SatellitePage from "./pages/SatellitePage/SatellitePage";
import Navbar from "./components/Navbar/Navbar";
import "./index.css";

const App = () => {
    return <>
        <Navbar/>
        <Routes>
            <Route path="" element={<MainPage/>}/>
            <Route path="/satellite" element={<SatellitePage/>}/>
        </Routes>
    </>;
};

export default App;
export const baseURL = "http://127.0.0.1:8080";