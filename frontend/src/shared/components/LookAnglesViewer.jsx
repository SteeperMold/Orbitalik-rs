import {useState, forwardRef, useImperativeHandle} from "react";
import {differenceInSeconds} from "date-fns";
import TableItem from "~/src/shared/components/TableItem";

const LookAnglesViewer = forwardRef(({lookAngles}, ref) => {
    const [currentIndex, setCurrentIndex] = useState(0);
    const startTime = new Date();

    useImperativeHandle(ref, () => ({
        update: () => {
            setCurrentIndex(differenceInSeconds(new Date(), startTime));
        },
    }));

    return <>
        <tr>
            <TableItem>Азимут</TableItem>
            <TableItem>{lookAngles[currentIndex].az.toFixed(3)} °</TableItem>
        </tr>
        <tr>
            <TableItem>Элевация</TableItem>
            <TableItem>{lookAngles[currentIndex].el.toFixed(3)} °</TableItem>
        </tr>
    </>;
});

export default LookAnglesViewer;
