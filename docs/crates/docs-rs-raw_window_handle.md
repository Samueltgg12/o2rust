Interoperability library for Rust Windowing applications.

This library provides standard types for accessing a window’s platform-specific raw window handle and platforms display handle. This does not provide any utilities for creating and managing windows; instead, it provides a common interface that window creation libraries (e.g. Winit, SDL) can use to easily talk with graphics libraries (e.g. gfx-hal).
Safety guarantees

Please see the docs of HasWindowHandle and HasDisplayHandle.
Platform handle initialization

Each platform handle struct is purposefully non-exhaustive, so that additional fields may be added without breaking backwards compatibility. Each struct provides an empty method that may be used along with the struct update syntax to construct it. See each specific struct for examples.
Display Handles

Some windowing systems use a separate display handle for some operations. The display usually represents a connection to some display server, but it is not necessarily tied to a particular window. See RawDisplayHandle for more details.
Structs

AndroidDisplayHandle
    Raw display handle for Android.
AndroidNdkWindowHandle
    Raw window handle for Android NDK.
AppKitDisplayHandle
    Raw display handle for AppKit.
AppKitWindowHandle
    Raw window handle for AppKit.
DisplayHandle
    The handle to the display controller of the windowing system.
DrmDisplayHandle
    Raw display handle for the Linux Kernel Mode Set/Direct Rendering Manager.
DrmWindowHandle
    Raw window handle for the Linux Kernel Mode Set/Direct Rendering Manager.
GbmDisplayHandle
    Raw display handle for the Linux Generic Buffer Manager.
GbmWindowHandle
    Raw window handle for the Linux Generic Buffer Manager.
HaikuDisplayHandle
    Raw display handle for Haiku.
HaikuWindowHandle
    Raw window handle for Haiku.
OhosDisplayHandle
    Raw display handle for OpenHarmony.
OhosNdkWindowHandle
    Raw window handle for Ohos NDK.
OrbitalDisplayHandle
    Raw display handle for the Redox operating system.
OrbitalWindowHandle
    Raw window handle for the Redox operating system.
UiKitDisplayHandle
    Raw display handle for UIKit.
UiKitWindowHandle
    Raw window handle for UIKit.
WaylandDisplayHandle
    Raw display handle for Wayland.
WaylandWindowHandle
    Raw window handle for Wayland.
WebCanvasWindowHandle
    Raw window handle for a Web canvas registered via wasm-bindgen.
WebDisplayHandle
    Raw display handle for the Web.
WebOffscreenCanvasWindowHandle
    Raw window handle for a Web offscreen canvas registered via wasm-bindgen.
WebWindowHandle
    Raw window handle for the Web.
Win32WindowHandle
    Raw window handle for Win32.
WinRtWindowHandle
    Raw window handle for WinRT.
WindowHandle
    The handle to a window.
WindowsDisplayHandle
    Raw display handle for Windows.
XcbDisplayHandle
    Raw display handle for Xcb.
XcbWindowHandle
    Raw window handle for Xcb.
XlibDisplayHandle
    Raw display handle for Xlib.
XlibWindowHandle
    Raw window handle for Xlib.

Enums

HandleError
    An error that can occur while fetching a display or window handle.
RawDisplayHandle
    A display server handle for a particular windowing system.
RawWindowHandle
    A window handle for a particular windowing system.

Traits

HasDisplayHandle
    A display that acts as a wrapper around a display handle.
HasRawDisplayHandleDeprecated
    Display that wraps around a raw display handle.
HasRawWindowHandleDeprecated
    Window that wraps around a raw window handle.
HasWindowHandle
    A handle to a window.