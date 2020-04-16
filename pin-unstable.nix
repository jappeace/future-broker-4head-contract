# this allows caching of instability.
# making switches less annoying but also less up to date
import (
    builtins.fetchGit (
    {
        url = "https://github.com/NixOS/nixpkgs";
        ref = "master";
        rev = "b61999e4ad60c351b4da63ae3ff43aae3c0bbdfb";
    }
    ))
