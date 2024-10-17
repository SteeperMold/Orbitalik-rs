import {useSearchParams} from "react-router-dom";
import PassesForm from "./PassesForm";
import PassesList from "./PassesList";

const PassesSearchingPage = () => {
    const [searchParams, _] = useSearchParams();

    if (searchParams.size === 0) {
        return <PassesForm/>;
    }

    const params = {
        lat: searchParams.get('lat'),
        lon: searchParams.get('lon'),
        alt: searchParams.get('alt'),
        min_elevation: searchParams.get('min_elevation'),
        min_apogee: searchParams.get('min_apogee'),
        start_time: searchParams.get('start_time'),
        duration: searchParams.get('duration'),
        satellites: searchParams.get('satellites'),
    };

    return <PassesList params={params}/>
};

export default PassesSearchingPage;
