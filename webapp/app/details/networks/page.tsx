"use client";
import React, { useEffect, useState } from "react";
import { useSearchParams } from "next/navigation";
import { NetworkConnectionNode } from "../../components/types";

const DetailsPage: React.FC = () => {
    const searchParams = useSearchParams();
    const [itemData, setItemData] = useState<NetworkConnectionNode | null>(
        null
    );

    useEffect(() => {
        // Check if searchParams is not null and get the 'data' parameter
        const dataParam = searchParams ? searchParams.get("data") : null;

        if (dataParam) {
            try {
                // Decode the URL-encoded string and parse the JSON
                const decodedData = decodeURIComponent(dataParam);
                const parsedData = JSON.parse(
                    decodedData
                ) as NetworkConnectionNode;
                setItemData(parsedData);
            } catch (error) {
                console.error("Error parsing item data:", error);
            }
        }
    }, [searchParams]);

    if (!itemData) {
        return <div>Loading...</div>;
    }

    return (
        <div>
            <h1>Details for Item ID: {itemData.agent_id}</h1>
            {/* Render other item details here */}
        </div>
    );
};

export default DetailsPage;
