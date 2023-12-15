import React from "react";

interface PaginationProps {
    currentPage: number;
    totalPages: number;
    onPageChange: (page: number) => void;
}

const Pagination: React.FC<PaginationProps> = ({
    currentPage,
    totalPages,
    onPageChange,
}) => {
    const pageNumbers: any[] = [];

    pageNumbers.push(1);

    let startPage = Math.max(2, currentPage - 2);
    let endPage = Math.min(totalPages - 1, currentPage + 2);

    if (currentPage < 5) {
        endPage = Math.min(6, totalPages - 1);
    }
    if (totalPages - currentPage < 5) {
        startPage = Math.max(totalPages - 5, 2);
    }

    for (let i = startPage; i <= endPage; i++) {
        pageNumbers.push(i);
    }

    if (!pageNumbers.includes(totalPages)) {
        pageNumbers.push("...");
        pageNumbers.push(totalPages);
    }
    const handleClick = (page: string | number) => {
        if (page === "...") return;
        onPageChange(Number(page));
    };

    return (
        <div className="flex justify-center my-4">
            {pageNumbers.map((page, index) => (
                <button
                    key={index}
                    onClick={() => handleClick(page)}
                    disabled={page === "..."}
                    className={`mx-1 px-4 py-2 rounded ${
                        currentPage === page
                            ? "bg-blue-500 text-white"
                            : "bg-white dark:bg-gray-700 dark:text-gray-300"
                    }`}
                >
                    {page}
                </button>
            ))}
        </div>
    );
};

export default Pagination;
