import SoulverCore
import Foundation

struct SoulverResult: Codable {
    let value: String
    let type: String
    let error: String?

    init(value: String, type: String, error: String? = nil) {
        self.value = value
        self.type = type
        self.error = error
    }
}


@MainActor
private var globalCalculator: Calculator?

@MainActor
@_cdecl("initialize_soulver")
public func initialize_soulver(resourcesPath: UnsafePointer<CChar>) {
    guard globalCalculator == nil else {
        print("Soulver Wrapper: Calculator already initialized.")
        return
    }

    let pathString = String(cString: resourcesPath)
    let resourcesURL = URL(fileURLWithPath: pathString)
    
    guard let soulverResourceBundle = SoulverCore.ResourceBundle(url: resourcesURL) else {
        print("❌ Soulver Wrapper: Failed to create SoulverCore.ResourceBundle from path: \(pathString)")
        return
    }
    
    let customization = EngineCustomization(
        resourcesBundle: soulverResourceBundle,
        locale: .autoupdatingCurrent
    )
    
    globalCalculator = Calculator(customization: customization)
    print("✅ Soulver calculator initialized with EngineCustomization using SoulverCore.ResourceBundle at \(resourcesURL.path)")
}

@MainActor
@_cdecl("evaluate")
public func evaluate(expression: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>? {
    let encoder = JSONEncoder()

    guard let calculator = globalCalculator else {
        let errorMsg = "Error: SoulverCore not initialized. Call initialize_soulver() first."
        print("❌ Soulver Wrapper: \(errorMsg)")
        let errorResult = SoulverResult(value: "", type: "error", error: errorMsg)
        if let jsonData = try? encoder.encode(errorResult), let jsonString = String(data: jsonData, encoding: .utf8) {
            return strdup(jsonString)
        }
        return nil
    }

    let swiftExpression = String(cString: expression)
    
    if swiftExpression.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
        let emptyResult = SoulverResult(value: "", type: "none", error: nil)
        if let jsonData = try? encoder.encode(emptyResult), let jsonString = String(data: jsonData, encoding: .utf8) {
            return strdup(jsonString)
        }
        return nil
    }

    let result = calculator.calculate(swiftExpression)
    
    if result.isEmptyResult {
         let emptyResult = SoulverResult(value: "", type: "none", error: nil)
         if let jsonData = try? encoder.encode(emptyResult), let jsonString = String(data: jsonData, encoding: .utf8) {
             return strdup(jsonString)
         }
         return nil
    }
    
    let resultType = String(describing: result.evaluationResult.equivalentTokenType)

    let soulverResult = SoulverResult(value: result.stringValue, type: resultType)
    
    if let jsonData = try? encoder.encode(soulverResult),
       let jsonString = String(data: jsonData, encoding: .utf8) {
        return strdup(jsonString)
    }
    
    let errorMsg = "Failed to encode Soulver result to JSON."
    let errorResult = SoulverResult(value: "", type: "error", error: errorMsg)
    if let jsonData = try? encoder.encode(errorResult), let jsonString = String(data: jsonData, encoding: .utf8) {
        return strdup(jsonString)
    }
    return nil
}

@_cdecl("free_string")
public func free_string(ptr: UnsafeMutablePointer<CChar>?) {
    free(ptr)
}