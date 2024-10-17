import {useState, useEffect, useRef} from "react";
import axios from "axios";
import {baseURL} from "~/src/App";
import lens_svg from "./lens.svg";
import AutosuggestField from "~/src/shared/components/AutosuggestField";

const Searchbar = () => {
    const [items, setItems] = useState([]);
    const [value, setValue] = useState("");

    const formRef = useRef(null);
    const inputRef = useRef(null);

    useEffect(() => {
        axios.get(`${baseURL}/api/get-satellites-list`)
            .then(response => setItems(response.data))
            .catch(error => console.error(error));
    }, []);

    const inputProps = {
        placeholder: 'Найти спутник...',
        type: 'search',
        name: "name",
        ref: inputRef,
    };

    const onSuggestionSelected = (event, {suggestion}) => {
        inputRef.current.value = suggestion;
        formRef.current.submit();
    };

    return <>
        <form ref={formRef} method="get" action="/satellite"
              className="flex items-center py-2 px-6 bg-gray-800 rounded-md">
            <AutosuggestField
                value={value}
                setValue={setValue}
                items={items}
                onSuggestionSelected={onSuggestionSelected}
                inputProps={inputProps}
                id="searchbar"
                theme={{
                    container: "searchbar-container",
                    input: "searchbar-input",
                    suggestionsContainer: "searchbar-suggestions-container",
                    suggestion: "searchbar-suggestion",
                    suggestionHighlighted: "searchbar-suggestion-highlighted"
                }}
            />
            <button type="submit" className="w-1/6 ml-4">
                <img src={lens_svg} alt="Найти"/>
            </button>
        </form>
    </>;
};

export default Searchbar;
