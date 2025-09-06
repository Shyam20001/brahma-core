import { describe, it, expect } from "vitest";
import { parseFile } from "@brahma-1/brahma-firelight/reinforcements";


describe("parseFile", () => {
    it("should return a String from a file path", async () => {

        // Act
        const result = parseFile('./rust-parse.txt');

        // Assert
        expect(result.toString()).toBe("This is parsed on the Rust via NAPI-FFI");
    });
});
