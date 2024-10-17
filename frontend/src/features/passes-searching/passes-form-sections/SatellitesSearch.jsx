import {useEffect, useRef, useState} from "react";
import axios from "axios";
import {baseURL} from "~/src/App";
import AutosuggestField from "~/src/shared/components/AutosuggestField";

const SatellitesSearch = ({selectedSatellites, setSelectedSatellites}) => {
    const [items, setItems] = useState([]);
    const [value, setValue] = useState("");

    const inputRef = useRef(null);

    useEffect(() => {
        axios.get(`${baseURL}/api/get-satellites-list`)
            .then(response => setItems(response.data))
            .catch(error => console.error(error));
    }, []);

    const inputProps = {
        placeholder: 'Найти спутник...',
        type: 'search',
        ref: inputRef,
    };

    const onSuggestionSelected = (event, {suggestion}) => {
        inputRef.current.value = "";
        setValue("");

        if (!selectedSatellites.includes(suggestion)) {
            setSelectedSatellites([...selectedSatellites, suggestion]);
        }
    };

    return <div className="flex flex-col items-start py-2 px-6 w-1/3">
        <h2 className="mb-6">Добавьте интерисующие вас спутники</h2>

        <AutosuggestField
            value={value}
            setValue={setValue}
            items={items}
            onSuggestionSelected={onSuggestionSelected}
            inputProps={inputProps}
            id="satellites-section"
            theme={{
                container: "satellites-section-container",
                input: "satellites-section-input",
                suggestionsContainer: "satellites-section-suggestions-container",
                suggestion: "satellites-section-suggestion",
                suggestionHighlighted: "satellites-section-suggestion-highlighted",
                suggestionsContainerOpen: "opened"
            }}
        />
    </div>
};

export default SatellitesSearch;
