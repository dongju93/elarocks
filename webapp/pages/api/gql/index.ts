import type { NextApiRequest, NextApiResponse } from "next";
import axios from "axios";
import {
    // NetworkConnectionNode,
    // NetworkConnectionEdge,
    // PageInfo,
    // NetworkConnectionEve,
    // GraphQLData,
    GraphQLResponse,
    GraphQLQuery,
} from "../../../app/components/types";

export default async function POST(
    req: NextApiRequest,
    res: NextApiResponse<GraphQLResponse | { message: string }>
) {
    if (req.method === "POST") {
        // console.log("Request body:", req.body);
        // cursor is handle with api, front-end just only pass offset
        const { startTime, endTime, perPage, cursor, offset, selectedOption } =
            req.body;

        const graphqlQuery: GraphQLQuery = {
            query: `
              query getRawEvents($start: String!, $end: String!, $last: Int, $offset: Int) {
                NetworkConnectionEve(
                      filter: {
                          datetime: {
                              start: $start,
                              end: $end
                          }
                      }
                      pagination: {
                          last: $last,
                          offset: $offset
                      }
                  ) {
                      totalCount
                      pageInfo {
                          startCursor
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
                last: perPage,
                offset: offset,
            },
        };
        console.log(graphqlQuery);

        try {
            const response = await axios.post(
                "http://localhost:4000/",
                JSON.stringify(graphqlQuery),
                {
                    headers: {
                        "Content-Type": "application/json",
                    },
                }
            );
            res.status(200).json(response.data);
        } catch (error) {
            if (error instanceof Error) {
                console.error(error.message);
                res.status(500).json({ message: error.message });
            } else {
                console.error("An unknown error occurred");
                res.status(500).json({ message: "An unknown error occurred" });
            }
        }
    }
}
