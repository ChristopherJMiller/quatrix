{ pkgs, ... }:

let
  runtimeInputs = with pkgs; [
    libxkbcommon
  ];

  buildInputs = with pkgs; [
    udev alsa-lib-with-plugins vulkan-loader
    xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
    libxkbcommon wayland # To use the wayland feature
  ] ++ runtimeInputs;
in
{
  inherit buildInputs runtimeInputs;
}