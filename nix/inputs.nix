{ pkgs, ... }:

let
  runtimeInputs = with pkgs; [
    libxkbcommon
  ];
  buildInputs = with pkgs; [
    pkg-config
    openssl
    python310
    udev alsa-lib vulkan-loader
    xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr
    wayland
  ] ++ runtimeInputs;
in
{
  inherit buildInputs;
}