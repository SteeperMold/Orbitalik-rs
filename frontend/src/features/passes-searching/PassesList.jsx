import {useEffect, useState} from "react";
import axios from "axios";
import {format, parseISO, setDefaultOptions} from "date-fns";
import {ru} from "date-fns/locale";
import {baseURL} from "~/src/App";
import PassesTable from "~/src/shared/components/PassesTable";

const PassesList = ({params}) => {
    const [passData, setPassData] = useState(null);

    useEffect(() => {
        axios.get(`${baseURL}/api/get-passes-list`, {params: params})
            .then(response => setPassData(response.data))
            .catch(error => setPassData({errorType: error.response.data}));
    }, []);

    setDefaultOptions({locale: ru})
    const formatString = "dd LLL HH:mm:ss";

    const startDate = parseISO(params.start_time);
    const startDateFormatted = format(parseISO(params.start_time), formatString);
    const endDateFormatted = format(startDate.setHours(startDate.getHours() + parseInt(params.duration)), formatString);

    return <>
        {!passData && <h1 className="text-3xl">Загрузка...</h1>}
        {passData?.errorType && (
            <div className="text-center mt-16">
                <h1 className="text-3xl italic">Упс!</h1>
                <h1 className="text-3xl mt-6">Похоже что-то пошло не так</h1>
            </div>
        )}
        {passData && !passData.errorType && <>
            <h1 className="text-3xl text-center mt-4 mb-8">
                Пролеты указанных спутников на период с {startDateFormatted}, UTC по {endDateFormatted}, UTC
            </h1>

            <PassesTable
                passes={passData} params={params}
                doShowName={true} doShowTrackButton={true}
                className="mx-auto w-2/3 h-full table-auto border border-gray-600 mb-10"
            />
        </>}
    </>;
};

export default PassesList;
