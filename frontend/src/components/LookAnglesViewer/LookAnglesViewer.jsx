import {useState, forwardRef, useImperativeHandle} from "react";
import {differenceInSeconds} from "date-fns";

const LookAnglesViewer = forwardRef(({className, lookAngles}, ref) => {
    const [currentIndex, setCurrentIndex] = useState(0);
    const startTime = new Date();

    useImperativeHandle(ref, () => ({
        update: () => {
            setCurrentIndex(differenceInSeconds(new Date(), startTime));
        },
    }));

    return <>
        <tr>
            <th className={className}>Азимут</th>
            <th className={className}>{lookAngles[currentIndex].az.toFixed(3)} °</th>
        </tr>
        <tr>
            <th className={className}>Элевация</th>
            <th className={className}>{lookAngles[currentIndex].el.toFixed(3)} °</th>
        </tr>
    </>;
});

export default LookAnglesViewer;
