import type { NextApiRequest, NextApiResponse } from "next";

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

interface NetworkConnectionPageInfo {
    endCursor: string;
    hasNextPage: boolean;
    hasPreviousPage: boolean;
}

interface NetworkConnectionEve {
    totalCount: number;
    pageInfo: NetworkConnectionPageInfo;
    edges: NetworkConnectionEdge[];
}

interface GraphQLData {
    NetworkConnectionEve: NetworkConnectionEve;
}

interface GraphQLResponse {
    data: GraphQLData;
    errors?: any[]; // Adjust error typing as needed
}

type GraphQLQuery = {
    query: string;
    variables: {
        [key: string]: any; // or a more specific type based on your variables
    };
};

export default async function POST(
    req: NextApiRequest,
    res: NextApiResponse<GraphQLResponse | { message: string }>
) {
    res.setHeader("Access-Control-Allow-Origin", "*"); // Adjust as needed for your use case
    res.setHeader(
        "Access-Control-Allow-Methods",
        "GET, POST, PUT, DELETE, OPTIONS"
    );
    res.status(200).end();
    console.log(req.method);
    if (req.method === "POST") {
        const { startTime, endTime, last, before, selectedOption } = req.body;

        const graphqlQuery: GraphQLQuery = {
            query: `
              query GetNetworkConnectionEve($start: String!, $end: String!, $last: Int, $before: String) {
                  NetworkConnectionEve(
                      filter: {
                          datetime: {
                              start: $start,
                              end: $end
                          }
                      }
                      pagination: {
                          last: $last,
                      }
                  ) {
                      totalCount
                      pageInfo {
                          endCursor
                          hasNextPage
                          hasPreviousPage
                      }
                      edges {
                          cursor
                          node {
                              agent_name
                              agent_id
                              event_action
                              utc_time
                              process_guid
                              process_id
                              image
                              user
                              protocol
                              initiated
                              source_is_ipv6
                              source_ip
                              source_hostname
                              source_port
                              destination_is_ipv6
                              destination_ip
                              destination_hostname
                              destination_port
                              destination_port_name
                          }
                      }
                  }
              }
          `,
            variables: {
                start: startTime,
                end: endTime,
                last: 1,
            },
        };

        console.log(graphqlQuery);

        try {
            const response = await fetch("http://localhost:4000/", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(graphqlQuery),
            });

            const data: GraphQLResponse = await response.json();
            res.status(200).json(data);
        } catch (error) {
            res.status(500).json({ message: "Error fetching data" });
        }
    } else {
        res.status(405).end();
    }
}
