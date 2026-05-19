// swift-tools-version: 5.9

import PackageDescription

let package = Package(
    name: "TrafilaturaSwift",
    platforms: [
        .macOS(.v12)
    ],
    products: [
        .library(
            name: "Trafilatura",
            targets: ["Trafilatura"]
        )
    ],
    targets: [
        .binaryTarget(
            name: "TrafilaturaFFI",
            path: "Frameworks/TrafilaturaFFI.xcframework"
        ),
        .target(
            name: "Trafilatura",
            dependencies: ["TrafilaturaFFI"]
        )
    ]
)
