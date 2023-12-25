const { execFile } = require("child_process");
const fs = require("fs");
const path = require("path");

// Executes the Rust binary and returns the parsed data
async function executeRustBinary(
    startKey,
    endKey,
    searchDirection,
    maxReturns,
    cursorValue,
    imageContains,
    pidMatch
) {
    return new Promise((resolve, reject) => {
        const filePath = path.join(__dirname, "nano-select");

        console.log("Executing:", filePath);

        const args = [
            startKey,
            endKey,
            searchDirection,
            maxReturns,
            cursorValue,
            imageContains,
            pidMatch,
        ];

        execFile(filePath, args, (error, stdout, stderr) => {
            if (error) {
                console.error("Command Error:", error);
                console.error("Standard Error Output:", stderr);
                reject(error);
                return;
            }

            try {
                const lines = stdout.trim().split("\n");
                const data = lines.map((line) => JSON.parse(line));
                resolve(data);
            } catch (parseError) {
                reject(parseError);
            }
        });
    });
}

// GraphQL resolvers
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

// Fetches Sysmon data using the Rust binary
async function fetchSysmonData(filter, eventType, pagination) {
    start_time = filter.datetime.start.replace("T", " ").replace("Z", "00000");
    end_time = filter.datetime.end.replace("T", " ").replace("Z", "99999");

    const startKey = `${eventType}_${start_time}`;
    const endKey = `${eventType}_${end_time}`;
    // first or last set to forward or reverse search
    let searchDirection = "first";
    if (pagination.first) {
        searchDirection = "first";
    } else if (pagination.last) {
        searchDirection = "last";
    }

    let maxReturns = pagination.first || 10;
    if (pagination.last && !pagination.first) {
        maxReturns = pagination.last;
    }

    let imageContains = filter.image || "";
    let pidMatch = filter.process_id;

    // first-after, last-before combination
    const cursorValue = pagination.after || pagination.before || "";

    try {
        let cursorForOffset = cursorValue;
        if (pagination.offset) {
            // Fetch the cursor corresponding to the offset position
            const offsetData = await executeRustBinary(
                startKey,
                endKey,
                searchDirection,
                pagination.offset, // offset to maxValue search
                "", // cursor must be empty
                imageContains,
                pidMatch
            );

            const offsetDataReturn = offsetData[0];
            const offsetPageInfoDataReturn =
                offsetDataReturn[offsetDataReturn.length - 1];
            // real-data cursor set
            cursorForOffset = offsetPageInfoDataReturn.end_cursor;
        }

        const rawData = await executeRustBinary(
            startKey,
            endKey,
            searchDirection,
            maxReturns,
            cursorForOffset,
            imageContains,
            pidMatch
        );

        const data = rawData[0];
        const pageInfoData = data[data.length - 1];

        // Extract the edges (all elements except the last one)
        const edges = data.slice(0, -1).map((item) => ({
            cursor: item.cursor,
            node: item.node,
        }));

        const pageInfo = {
            startCursor: pageInfoData.start_cursor,
            endCursor: pageInfoData.end_cursor,
            hasNextPage: pageInfoData.has_next_page,
            hasPreviousPage: pageInfoData.has_previous_page,
        };

        return {
            edges: edges,
            pageInfo: pageInfo,
            totalCount: pageInfoData.total_count,
        };
    } catch (error) {
        throw new Error(error);
    }
}

module.exports = { resolvers };
