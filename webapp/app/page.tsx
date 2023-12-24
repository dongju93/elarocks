"use client";
import React, { useState } from "react";
import axios from "axios";
import {
    useQuery,
    QueryClient,
    QueryClientProvider,
    keepPreviousData,
} from "@tanstack/react-query";
import SearchArea from "./components/searchArea";
import Pagination from "./components/pagination";

interface SearchParams {
    selectedOption: string;
    startTime: string;
    endTime: string;
    perPage: number;
    before: string | null;
}

interface NetworkConnectionNode {
    agent_name: string;
    agent_id: string;
    event_action: string;
    utc_time: string;
    process_guid: string;
    process_id: number;
    image: string;
    user: string;
    protocol: string;
    initiated: boolean;
    source_is_ipv6: boolean;
    source_ip: string;
    source_hostname: string;
    source_port: number;
    destination_is_ipv6: boolean;
    destination_ip: string;
    destination_hostname: string;
    destination_port: number;
    destination_port_name: string;
}

interface NetworkConnectionEdge {
    cursor: string;
    node: NetworkConnectionNode;
}

const fetchNetworkData = async (queryParams: SearchParams) => {
    const response = await axios.post("/api/gql", queryParams);
    return response.data.data;
};

function Home() {
    const [searchParams, setSearchParams] = useState<SearchParams>({
        selectedOption: "ProcessCreateEve",
        startTime: new Date().toISOString(),
        endTime: new Date().toISOString(),
        perPage: 10,
        before: null,
    });

    // react-query main hook
    const { data, isLoading, error, isPlaceholderData } = useQuery({
        // key is core of react-hook
        queryKey: ["networkData", searchParams],
        // data fetch query
        queryFn: () => fetchNetworkData(searchParams),
        // STUDY!
        placeholderData: keepPreviousData,
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
            ...searchParams,
            selectedOption,
            startTime,
            endTime,
        });
    };

    if (isLoading) return <div>Loading...</div>;
    if (error) return <div>An error occurred: {error.message}</div>;

    return (
        <div>
            <SearchArea onSubmit={handleSearchSubmit} />
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
                            <tr key={edge.cursor}>
                                <td>{edge.node.agent_name}</td>
                                <td>{edge.node.agent_id}</td>
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
