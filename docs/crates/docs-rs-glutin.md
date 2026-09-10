The purpose of this library is to provide an OpenGL context for as many platforms as possible, abstracting away the underlying differences without losing access to platform specific extensions.

However Glutin doesn’t force users into using the cross platform abstractions. When only a particular api is desired, it can be used directly.

The initialization starts by loading and connecting to the platform’s graphics Api when creating a display. This object is used to create all the OpenGL objects, such as config, context, and surface.
Environment variables

GLUTIN_WGL_OPENGL_DLL - change the name of the OpenGL DLL to load.
Modules

api
    The underlying OpenGL platform Api.
config
    Api config picking and creating utils.
context
    OpenGL context creation and initialization.
display
    The OpenGL platform display selection and creation.
error
    Glutin error handling.
platform
    Platform-specific API helpers.
prelude
    The glutin prelude.
surface
    A cross platform OpenGL surface representation.