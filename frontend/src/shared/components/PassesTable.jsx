import {Link} from "react-router-dom";
import {format, parseISO, setDefaultOptions} from "date-fns";
import {formatInTimeZone} from "date-fns-tz";
import {ru} from "date-fns/locale";
import TableItem from "./TableItem";

const PassesTable = ({passes, params = {}, className, doShowName = false, doShowTrackButton = false}) => {
    return <table className={className}>
        <thead>
        <tr>
            {doShowName && <TableItem rowSpan="2">Название спутника</TableItem>}
            <TableItem colSpan="2">Начало пролета</TableItem>
            <TableItem colSpan="3">Кульминация</TableItem>
            <TableItem colSpan="2">Конец пролета</TableItem>
            {doShowTrackButton && <TableItem rowSpan="2">Скачать расписание</TableItem>}
        </tr>
        <tr>
            <TableItem>Дата, местное время</TableItem>
            <TableItem>Аз</TableItem>
            <TableItem>Местное время</TableItem>
            <TableItem>Аз</TableItem>
            <TableItem>Эл</TableItem>
            <TableItem>Местное время</TableItem>
            <TableItem>Аз</TableItem>
        </tr>
        </thead>

        <tbody>
        {passes.map((passData, index) => {
            setDefaultOptions({locale: ru});

            const riseTime = parseISO(passData.rise_time);
            const fallTime = parseISO(passData.fall_time);

            return <tr key={index}>
                {doShowName && <TableItem>{passData.satellite_name}</TableItem>}
                <TableItem>{format(riseTime, "dd LLL HH:mm:ss")}</TableItem>
                <TableItem>{passData.rise_azimuth.toFixed(2)}</TableItem>
                <TableItem>{format(parseISO(passData.apogee_time), "HH:mm:ss")}</TableItem>
                <TableItem>{passData.apogee_azimuth.toFixed(2)}</TableItem>
                <TableItem>{passData.apogee_elevation.toFixed(2)}</TableItem>
                <TableItem>{format(fallTime, "HH:mm:ss")}</TableItem>
                <TableItem>{passData.fall_azimuth.toFixed(2)}</TableItem>
                {doShowTrackButton && <TableItem>
                    <Link to={
                        `/pass?satellite=${passData.satellite_name}&` +
                        `lat=${params.lat}&lon=${params.lon}&alt=${params.alt}&` +
                        `start_time=${formatInTimeZone(riseTime, "UTC", "yyyy-MM-dd'T'HH:mm")}&` +
                        `end_time=${formatInTimeZone(fallTime, "UTC", "yyyy-MM-dd'T'HH:mm")}`
                    }>Отслеживать</Link>
                </TableItem>}
            </tr>;
        })}
        </tbody>
    </table>;
};

export default PassesTable;
