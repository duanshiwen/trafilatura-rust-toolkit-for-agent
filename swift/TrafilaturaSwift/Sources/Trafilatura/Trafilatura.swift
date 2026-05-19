import Foundation
import TrafilaturaFFI

public enum TrafilaturaError: Error, LocalizedError, Equatable {
    case extractionFailed(String)
    case unknownFailure

    public var errorDescription: String? {
        switch self {
        case .extractionFailed(let message):
            return message
        case .unknownFailure:
            return "未知提取错误"
        }
    }
}

public enum Trafilatura {
    public static func extractText(fromHTML html: String) throws -> String {
        try unwrap(trafilatura_extract_text(html))
    }

    public static func extractJSONForMCP(fromHTML html: String) throws -> String {
        try unwrap(trafilatura_extract_json_for_mcp(html))
    }

    public static func extract(fromHTML html: String, optionsJSON: String) throws -> String {
        try unwrap(trafilatura_extract_with_options_json(html, optionsJSON))
    }

    private static func unwrap(_ result: TrafilaturaResult) throws -> String {
        defer {
            trafilatura_free_result(result)
        }

        if let errorPointer = result.error {
            throw TrafilaturaError.extractionFailed(String(cString: errorPointer))
        }

        guard let dataPointer = result.data else {
            throw TrafilaturaError.unknownFailure
        }

        return String(cString: dataPointer)
    }
}
