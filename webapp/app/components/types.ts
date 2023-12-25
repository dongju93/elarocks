export interface NetworkConnectionNode {
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

export interface NetworkConnectionEdge {
    cursor: string;
    node: NetworkConnectionNode;
}

export interface SearchParams {
    selectedOption: string;
    startTime: string;
    endTime: string;
    perPage: number;
    offset: number;
}

export interface PageInfo {
    startCursor: string;
    endCursor: string;
    hasNextPage: boolean;
    hasPreviousPage: boolean;
}

export interface NetworkConnectionEve {
    totalCount: number;
    pageInfo: PageInfo;
    edges: NetworkConnectionEdge[];
}

export interface GraphQLData {
    NetworkConnectionEve: NetworkConnectionEve;
}

export interface GraphQLResponse {
    data: GraphQLData;
    errors?: any[];
}

export type GraphQLQuery = {
    query: string;
    variables: {
        [key: string]: any;
    };
};

export interface PaginationProps {
    totalCount: number;
    itemsPerPage: number;
    currentPage: number;
    onPageChange: (page: number) => void;
}
