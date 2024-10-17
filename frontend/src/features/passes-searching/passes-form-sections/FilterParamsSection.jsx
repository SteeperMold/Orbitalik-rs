import LabeledInput from "./LabeledInput";

const FilterParamsSection = () => {
    return <div className="flex flex-col mx-6 w-1/3">
        <h2 className="mb-6">Параметры пролета</h2>

        <LabeledInput
            labeltext="Минимальная элевация, °"
            defaultValue="0"
            name="min_elevation" type="number"
            min="0" max="90" step="0.001"
        />

        <LabeledInput
            labeltext="Минимальный апогей, °"
            defaultValue="0"
            name="min_apogee" type="number"
            min="0" max="90" step="0.001"
        />
    </div>;
};

export default FilterParamsSection;
