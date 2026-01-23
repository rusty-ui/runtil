import AppKit
import CoreFoundation

struct State {
    var source: CFRunLoopSource;
    var loop: CFRunLoop;
}

@MainActor
class RuntilAppkitState {
    static let shared = RuntilAppkitState();
    
    var state: State?;
}

@MainActor
@_cdecl("runtilappkit_init")
func runtilAppkitInit(ud: UnsafeMutableRawPointer, callback: @convention(c) (UnsafeMutableRawPointer?) -> Void) {
    let loop = RunLoop.current.getCFRunLoop();
    
    assert(RuntilAppkitState.shared.state == nil, "RuntilAppkitState is already initialized");
    
    var source_cx = CFRunLoopSourceContext();
    source_cx.info = ud;
    source_cx.perform = callback;
    let source = CFRunLoopSourceCreate(nil, 1, &source_cx)!;
    CFRunLoopAddSource(loop, source, .commonModes)
    
    let app = NSApplication.shared;
    app.setActivationPolicy(.regular);
    
    
    RuntilAppkitState.shared.state = State(source: source, loop: loop)
}

@_cdecl("runtilappkit_schedule")
func runtilAppkitSchedule() {
    Task { @MainActor in
        guard let state = RuntilAppkitState.shared.state else {
            assertionFailure("RuntilAppkitState is not initialized")
            return
        }
        
        CFRunLoopSourceSignal(state.source);
        CFRunLoopWakeUp(state.loop);
    }
}

@MainActor
@_cdecl("runtilappkit_run")
func runtilAppkitRun() {
    guard let state = RuntilAppkitState.shared.state else {
        assertionFailure("RuntilAppkitState is not initialized")
        return
    }
    
    RunLoop.current.run()
}

@MainActor
@_cdecl("runtilappkit_destroy")
func runtilAppkitDestroy() {
    RuntilAppkitState.shared.state = nil
}

