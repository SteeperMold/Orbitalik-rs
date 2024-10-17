import LabeledInput from "./LabeledInput";

const CoordsSection = ({coords}) => {
    return <div className="flex flex-col mr-6 w-1/3">
        <h2 className="mb-6">Координаты точки наблюдения</h2>

        <LabeledInput
            labeltext="Широта, °"
            defaultValue={coords.lat}
            name="lat" type="number"
            min="-90" max="90" step="0.000000000000001"
        />

        <LabeledInput
            labeltext="Долгота, °"
            defaultValue={coords.lon}
            name="lon" type="number"
            min="-180" max="180" step="0.000000000000001"
        />

        <LabeledInput
            labeltext="Высота над уровнем моря, м"
            defaultValue="0"
            name="alt" type="number"
            min="0" max="10000" step="1"
        />
    </div>;
};

export default CoordsSection;
