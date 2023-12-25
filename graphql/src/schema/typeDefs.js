const { gql } = require("apollo-server");
const typeDefs = gql`
    # query type
    type ProcessCreateEve {
        agent_name: String!
        agent_id: String!
        event_action: String!
        utc_time: String!
        process_guid: String!
        process_id: Int!
        image: String!
        file_version: String!
        description: String!
        product: String!
        company: String!
        original_file_name: String!
        command_line: String!
        user: String!
        logon_guid: String!
        logon_id: Int!
        terminal_session_id: Int!
        integrity_level: String!
        hashes: [String!]
        parent_process_guid: String!
        parent_process_id: Int!
        parent_image: String!
        parent_command_line: String!
        parent_user: String!
    }

    type RegValueSetEve {
        agent_name: String!
        agent_id: String!
        event_action: String!
        event_type: String!
        utc_time: String!
        process_guid: String!
        process_id: Int!
        image: String!
        target_object: String!
        details: String!
        user: String!
    }

    type NetworkConnectionEve {
        agent_name: String!
        agent_id: String!
        event_action: String!
        utc_time: String!
        process_guid: String!
        process_id: Int!
        image: String!
        user: String!
        protocol: String!
        initiated: Boolean!
        source_is_ipv6: Boolean!
        source_ip: String!
        source_hostname: String!
        source_port: Int!
        source_port_name: String!
        destination_is_ipv6: Boolean!
        destination_ip: String!
        destination_hostname: String!
        destination_port: Int!
        destination_port_name: String!
    }

    # input filter
    input DateTimeRange {
        start: String!
        end: String!
    }

    input SysmonFilter {
        datetime: DateTimeRange!
        process_id: Int
        # user: String
        # agent_id: String
        image: String
    }

    # node with edges, pagination(cursor based)
    type PageInfo {
        startCursor: String
        endCursor: String
        hasNextPage: Boolean!
        hasPreviousPage: Boolean!
    }

    type ProcessCreateEveConnection {
        edges: [ProcessCreateEveEdge!]
        pageInfo: PageInfo!
        totalCount: Int
    }

    type ProcessCreateEveEdge {
        cursor: String!
        node: ProcessCreateEve!
    }

    type RegValueSetEveConnection {
        edges: [RegValueSetEveEdge!]
        pageInfo: PageInfo!
        totalCount: Int
    }

    type RegValueSetEveEdge {
        cursor: String!
        node: RegValueSetEve!
    }

    type NetworkConnectionEveConnection {
        edges: [NetworkConnectionEveEdge!]
        pageInfo: PageInfo!
        totalCount: Int
    }

    type NetworkConnectionEveEdge {
        cursor: String!
        node: NetworkConnectionEve!
    }

    input PaginationInput {
        first: Int
        last: Int
        before: String
        after: String
        offset: Int
    }

    type Query {
        RegValueSetEve(
            filter: SysmonFilter!
            pagination: PaginationInput
        ): RegValueSetEveConnection
        ProcessCreateEve(
            filter: SysmonFilter!
            pagination: PaginationInput
        ): ProcessCreateEveConnection
        NetworkConnectionEve(
            filter: SysmonFilter!
            pagination: PaginationInput
        ): NetworkConnectionEveConnection
    }
`;

module.exports = typeDefs;
