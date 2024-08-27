import {useState, useEffect, useRef} from "react";
import axios from "axios";
import Autosuggest from 'react-autosuggest';
import levenshtein from 'js-levenshtein';
import {baseURL} from "~/src/App";
import lens_svg from "./lens.svg";

const Searchbar = () => {
    const [items, setItems] = useState([]);
    const [suggestions, setSuggestions] = useState([]);
    const [value, setValue] = useState('');

    const formRef = useRef(null);
    const inputRef = useRef(null);

    useEffect(() => {
        axios.get(`${baseURL}/api/get-satellites-list`)
            .then(response => setItems(response.data))
            .catch(error => console.error(error));
    }, []);

    const onSuggestionsFetchRequested = ({value}) => {
        value = value.trim().toLowerCase();

        const suggestions = items
            .filter(item => item.toLowerCase().includes(value))
            .sort((left, right) => {
                const leftDistance = levenshtein(left.toLowerCase(), value);
                const rightDistance = levenshtein(right.toLowerCase(), value);

                return leftDistance - rightDistance;
            });

        setSuggestions(suggestions.slice(0, 8));
    };

    const inputProps = {
        placeholder: 'Найти спутник...',
        type: 'search',
        value: value,
        onChange: (event, {newValue}) => setValue(newValue),
        name: "name",
        ref: inputRef,
    };

    const onSuggestionSelected = (event, {suggestion}) => {
        inputRef.current.value = suggestion;
        formRef.current.submit();
    };

    return (
        <form ref={formRef} method="get" action="/satellite"
              className="flex items-center py-2 px-6 bg-gray-800 rounded-md">
            <Autosuggest
                suggestions={suggestions}
                onSuggestionsFetchRequested={onSuggestionsFetchRequested}
                onSuggestionsClearRequested={() => setSuggestions([])}
                getSuggestionValue={suggestion => suggestion}
                onSuggestionSelected={onSuggestionSelected}
                renderSuggestion={suggestion => <div>{suggestion}</div>}
                inputProps={inputProps}
            />
            <button type="submit" className="w-1/6 ml-4">
                <img src={lens_svg} alt="Найти"/>
            </button>
        </form>
    );
};

export default Searchbar;
