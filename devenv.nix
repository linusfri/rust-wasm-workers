{ pkgs, lib, config, inputs, ... }:

{
  config = {
    packages = with pkgs; [ git nodejs_22 openssl ];

    languages = {
      rust = {
        enable = true;
        channel = "stable";
      };
      
      typescript.enable = true;
    };
  };
}
