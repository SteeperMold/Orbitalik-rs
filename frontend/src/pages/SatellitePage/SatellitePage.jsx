import {useEffect, useState, useRef} from "react";
import {useSearchParams} from "react-router-dom";
import axios from "axios";
import {format, parseISO, setDefaultOptions} from "date-fns";
import {formatInTimeZone} from "date-fns-tz";
import {ru} from "date-fns/locale";
import {baseURL} from "~/src/App";
import CesiumViewer from "~/src/components/CesiumViewer/CesiumViewer";
import LiveClock from "~/src/components/LiveClock/LiveClock";
import CoordsViewer from "~/src/components/CoordsViewer/CoordsViewer";
import LookAnglesViewer from "~/src/components/LookAnglesViewer/LookAnglesViewer";

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

    const thClassName = "border border-gray-600";

    return satelliteData === null ? <h1 className="text-3xl">Загрузка...</h1> : (
        <div className="flex flex-row justify-between">
            <CesiumViewer
                ref={el => updatableComponentsRef.current[0] = el}
                trajectory={satelliteData.trajectory}
                observerPosition={coords}
                className="w-1/2 h-[91.5vh]"
            />

            <table className="w-[12.5%] h-full mx-4 border border-gray-600 rounded-md">
                <thead>
                <tr>
                    <th colSpan="2" className={thClassName}>{searchParams.get('name')}</th>
                </tr>
                </thead>

                <tbody>
                <tr>
                    <th className="w-1/2 border border-gray-600">NORAD id</th>
                    <th className="w-1/2 border border-gray-600">{satelliteData.norad_id}</th>
                </tr>
                <tr>
                    <th className={thClassName}>Время, UTC</th>
                    <LiveClock
                        ref={el => updatableComponentsRef.current[3] = el}
                        className={thClassName}
                    />
                </tr>
                <tr>
                    <th className={thClassName}>Местное время</th>
                    <LiveClock
                        ref={el => updatableComponentsRef.current[4] = el}
                        localTime={true}
                        className={thClassName}
                    />
                </tr>
                <CoordsViewer
                    ref={el => updatableComponentsRef.current[1] = el}
                    trajectory={satelliteData.trajectory}
                    className="w-1/2 border border-gray-600"
                />
                <LookAnglesViewer
                    ref={el => updatableComponentsRef.current[2] = el}
                    lookAngles={satelliteData.look_angles}
                    className="w-1/2 border border-gray-600"
                />
                <tr>
                    <th className={thClassName}>Наклонение</th>
                    <th className={thClassName}>{satelliteData.inclination.toFixed(5)} °</th>
                </tr>
                <tr>
                    <th className={thClassName}>Эксцентриситет</th>
                    <th className={thClassName}>{satelliteData.eccentricity.toFixed(5)}</th>
                </tr>
                <tr>
                    <th className={thClassName}>Период обращения</th>
                    <th className={thClassName}>{satelliteData.period_minutes.toFixed(2)} мин.</th>
                </tr>
                <tr>
                    <th className={thClassName}>Среднее движение</th>
                    <th className={thClassName}>{satelliteData.mean_motion.toFixed(5)} °/с</th>
                </tr>
                <tr>
                    <th className={thClassName}>Аргумент перицентра</th>
                    <th className={thClassName}>{satelliteData.argument_of_pericenter.toFixed(5)} °</th>
                </tr>
                <tr>
                    <th className={thClassName}>Средняя аномалия</th>
                    <th className={thClassName}>{satelliteData.mean_anomaly.toFixed(5)} °</th>
                </tr>
                <tr>
                    <th className={thClassName}>Долгота восходящего узла</th>
                    <th className={thClassName}>{satelliteData.raan.toFixed(5)} °</th>
                </tr>
                <tr>
                    <th className={thClassName}>Эпоха</th>
                    <th className={thClassName}>
                        {formatInTimeZone(parseISO(satelliteData.epoch), "UTC", "dd-MM-yyyy hh:mm:ss 'UTC'")}
                    </th>
                </tr>
                </tbody>
            </table>

            {satelliteData.passes.length === 0 ?
                <p className="w-1/3 text-center">В близжайшие сутки пролетов нет</p> : (
                    <table className="w-[37.5%] h-full mr-4 table-auto border border-gray-600">
                        <thead className="h-1/4">
                        <tr>
                            <th colSpan="2" className="w-1/3 border border-gray-600">Начало пролета</th>
                            <th colSpan="3" className="w-1/3 border border-gray-600">Кульминация</th>
                            <th colSpan="2" className="w-1/3 border border-gray-600">Конец пролета</th>
                        </tr>
                        <tr>
                            <th className={thClassName}>Дата, местное время</th>
                            <th className={thClassName}>Аз</th>
                            <th className={thClassName}>Местное время</th>
                            <th className={thClassName}>Аз</th>
                            <th className={thClassName}>Эл</th>
                            <th className={thClassName}>Местное время</th>
                            <th className={thClassName}>Аз</th>
                        </tr>
                        </thead>

                        <tbody>
                        {satelliteData.passes.map((passData, index) => {
                            setDefaultOptions({locale: ru});

                            return <tr key={index}>
                                <th className={thClassName}>{format(parseISO(passData.rise_time), "dd LLL HH:mm:ss")}</th>
                                <th className={thClassName}>{passData.rise_azimuth.toFixed(2)}</th>
                                <th className={thClassName}>{format(parseISO(passData.apogee_time), "HH:mm:ss")}</th>
                                <th className={thClassName}>{passData.apogee_azimuth.toFixed(2)}</th>
                                <th className={thClassName}>{passData.apogee_elevation.toFixed(2)}</th>
                                <th className={thClassName}>{format(parseISO(passData.fall_time), "HH:mm:ss")}</th>
                                <th className={thClassName}>{passData.fall_azimuth.toFixed(2)}</th>
                            </tr>;
                        })}
                        </tbody>
                    </table>
                )}
        </div>
    );
};

export default SatellitePage;
