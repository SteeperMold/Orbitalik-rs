import {useState, forwardRef, useImperativeHandle} from "react";
import {format, formatInTimeZone} from "date-fns-tz";
import TableItem from "~/src/shared/components/TableItem";

const LiveClock = forwardRef(({localTime = false}, ref) => {
    const [currentTime, setCurrentTime] = useState(new Date());

    useImperativeHandle(ref, () => ({
        update: () => setCurrentTime(new Date()),
    }));

    const time = localTime ? format(currentTime, "HH:mm:ss") : formatInTimeZone(currentTime, "UTC", "HH:mm:ss");

    return <tr>
        {localTime ? <TableItem>Местное время</TableItem> : <TableItem>Время, UTC</TableItem>}
        <TableItem>{time}</TableItem>
    </tr>;
});

export default LiveClock;
