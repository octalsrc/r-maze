with import <nixpkgs> {};
with xorg;

writeText "piston-env" ''
${libX11}/lib:${libXcursor}/lib:${libXrandr}/lib:${libXi}/lib:${libglvnd}/lib
''
