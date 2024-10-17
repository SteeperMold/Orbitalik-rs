import {useState} from "react";
import SatellitesSearch from "./SatellitesSearch";
import cross_svg from "./cross.svg";

const SatellitesSection = () => {
    const [selectedSatellites, setSelectedSatellites] = useState([]);

    const deleteSatellite = satelliteName => {
        setSelectedSatellites(
            selectedSatellites.filter(name => name !== satelliteName)
        );
    };

    return <div className="flex flex-row justify-between">
        <input name="satellites" type="hidden" value={selectedSatellites.join(",")}/>

        <SatellitesSearch
            selectedSatellites={selectedSatellites}
            setSelectedSatellites={setSelectedSatellites}
        />

        <div className="flex flex-row flex-wrap w-2/3 mt-6">
            {selectedSatellites.map((satellite, index) => {
                return <div className="flex flex-row w-1/4 justify-between items-center" key={index}>
                    <p>{satellite}</p>
                    <button className="w-[8%] mr-4" type="button" onClick={() => deleteSatellite(satellite)}>
                        <img src={cross_svg} alt="Удалить"/>
                    </button>
                </div>;
            })}
        </div>

    </div>;
};

export default SatellitesSection;
