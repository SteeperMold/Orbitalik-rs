const LabeledInput = inputProps => {
    const {labeltext} = inputProps;

    return <div className="w-full">
        <p>{labeltext}</p>
        <input
            {...inputProps} required
            className="w-full my-4 py-2 px-2 text-gray-400 bg-gray-800 rounded-md outline-none cursor-text"
        />
    </div>;
};

export default LabeledInput;
