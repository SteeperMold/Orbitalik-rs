import {useEffect, useRef, forwardRef, useImperativeHandle} from "react";
import {Viewer, Entity, PolylineGraphics, PointGraphics,} from "resium";
import {Cartesian3, Color} from "cesium";
import {differenceInSeconds} from "date-fns";

const CesiumViewer = forwardRef(({className, trajectory, observerPosition}, ref) => {
    const satellitePointRef = useRef(null);
    const startTime = new Date();
    let currentIndex = trajectory.length / 2;

    useImperativeHandle(ref, () => ({
        update: () => {
            currentIndex = trajectory.length / 2 + differenceInSeconds(new Date(), startTime);

            satellitePointRef.current?.cesiumElement.position.setValue(Cartesian3.fromDegrees(
                trajectory[currentIndex].lon,
                trajectory[currentIndex].lat,
            ));
        },
    }));

    // useEffect(() => {
    //     document.querySelector('.cesium-widget-credits').remove();  // TODO вернуть
    // }, []);

    const initialSatellitePosition = Cartesian3.fromDegrees(
        trajectory[trajectory.length / 2].lon,
        trajectory[trajectory.length / 2].lat,
    );

    return <Viewer className={className} animation={false} timeline={false}>
        <Entity
            name="Траектория спутника"
            description="Траектория спутника за час до текущего времени, и на час вперед"
        >
            <PolylineGraphics
                positions={trajectory.map((geodedic) => Cartesian3.fromDegrees(geodedic.lon, geodedic.lat))}
                material={new Color(0, 1, 1, 1)}
                width={3}
            />
        </Entity>

        <Entity
            name="Текущее местонахождение спутника"
            ref={satellitePointRef}
            position={initialSatellitePosition}
        >
            <PointGraphics
                color={new Color(1, 0.6, 0, 1)}
                pixelSize={15}
            />
        </Entity>

        <Entity
            name="Точка наблюдения"
            position={Cartesian3.fromDegrees(observerPosition.lon, observerPosition.lat)}
        >
            <PointGraphics
                color={new Color(1, 0, 1, 1)}
                pixelSize={12}
            />
        </Entity>
    </Viewer>;
});

export default CesiumViewer;
