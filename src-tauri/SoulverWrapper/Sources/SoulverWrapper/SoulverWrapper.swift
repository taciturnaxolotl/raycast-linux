import SoulverCore
import Foundation

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
    guard let calculator = globalCalculator else {
        let errorMsg = "Error: SoulverCore not initialized. Call initialize_soulver() first."
        print("❌ Soulver Wrapper: \(errorMsg)")
        return strdup(errorMsg)
    }

    let swiftExpression = String(cString: expression)
    let result = calculator.calculate(swiftExpression)
    let resultString = result.stringValue
    
    return strdup(resultString)
}

@_cdecl("free_string")
public func free_string(ptr: UnsafeMutablePointer<CChar>?) {
    free(ptr)
}