import {Link} from "react-router-dom";
import Searchbar from "./Searchbar";

const Navbar = () => {
    return <nav className="flex items-center justify-between w-full mx-auto p-2 bg-gray-900">
        <div className="flex items-center justify-between">
            <Link to="/" className="text-3xl mr-8 p-4 hover:text-slate-400">Orbitalik</Link>
            <Link to="/passes" className="text-xl p-4 hover:text-slate-400">Пролетающие спутники</Link>
        </div>
        <div>
            <Searchbar/>
        </div>
    </nav>;
};

export default Navbar;
