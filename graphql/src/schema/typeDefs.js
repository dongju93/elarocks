const { gql } = require("apollo-server");
const typeDefs = gql`
    # query type
    scalar DateTime
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
        user: String
        agent_id: String
    }

    # node, paginaion(offset, limit)
    type ProcessCreateEveConnection {
        node: [ProcessCreateEve!]
        totalCount: Int
    }

    type RegValueSetEveConnection {
        node: [RegValueSetEve!]
        totalCount: Int
    }

    type NetworkConnectionEveConnection {
        node: [NetworkConnectionEve!]
        totalCount: Int
    }

    input PaginationInput {
        offset: Int
        limit: Int
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
