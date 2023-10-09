{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    # Всякие бинарные программы, типа компиляторов
    nativeBuildInputs = with pkgs; [
      rustc
      cargo
      protobuf
      cmake
      pkg-config
    ];
    # Всякие библиотечные зависимости
    buildInputs = with pkgs; [
       openssl
       zlib
       libiconv
       libobjc2
    ];
}