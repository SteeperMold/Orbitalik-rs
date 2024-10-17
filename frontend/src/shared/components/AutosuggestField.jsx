import {useState} from "react";
import Autosuggest from "react-autosuggest";
import levenshtein from 'js-levenshtein';

const AutosuggestField = ({value, setValue, items, onSuggestionSelected, inputProps, id, theme}) => {
    const [suggestions, setSuggestions] = useState([]);

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

    const fullInputProps = {
        ...inputProps,
        onChange: (event, {newValue}) => setValue(newValue),
        value: value,
    }

    return <Autosuggest
        suggestions={suggestions}
        onSuggestionsFetchRequested={onSuggestionsFetchRequested}
        onSuggestionsClearRequested={() => setSuggestions([])}
        getSuggestionValue={suggestion => suggestion}
        onSuggestionSelected={onSuggestionSelected}
        renderSuggestion={suggestion => <div>{suggestion}</div>}
        inputProps={fullInputProps}
        id={id}
        theme={theme}
    />;
};

export default AutosuggestField