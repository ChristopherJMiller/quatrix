{ pkgs, ... }:

let
  buildInputs = with pkgs; [
    pkg-config
    openssl
    python310
    udev alsa-lib vulkan-loader
    xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr
    libxkbcommon wayland
  ];
in
{
  inherit buildInputs;
}