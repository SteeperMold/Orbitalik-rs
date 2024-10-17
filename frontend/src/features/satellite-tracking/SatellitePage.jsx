import {useEffect, useState, useRef} from "react";
import {useSearchParams} from "react-router-dom";
import axios from "axios";
import {baseURL} from "~/src/App";
import CesiumViewer from "~/src/shared/components/CesiumViewer";
import PassesTable from "~/src/shared/components/PassesTable";
import SatelliteDataTable from "~/src/shared/components/SatelliteDataTable";

const SatellitePage = () => {
    const [searchParams, _] = useSearchParams();
    const [satelliteData, setSatelliteData] = useState(null);
    const [coords, setCoords] = useState(null);

    const updatableComponentsRef = useRef([]);

    useEffect(() => {
        navigator.geolocation.getCurrentPosition(position => {
            const {latitude, longitude} = position.coords;
            setCoords({lat: latitude, lon: longitude});
        });
    }, []);

    useEffect(() => {
        if (!coords) {
            return;
        }

        const params = {
            satellite_name: searchParams.get('name'),
            lat: coords.lat,
            lon: coords.lon,
            alt: 0,
        };

        axios.get(`${baseURL}/api/get-satellite-data`, {params: params})
            .then(response => setSatelliteData(response.data))
            .catch(error => console.error(error));
    }, [searchParams, coords]);

    useEffect(() => {
        const interval = setInterval(() => {
            updatableComponentsRef.current?.forEach(item => item.update());
        }, 1000);

        return () => clearInterval(interval);
    }, []);

    return satelliteData === null ? <h1 className="text-3xl">Загрузка...</h1> : (
        <div className="flex flex-row justify-between">
            <CesiumViewer
                ref={el => updatableComponentsRef.current[0] = el}
                trajectory={satelliteData.trajectory}
                observerPosition={coords}
                className="w-1/2 h-[91.5vh]"
            />

            <SatelliteDataTable
                ref={el => updatableComponentsRef.current[1] = el}
                satelliteData={satelliteData}
            />

            {satelliteData.passes.length === 0 && !satelliteData.is_geostationary && (
                <p className="w-1/3 text-center">В близжайшие сутки пролетов нет</p>
            )}
            {satelliteData.passes.length !== 0 && !satelliteData.is_geostationary && (
                <PassesTable
                    passes={satelliteData.passes}
                    className="w-[37.5%] h-full mr-4 table-auto border border-gray-600"
                />
            )}
        </div>
    );
};

export default SatellitePage;
