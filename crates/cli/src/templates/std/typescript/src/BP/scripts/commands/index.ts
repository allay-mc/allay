import { StartupEvent, system } from "@minecraft/server";
import * as greet from "./greet.ts";

export function registerCustomCommands() {
  system.beforeEvents.startup.subscribe((init: StartupEvent) => {
    init.customCommandRegistry.registerCommand(greet.command, greet.callback);
    // Register more commands here...
  });
}
