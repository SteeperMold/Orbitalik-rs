import {useState, forwardRef, useImperativeHandle} from "react";
import {differenceInSeconds} from "date-fns";
import TableItem from "~/src/shared/components/TableItem";

const CoordsViewer = forwardRef(({trajectory}, ref) => {
    const [currentIndex, setCurrentIndex] = useState(trajectory.length / 2);
    const startTime = new Date();

    useImperativeHandle(ref, () => ({
        update: () => {
            setCurrentIndex(trajectory.length / 2 + differenceInSeconds(new Date(), startTime));
        },
    }));

    return <>
        <tr>
            <TableItem>Широта</TableItem>
            <TableItem>{trajectory[currentIndex].lat.toFixed(3)} °</TableItem>
        </tr>
        <tr>
            <TableItem>Долгота</TableItem>
            <TableItem>{trajectory[currentIndex].lon.toFixed(3)} °</TableItem>
        </tr>
        <tr>
            <TableItem>Высота</TableItem>
            <TableItem>{trajectory[currentIndex].alt.toFixed(3)} км</TableItem>
        </tr>
    </>;
});

export default CoordsViewer;
