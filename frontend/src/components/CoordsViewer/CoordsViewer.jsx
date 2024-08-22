import {useState, forwardRef, useImperativeHandle} from "react";
import {differenceInSeconds} from "date-fns";

const CoordsViewer = forwardRef(({className, trajectory}, ref) => {
    const [currentIndex, setCurrentIndex] = useState(trajectory.length / 2);
    const startTime = new Date();

    useImperativeHandle(ref, () => ({
        update: () => {
            setCurrentIndex(trajectory.length / 2 + differenceInSeconds(new Date(), startTime));
        },
    }));

    return <>
        <tr>
            <th className={className}>Широта</th>
            <th className={className}>{trajectory[currentIndex].lat.toFixed(3)} °</th>
        </tr>
        <tr>
            <th className={className}>Долгота</th>
            <th className={className}>{trajectory[currentIndex].lon.toFixed(3)} °</th>
        </tr>
        <tr>
            <th className={className}>Высота</th>
            <th className={className}>{trajectory[currentIndex].alt.toFixed(3)} км</th>
        </tr>
    </>;
});

export default CoordsViewer;
