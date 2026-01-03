import AppKit
import CoreFoundation

@MainActor
class RuntilAppkitState {
    static let shared = RuntilAppkitState()

    var loop: CFRunLoop?;
}

@_cdecl("runtilappkit_init")
func RuntilAppkitInit() {
    let loop = RunLoop.current.getCFRunLoop();
    Task {@MainActor in
        assert(RuntilAppkitState.shared.loop == nil, "RuntilAppkitState is already initialized")
        
        let app = NSApplication.shared
        app.setActivationPolicy(.regular)
        
        RuntilAppkitState.shared.loop = loop
    }
}

@_cdecl("runtilappkit_run")
func RuntilAppkitRun() {
    Task { @MainActor in
        guard let loop = RuntilAppkitState.shared.loop else {
            assertionFailure("RuntilAppkitState is not initialized")
            return
        }

        CFRunLoopPerformBlock(loop, CFRunLoopMode.commonModes.rawValue) {
            //
        }
        CFRunLoopWakeUp(loop)
    }
    
    RunLoop.current.run()
}

@_cdecl("runtilappkit_destroy")
func RuntilAppkitDestroy() {
    Task { @MainActor in
        RuntilAppkitState.shared.loop = nil
    }
}

