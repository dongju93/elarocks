"use client";
import React, { useMemo, useState } from "react";
import axios, { AxiosError } from "axios";
import SearchArea from "./components/searchArea";
import Pagination from "./components/pagination";

export default function Home() {
    const [searchResults, setSearchResults] = useState<any[]>([]);
    const [totalCount, setTotalCount] = useState(0);
    const [currentPage, setCurrentPage] = useState(1);
    const [startCursor, setStartCursor] = useState<string | null>(null);
    const [endCursor, setEndCursor] = useState<string | null>(null);
    const [searchParams, setSearchParams] = useState({
        selectedOption: "",
        startTime: "",
        endTime: "",
    });
    const resultsPerPage = 10;

    const sortedResults = useMemo(() => {
        return searchResults.slice().sort((a, b) => {
            const dateA = new Date(a.node.utc_time).getTime();
            const dateB = new Date(b.node.utc_time).getTime();
            return dateB - dateA;
        });
    }, [searchResults]);

    const handleSearch = async ({
        selectedOption,
        startTime,
        endTime,
        cursor,
    }: {
        selectedOption: string;
        startTime: string;
        endTime: string;
        cursor?: string | null;
    }) => {
        setSearchParams({ selectedOption, startTime, endTime });

        try {
            const response = await axios.post("/api/gql", {
                selectedOption,
                startTime,
                endTime,
                before: cursor,
                perPage: resultsPerPage,
            });

            setSearchResults(response.data.data.NetworkConnectionEve.edges);
            setTotalCount(response.data.data.NetworkConnectionEve.totalCount);
            setStartCursor(
                response.data.data.NetworkConnectionEve.pageInfo.startCursor
            );
            setEndCursor(
                response.data.data.NetworkConnectionEve.pageInfo.endCursor
            );
        } catch (error) {
            if (axios.isAxiosError(error)) {
                console.error("Error response:", error.response);
            } else {
                console.error("Failed to fetch data:", error);
            }
        }
    };

    const handlePageChange = (newPage: number) => {
        setCurrentPage(newPage);
        let cursor = null;
        if (newPage === 1) {
            cursor = null;
        } else if (newPage == currentPage + 1) {
            cursor = startCursor;
        } else if (newPage == currentPage - 1) {
            cursor = endCursor;
        }

        handleSearch({ ...searchParams, cursor });
    };

    return (
        <div>
            <SearchArea onSubmit={handleSearch} />
            <div className="my-4 text-center dark:text-gray-300">
                Total Results: {totalCount}
            </div>
            <Pagination
                currentPage={currentPage}
                totalPages={Math.ceil(totalCount / resultsPerPage)}
                onPageChange={handlePageChange}
            />
            <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                <thead className="bg-gray-50 dark:bg-gray-700">
                    <tr>
                        <th>Agent Name</th>
                        <th>Agent ID</th>
                        <th>Event Action</th>
                        <th>UTC Time</th>
                        <th>Process GUID</th>
                        <th>Process ID</th>
                        <th>Image</th>
                        <th>User</th>
                        <th>Protocol</th>
                        <th>Initiated</th>
                        <th>Source is IPv6</th>
                        <th>Source IP</th>
                        <th>Source Hostname</th>
                        <th>Source Port</th>
                        <th>Destination is IPv6</th>
                        <th>Destination IP</th>
                        <th>Destination Hostname</th>
                        <th>Destination Port</th>
                        <th>Destination Port Name</th>
                    </tr>
                </thead>
                <tbody className="bg-white divide-y divide-gray-200 dark:bg-gray-800 dark:divide-gray-700">
                    {sortedResults.length > 0 ? (
                        sortedResults.map((edge) => (
                            <tr
                                key={edge.cursor}
                                className="dark:text-gray-300"
                            >
                                <td>{edge.node.agent_name}</td>
                                <td>{edge.node.agent_id}</td>
                                <td>{edge.node.event_action}</td>
                                <td>{edge.node.utc_time}</td>
                                <td>{edge.node.process_guid}</td>
                                <td>{edge.node.process_id}</td>
                                <td>{edge.node.image}</td>
                                <td>{edge.node.user}</td>
                                <td>{edge.node.protocol}</td>
                                <td>{edge.node.initiated.toString()}</td>
                                <td>{edge.node.source_is_ipv6.toString()}</td>
                                <td>{edge.node.source_ip}</td>
                                <td>{edge.node.source_hostname}</td>
                                <td>{edge.node.source_port}</td>
                                <td>
                                    {edge.node.destination_is_ipv6.toString()}
                                </td>
                                <td>{edge.node.destination_ip}</td>
                                <td>{edge.node.destination_hostname}</td>
                                <td>{edge.node.destination_port}</td>
                                <td>{edge.node.destination_port_name}</td>
                            </tr>
                        ))
                    ) : (
                        <tr>
                            <td
                                colSpan={19}
                                className="text-center py-4 dark:text-gray-300"
                            >
                                No results found
                            </td>
                        </tr>
                    )}
                </tbody>
            </table>
        </div>
    );
}
