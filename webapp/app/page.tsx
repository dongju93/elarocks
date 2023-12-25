"use client";
import React, { useState } from "react";
import axios from "axios";
import {
    useQuery,
    QueryClient,
    QueryClientProvider,
    keepPreviousData,
} from "@tanstack/react-query";
import Link from "next/link";
import SearchArea from "./components/searchArea";
import Pagination from "./components/pagination";
import { SearchParams, NetworkConnectionEdge } from "./components/types";

const fetchNetworkData = async (queryParams: SearchParams) => {
    const response = await axios.post("/api/gql", {
        ...queryParams,
        pagination: { last: queryParams.perPage, offset: queryParams.offset },
    });
    return response.data.data;
};

function Home() {
    const [currentPage, setCurrentPage] = useState(1);

    const [searchParams, setSearchParams] = useState<SearchParams>({
        selectedOption: "ProcessCreateEve",
        startTime: new Date().toISOString(),
        endTime: new Date().toISOString(),
        perPage: 10,
        offset: 0,
    });

    // react-query main hook
    const { data, isLoading, error } = useQuery({
        // key is core of react-hook
        queryKey: ["networkData", currentPage, searchParams],
        // data fetch query
        queryFn: () => fetchNetworkData(searchParams),
        // keep showing previous data when loading
        placeholderData: keepPreviousData,
        // if staleTime is over than stale
        staleTime: 10000,
        // refetch every interval
        // refetchInterval: 5000,
        // cachedTime
        gcTime: 8000,
    });

    const handleSearchSubmit = ({
        selectedOption,
        startTime,
        endTime,
    }: {
        selectedOption: string;
        startTime: string;
        endTime: string;
    }) => {
        setSearchParams({
            selectedOption,
            startTime,
            endTime,
            perPage: 10,
            offset: 0,
        });
        // setCurrentPage(1);
    };

    // offset calculate based on pages
    const handlePageChange = (newPage: number) => {
        const newOffset = (newPage - 1) * searchParams.perPage;
        setSearchParams({ ...searchParams, offset: newOffset });
        setCurrentPage(newPage);
    };

    if (isLoading) return <div>Loading...</div>;
    if (error) return <div>An error occurred: {error.message}</div>;

    return (
        <div>
            <SearchArea onSubmit={handleSearchSubmit} />
            <Pagination
                totalCount={data?.NetworkConnectionEve?.totalCount || 0}
                itemsPerPage={searchParams.perPage}
                currentPage={currentPage}
                onPageChange={handlePageChange}
            />

            <table>
                <thead>
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
                <tbody>
                    {data?.NetworkConnectionEve?.edges.map(
                        (edge: NetworkConnectionEdge) => (
                            <tr key={edge.cursor} style={{ cursor: "pointer" }}>
                                <td>{edge.node.agent_name}</td>
                                <td>
                                    <Link
                                        // node data pass to details page
                                        href={`/details/networks?data=${encodeURIComponent(
                                            JSON.stringify(edge.node)
                                        )}`}
                                    >
                                        {edge.node.agent_id}
                                    </Link>
                                </td>
                                <td>{edge.node.event_action}</td>
                                <td>{edge.node.utc_time}</td>
                                <td>{edge.node.process_guid}</td>
                                <td>{edge.node.process_id}</td>
                                <td>{edge.node.image}</td>
                                <td>{edge.node.user}</td>
                                <td>{edge.node.protocol}</td>
                                <td>{String(edge.node.initiated)}</td>
                                <td>{String(edge.node.source_is_ipv6)}</td>
                                <td>{edge.node.source_ip}</td>
                                <td>{edge.node.source_hostname}</td>
                                <td>{edge.node.source_port}</td>
                                <td>{String(edge.node.destination_is_ipv6)}</td>
                                <td>{edge.node.destination_ip}</td>
                                <td>{edge.node.destination_hostname}</td>
                                <td>{edge.node.destination_port}</td>
                                <td>{edge.node.destination_port_name}</td>
                            </tr>
                        )
                    )}
                </tbody>
            </table>
        </div>
    );
}

const queryClient = new QueryClient();

export default function HomePage() {
    return (
        <QueryClientProvider client={queryClient}>
            <Home />
        </QueryClientProvider>
    );
}
