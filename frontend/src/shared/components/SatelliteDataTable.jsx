import {forwardRef, useImperativeHandle, useRef} from "react";
import LiveClock from "./LiveClock";
import CoordsViewer from "./CoordsViewer";
import LookAnglesViewer from "./LookAnglesViewer";
import OrbitDataViewer from "./OrbitDataViewer";
import TableItem from "./TableItem";

const SatelliteDataTable = forwardRef(({satelliteData}, ref) => {
    const updatableComponentsRef = useRef([]);

    useImperativeHandle(ref, () => ({
        update: () => updatableComponentsRef.current?.forEach(item => item.update()),
    }));

    const className = `${satelliteData.is_geostationary ? 'w-1/2' : 'w-[12.5%]'} h-full mx-4 border border-gray-600 rounded-md`;

    return <table className={className}>
        <thead>
        <tr>
            <TableItem colSpan="2">{satelliteData.satellite_name}</TableItem>
        </tr>
        </thead>

        <tbody>
        <tr>
            <TableItem>NORAD id</TableItem>
            <TableItem>{satelliteData.norad_id}</TableItem>
        </tr>
        <LiveClock
            ref={el => updatableComponentsRef.current[0] = el}
        />
        <LiveClock
            ref={el => updatableComponentsRef.current[1] = el}
            localTime={true}
        />
        <CoordsViewer
            ref={el => updatableComponentsRef.current[2] = el}
            trajectory={satelliteData.trajectory}
            className="w-1/2 border border-gray-600"
        />
        <LookAnglesViewer
            ref={el => updatableComponentsRef.current[3] = el}
            lookAngles={satelliteData.look_angles}
            className="w-1/2 border border-gray-600"
        />
        <OrbitDataViewer
            satelliteData={satelliteData}
        />
        </tbody>
    </table>;
});

export default SatelliteDataTable;
