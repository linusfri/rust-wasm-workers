{ pkgs, lib, config, inputs, ... }:

{
  config = {
    packages = with pkgs; [ git nodejs_22 openssl ];

    languages = {
      rust = {
        enable = true;
        channel = "stable";
        targets = [ "wasm32-unknown-unknown" ];
      };
      
      typescript.enable = true;
    };

    scripts.init-and-start.exec = "(cd frontend && npm install) && start";
    scripts.start.exec = "cd frontend && npm run dev";
  };
}
