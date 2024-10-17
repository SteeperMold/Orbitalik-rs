import {forwardRef, useEffect, useImperativeHandle, useRef} from "react";
import {Entity, PointGraphics, PolylineGraphics, Viewer, BillboardCollection, Billboard} from "resium";
import {buildModuleUrl, Cartesian2, Cartesian3, Color, ImageryLayer, TileMapServiceImageryProvider} from "cesium";
import {isAfter, isBefore, differenceInSeconds, format} from "date-fns";
import {formatInTimeZone} from "date-fns-tz";

const generateBillboardSvgUrl = (label, time) => {
    const svg = `<svg id="Слой_1" xmlns="http://www.w3.org/2000/svg" x="0px" y="0px" 
        viewBox="0 0 100 100" overflow="visible">
        <path fill="#545454"
              stroke="#545454"
              stroke-miterlimit="10"
              d="M97.22,58.5H2.78c-1.26,0-2.28-1.02-2.28-2.28V2.78
            c0-1.26,1.02-2.28,2.28-2.28h94.44c1.26,0,2.28,1.02,2.28,2.28v53.44C99.5,57.48,98.48,58.5,97.22,58.5z"
        />
        <path fill="#545454"
              d="M38.5,58.5c1.33,6.67,2.67,13.33,4,20c7.67-6.67,15.33-13.33,23-20C56.5,58.5,47.5,58.5,38.5,58.5z"/>
        <text transform="matrix(1 0 0 1 9.8443 25.4443)"
              fill="#C4DDCF"
              font-family="'ArialMT'"
              font-size="13px">${label}</text>
        <text transform="matrix(1 0 0 1 23.7569 44.5775)"
              fill="#C4DDCF"
              font-family="'ArialMT'"
              font-size="16px">${time}</text>
    </svg>`;

    const blob = new Blob([svg], {type: 'image/svg+xml'});

    return URL.createObjectURL(blob);
};

const CesiumPassViewer = forwardRef(({className, trajectory, startTime, endTime, observerPosition}, ref) => {
    const satellitePointRef = useRef(null);

    useImperativeHandle(ref, () => ({
        update: () => {
            const currentTime = new Date();

            if (!(isAfter(currentTime, startTime) && isBefore(currentTime, endTime))) {
                satellitePointRef.current.cesiumElement.show = false;
                return;
            }

            const currentIndex = differenceInSeconds(currentTime, startTime);

            satellitePointRef.current?.cesiumElement.position.setValue(Cartesian3.fromDegrees(
                trajectory[currentIndex].lon,
                trajectory[currentIndex].lat,
            ));

            satellitePointRef.current.cesiumElement.show = true;
        },
    }));

    // useEffect(() => {
    //     document.querySelector('.cesium-widget-credits').remove();  // TODO вернуть
    // }, []);

    return <Viewer
        className={className}
        animation={false}
        timeline={false}
        baseLayerPicker={false}
        geocoder={false}
        baseLayer={
            ImageryLayer.fromProviderAsync(
                TileMapServiceImageryProvider.fromUrl(
                    buildModuleUrl('Assets/Textures/NaturalEarthII')
                )
            )
        }
    >
        <Entity
            name="Траектория спутника"
            description="Траектория спутника за то время, пока он будет виден из точки наблюдения"
        >
            <PolylineGraphics
                positions={trajectory.map(geodedic => Cartesian3.fromDegrees(geodedic.lon, geodedic.lat))}
                material={new Color(0, 1, 1, 1)}
                width={3}
            />
        </Entity>

        <Entity>
            <BillboardCollection>
                <Billboard
                    position={Cartesian3.fromDegrees(
                        trajectory[0].lon,
                        trajectory[0].lat,
                    )}
                    alignedAxis={Cartesian3.UNIT_Z}
                    image={generateBillboardSvgUrl("Начало пролета", format(startTime, "HH:mm:ss"))}
                    width={100} height={100}
                    pixelOffset={Cartesian2.fromElements(7, -25)}
                />

                <Billboard
                    position={Cartesian3.fromDegrees(
                        trajectory[trajectory.length - 1].lon,
                        trajectory[trajectory.length - 1].lat,
                    )}
                    alignedAxis={Cartesian3.UNIT_Z}
                    image={generateBillboardSvgUrl("Конец пролета", format(endTime, "HH:mm:ss"))}
                    width={100} height={100}
                    pixelOffset={Cartesian2.fromElements(7, -25)}
                />
            </BillboardCollection>
        </Entity>

        <Entity
            name="Текущее местонахождение спутника"
            ref={satellitePointRef}
            position={Cartesian3.fromDegrees(trajectory[0].lon, trajectory[0].lat)}
            show={false}
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
    </Viewer>
});

export default CesiumPassViewer;
