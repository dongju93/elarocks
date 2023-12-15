"use client";

import React, { useState } from "react";
import { format, subDays } from "date-fns";

type DateTimeInputProps = {
    id: string;
    label: string;
    value: Date;
    onChange: (newValue: Date) => void;
};

interface SearchAreaProps {
    onSubmit: (options: {
        selectedOption: string;
        startTime: string;
        endTime: string;
    }) => void;
}

const DateTimeInput: React.FC<DateTimeInputProps> = ({
    id,
    label,
    value,
    onChange,
}) => (
    <div className="mb-4">
        <label
            htmlFor={id}
            className="block text-gray-700 text-sm font-bold mb-2"
        >
            {label}
        </label>
        <input
            type="datetime-local"
            id={id}
            value={format(value, "yyyy-MM-dd'T'HH:mm")}
            onChange={(e) => onChange(new Date(e.target.value))}
            className="w-full p-2 border border-gray-300 rounded focus:border-blue-500 focus:outline-none text-black"
        />
    </div>
);

const SearchArea: React.FC<SearchAreaProps> = ({ onSubmit }) => {
    const [selectedOption, setSelectedOption] = useState("ProcessCreateEve");
    const [startTime, setStartTime] = useState(subDays(new Date(), 1));
    const [endTime, setEndTime] = useState(new Date());

    const handleOptionChange = (
        event: React.ChangeEvent<HTMLSelectElement>
    ) => {
        setSelectedOption(event.target.value);
    };

    const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        onSubmit({
            selectedOption,
            startTime: format(startTime, "yyyy-MM-dd HH:mm:ss"),
            endTime: format(endTime, "yyyy-MM-dd HH:mm:ss"),
        });
    };

    return (
        <form
            onSubmit={handleSubmit}
            className="max-w-lg mx-auto p-6 bg-white rounded-lg shadow-md"
        >
            <div className="mb-4">
                <label
                    htmlFor="combo-box"
                    className="block text-gray-700 text-sm font-bold mb-2"
                >
                    Select Events:
                </label>
                <select
                    id="combo-box"
                    value={selectedOption}
                    onChange={handleOptionChange}
                    className="w-full p-2 border border-gray-300 rounded focus:border-blue-500 focus:outline-none text-black"
                >
                    <option value="ProcessCreateEve">Process Create</option>
                    <option value="NetworkConnectionEve">
                        Network Connection
                    </option>
                    <option value="RegValueSetEve">Registry Value Set</option>
                </select>
            </div>
            <DateTimeInput
                id="start-time"
                label="Start Time:"
                value={startTime}
                onChange={setStartTime}
            />

            <DateTimeInput
                id="end-time"
                label="End Time:"
                value={endTime}
                onChange={setEndTime}
            />

            <button
                type="submit"
                className="w-full p-3 mt-4 text-white bg-emerald-600 rounded hover:bg-emerald-800 focus:outline-none focus:ring focus:ring-blue-200 focus:ring-opacity-50"
            >
                Search
            </button>
        </form>
    );
};

export default SearchArea;
