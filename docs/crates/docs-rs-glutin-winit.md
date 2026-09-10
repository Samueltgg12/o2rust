This library provides helpers for cross-platform glutin bootstrapping with winit.
Structs

DisplayBuilder
    The helper to perform Display creation and OpenGL platform bootstrapping with the help of winit with little to no platform specific code.

Enums

ApiPreference
    Simplified version of the DisplayApiPreference which is used to simplify cross platform window creation.

Traits

GlWindow
    Window extensions for working with glutin surfaces.

Functions

finalize_window
    Finalize Window creation by applying the options from the Config, be aware that it could remove incompatible options from the window builder like transparency, when the provided config doesn’t support it.