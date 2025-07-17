import {
  world,
  CustomCommand,
  CommandPermissionLevel,
  CustomCommandParamType,
  CustomCommandStatus,
  CustomCommandResult,
  CustomCommandOrigin,
  Player,
} from "@minecraft/server";
import { namespace } from "../config";

export const command: CustomCommand = {
  name: `${namespace}:greet`,
  description: "Greets someone",
  permissionLevel: CommandPermissionLevel.Any,
  optionalParameters: [
    { name: "target", type: CustomCommandParamType.PlayerSelector },
  ],
};

export function callback(_origin: CustomCommandOrigin, target?: Player): CustomCommandResult {
  if (target === undefined) {
    world.sendMessage(`Hello ${target}`);
  } else {
    world.sendMessage("Hello");
  }

  return {
    status: CustomCommandStatus.Success
  };
}
