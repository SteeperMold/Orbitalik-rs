import LabeledInput from "./LabeledInput";

const TimeSection = () => {
    return <div className="flex flex-col mx-6 w-1/3">
        <h2 className="mb-6">Время наблюдения</h2>

        <LabeledInput
            labeltext="Время начала наблюдения, UTC"
            defaultValue={new Date().toISOString().slice(0, 16)}
            name="start_time" type="datetime-local"
        />

        <LabeledInput
            labeltext="Длительность наблюдения в часах"
            defaultValue="24"
            name="duration" type="number"
            min="1" max="240" step="1"
        />
    </div>;
};

export default TimeSection;