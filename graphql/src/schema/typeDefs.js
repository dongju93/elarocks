const { gql } = require("apollo-server");

const typeDefs = gql`
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
    }
`;

module.exports = typeDefs;
