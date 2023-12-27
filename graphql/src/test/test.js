// TEST
// data fetching rocksDB using binary
const { execFile } = require("child_process");

// Your provided function
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

        execFile(
            filePath,
            [
                startKey,
                endKey,
                searchDirection,
                maxReturns,
                cursorValue,
                imageContains,
                pidMatch,
            ],
            (error, stdout, stderr) => {
                if (error) {
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
            }
        );
    });
}

// Example usage of the function
async function runExample() {
    try {
        // Replace these values with actual parameters relevant to your use case
        const startKey =
            "Network connection detected_2021-09-07 00:59:58.09100000";
        const endKey =
            "Network connection detected_2023-09-08 01:59:58.09100000";
        const searchDirection = "first";
        const maxReturns = "10";
        const cursorValue = "";
        const imageContains = "";
        const pidMatch = "";

        const result = await executeRustBinary(
            startKey,
            endKey,
            searchDirection,
            maxReturns,
            cursorValue,
            imageContains,
            pidMatch
        );

        console.log("Result:", result);
    } catch (error) {
        console.error("Error:", error);
    }
}

// Run the example
runExample();
