const { fetchKey } = require("../db");
const { fetchDataBasedOnTime } = require("../fetchData");

const resolvers = {
    Query: {
        RegValueSetEve: async (
            parent,
            { filter, pagination },
            context,
            info
        ) => {
            return fetchSysmonData(filter, "Registry value set", pagination);
        },
        ProcessCreateEve: async (
            parent,
            { filter, pagination },
            context,
            info
        ) => {
            return fetchSysmonData(filter, "Process Create", pagination);
        },
        NetworkConnectionEve: async (
            parent,
            { filter, pagination },
            context,
            info
        ) => {
            return fetchSysmonData(
                filter,
                "Network connection detected",
                pagination
            );
        },
    },
};

async function fetchSysmonData(filter, nodeType, pagination) {
    const { datetime, process_id, user, agent_id } = filter;
    const { start, end } = datetime;
    const filters = [];
    const allResults = [];
    // const DEFAULT_OFFSET = 0;
    const DEFAULT_LIMIT = 10;
    // const offset = pagination?.offset || DEFAULT_OFFSET;
    const afterCursor = pagination?.after;
    const limit = pagination?.limit || DEFAULT_LIMIT;
    const postgresResults = await fetchDataBasedOnTime(nodeType, start, end);

    if (process_id) {
        filters.push((result) => result.process_id == process_id);
    }

    if (user) {
        filters.push((result) => result.user == user);
    }

    if (agent_id) {
        filters.push(
            (result) => result.agent_id && result.agent_id.includes(agent_id)
        );
    }

    for (const row of postgresResults) {
        const key = `${nodeType}_${row.savedtime}`;
        const result = await fetchKey(key);
        // use every method to check filters is true
        if (result) {
            if (result.hashes) {
                result.hashes = result.hashes.split(",");
            }
            if (filters.every((filterFn) => filterFn(result))) {
                allResults.push(result);
            } else {
                allResults.push();
            }
        }
    }

    // Sort results by utc_time
    allResults.sort((a, b) => new Date(a.utc_time) - new Date(b.utc_time));

    // If an 'after' cursor is provided, slice results to start after that cursor
    let startIndex = 0;
    if (afterCursor) {
        startIndex =
            allResults.findIndex((item) => item.utc_time === afterCursor) + 1;
    }

    // Fetch 'limit + 1' items to check for next page (pageInfo)
    const results = allResults.slice(startIndex, startIndex + limit + 1);

    const edges = results.map((item) => ({
        cursor: item.utc_time,
        node: item,
    }));

    // Check if we have an extra item, indicating more items to fetch
    const hasNextPage = results.length > limit;
    if (hasNextPage) {
        edges.pop();
    }

    return {
        edges,
        pageInfo: {
            endCursor: edges[edges.length - 1]?.cursor || null,
            hasNextPage,
        },
        totalCount: allResults.length,
    };

    // console.log("Final allResults:", allResults);

    // switch (nodeType) {
    //     case "Registry value set":
    //         return {
    //             node: allResults.slice(offset, offset + limit),
    //             totalCount: allResults.length,
    //         };
    //     case "Process Create":
    //         return {
    //             node: allResults.slice(offset, offset + limit),
    //             totalCount: allResults.length,
    //         };
    //     case "Network connection detected":
    //         return {
    //             node: allResults.slice(offset, offset + limit),
    //             totalCount: allResults.length,
    //         };
    //     default:
    //         throw new Error("Invalid node type");
    // }
}

module.exports = resolvers;
