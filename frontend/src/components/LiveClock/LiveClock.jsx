import {useState, forwardRef, useImperativeHandle} from "react";
import {format, formatInTimeZone} from "date-fns-tz";

const LiveClock = forwardRef(({className, localTime = false}, ref) => {
    const [currentTime, setCurrentTime] = useState(new Date());

    useImperativeHandle(ref, () => ({
        update: () => setCurrentTime(new Date()),
    }));

    const time = localTime ? format(currentTime, "HH:mm:ss") : formatInTimeZone(currentTime, "UTC", "HH:mm:ss");

    return <th className={className}>{time}</th>
});

export default LiveClock;
