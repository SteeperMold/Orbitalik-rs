import {formatInTimeZone} from "date-fns-tz";
import {parseISO} from "date-fns";
import TableItem from "~/src/shared/components/TableItem";

const OrbitDataViewer = ({satelliteData}) => {
    return <>
        <tr>
            <th className="w-1/2 border border-gray-600">NORAD id</th>
            <th className="w-1/2 border border-gray-600">{satelliteData.norad_id}</th>
        </tr>
        <tr>
            <TableItem>Наклонение</TableItem>
            <TableItem>{satelliteData.inclination.toFixed(5)} °</TableItem>
        </tr>
        <tr>
            <TableItem>Эксцентриситет</TableItem>
            <TableItem>{satelliteData.eccentricity.toFixed(5)}</TableItem>
        </tr>
        <tr>
            <TableItem>Период обращения</TableItem>
            <TableItem>{satelliteData.period_minutes.toFixed(2)} мин.</TableItem>
        </tr>
        <tr>
            <TableItem>Среднее движение</TableItem>
            <TableItem>{satelliteData.mean_motion.toFixed(5)} °/с</TableItem>
        </tr>
        <tr>
            <TableItem>Аргумент перицентра</TableItem>
            <TableItem>{satelliteData.argument_of_pericenter.toFixed(5)} °</TableItem>
        </tr>
        <tr>
            <TableItem>Средняя аномалия</TableItem>
            <TableItem>{satelliteData.mean_anomaly.toFixed(5)} °</TableItem>
        </tr>
        <tr>
            <TableItem>Долгота восходящего узла</TableItem>
            <TableItem>{satelliteData.raan.toFixed(5)} °</TableItem>
        </tr>
        <tr>
            <TableItem>Эпоха</TableItem>
            <TableItem>
                {formatInTimeZone(parseISO(satelliteData.epoch), "UTC", "dd-MM-yyyy hh:mm:ss 'UTC'")}
            </TableItem>
        </tr>
    </>;
};

export default OrbitDataViewer;
