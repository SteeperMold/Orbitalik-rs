import {useSearchParams} from "react-router-dom";
import {useEffect, useRef, useState} from "react";
import axios from "axios";
import {fromZonedTime} from "date-fns-tz";
import {baseURL} from "~/src/App";
import CesiumPassViewer from "./CesiumPassViewer";
import PassProjectionViewer from "./PassProjectionViewer";
import SatelliteDataTable from "~/src/shared/components/SatelliteDataTable";

const PassViewingPage = () => {
    const [searchParams, _] = useSearchParams();
    const [satelliteData, setSatelliteData] = useState(null);

    const updatableComponentsRef = useRef([]);

    useEffect(() => {
        axios.get(`${baseURL}/api/get-trajectory`, {params: searchParams})
            .then(response => setSatelliteData(response.data))
            .catch(error => console.error(error));
    }, []);

    useEffect(() => {
        const interval = setInterval(() => {
            updatableComponentsRef.current?.forEach(item => item.update());
        }, 1000);

        return () => clearInterval(interval);
    }, []);

    console.log(satelliteData)

    return !satelliteData ? <h1 className="text-center text-3xl mt-6">Загрузка...</h1> : <>
        <div className="flex flex-row justify-between">
            <CesiumPassViewer
                ref={el => updatableComponentsRef.current[0] = el}
                trajectory={satelliteData.trajectory}
                startTime={fromZonedTime(new Date(searchParams.get('start_time')), "UTC")}
                endTime={fromZonedTime(new Date(searchParams.get('end_time')), "UTC")}
                observerPosition={{
                    lat: parseFloat(searchParams.get('lat')),
                    lon: parseFloat(searchParams.get('lon'))
                }}
                className="w-[40%] h-[91.5vh]"
            />

            <PassProjectionViewer

            />

            <SatelliteDataTable
                satelliteData={satelliteData}
            />
        </div>
    </>;
};

export default PassViewingPage;
