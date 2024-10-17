import {useState, useEffect} from "react";
import CoordsSection from "./passes-form-sections/CoordsSection";
import TimeSection from "./passes-form-sections/TimeSection";
import FilterParamsSection from "./passes-form-sections/FilterParamsSection";
import SatellitesSection from "./passes-form-sections/SatellitesSection";

const PassesForm = () => {
    const [coords, setCoords] = useState(null);
    const [errorMessage, setErrorMessage] = useState("");

    useEffect(() => {
        navigator.geolocation.getCurrentPosition(
            position => {
                const {latitude, longitude} = position.coords;
                setCoords({lat: latitude, lon: longitude});
            }, () => {
                setCoords({lat: 0, lon: 0});
            }
        );
    }, []);

    if (!coords) {
        return;
    }

    const validateForm = event => {
        const formElements = event.target.elements;

        if (formElements.lat.value < -180 || formElements.lat.value > 180) {
            event.preventDefault();
            setErrorMessage("Широта должна быть от -180 до 180");
        }

        if (formElements.lon.value < -90 || formElements.lon.value > 90) {
            event.preventDefault();
            setErrorMessage("Долгота должна быть от -90 до 90");
        }

        if (formElements.alt.value < 0 || formElements.alt.value > 10000) {
            event.preventDefault();
            setErrorMessage("Высота над уровнем моря должна быть от 0 до 10000 метров");
        }

        if (isNaN(Date.parse(formElements.start_time.value))) {
            event.preventDefault();
            setErrorMessage("Неверный формат времени");
        }

        if (formElements.duration.value < 1 || formElements.duration.value > 240) {
            event.preventDefault();
            setErrorMessage("Длительность наблюдения должна быть от 1 часа до 240 часов");
        }

        if (formElements.min_elevation.value < 0 || formElements.min_elevation.value > 90) {
            event.preventDefault();
            setErrorMessage("Элевация должна быть от 0 до 90");
        }

        if (formElements.min_apogee.value < 0 || formElements.min_apogee.value > 90) {
            event.preventDefault();
            setErrorMessage("Апогей должен быть от 0 до 90");
        }

        if (!formElements.satellites.value) {
            event.preventDefault();
            setErrorMessage("Выберите хотя бы один спутник");
        }
    };

    return <>
        <h1 className="text-center mb-6 text-2xl mt-2 mb-16">Чтобы получить расписание пролетов, заполните все поля</h1>

        <form method="get" className="flex flex-col w-2/3 mx-auto justify-start" onSubmit={validateForm}>
            <h2 className="text-red-500 text-xl mb-10">{errorMessage}</h2>

            <div className="flex flex-row w-full">
                <CoordsSection coords={coords}/>
                <TimeSection/>
                <FilterParamsSection/>
            </div>

            <SatellitesSection/>

            <button className="mt-16">Рассчитать пролеты →</button>
        </form>
    </>;
};

export default PassesForm;