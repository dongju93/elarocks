const { gql } = require("apollo-server");

const typeDefs = gql`
    type ProcessCreateEveResponse {
        Node: [ProcessCreateEve!]
        totalCount: Int
    }

    type ProcessCreateEve {
        agent_name: String!
        agent_id: String!
        event_action: String!
        utc_time: String!
        process_guid: String!
        process_id: String!
        image: String!
        file_version: String!
        description: String!
        product: String!
        company: String!
        original_file_name: String!
        command_line: String!
        user: String!
        logon_guid: String!
        logon_id: String!
        terminal_session_id: String!
        integrity_level: String!
        hashes: String!
        parent_process_guid: String!
        parent_process_id: String!
        parent_image: String!
        parent_command_line: String!
        parent_user: String!
    }

    type RegValueSetEveResponse {
        Node: [RegValueSetEve!]
        totalCount: Int
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

    type NetworkConnectionEveResponse {
        Node: [NetworkConnectionEve!]
        totalCount: Int
    }

    type NetworkConnectionEve {
        agent_name: String!
        agent_id: String!
        event_action: String!
        utc_time: String!
        process_guid: String!
        process_id: String!
        image: String!
        user: String!
        protocol: String!
        initiated: String!
        source_is_ipv6: String!
        source_ip: String!
        source_hostname: String!
        source_port: String!
        source_port_name: String!
        destination_is_ipv6: String!
        destination_ip: String!
        destination_hostname: String!
        destination_port: String!
        destination_port_name: String!
    }

    input DateTimeRange {
        start: String!
        end: String!
    }

    input SysmonFilter {
        event: String!
        datetime: DateTimeRange!
    }

    type Query {
        RegValueSetEve(filter: SysmonFilter!): RegValueSetEveResponse
        ProcessCreateEve(filter: SysmonFilter!): ProcessCreateEveResponse
        NetworkConnectionEve(filter: SysmonFilter!): NetworkConnectionEveResponse
    }
`;

module.exports = typeDefs;
