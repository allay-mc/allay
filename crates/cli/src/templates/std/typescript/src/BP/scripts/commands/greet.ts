import * from "@minecraft/server";
import { namespace } from "../config.ts";

export const command: CustomCommand = {
  name: `${namespace}:greet`,
  description: "Greets someone",
  permissionLevel: CustomCommandPermissionLevel.Any,
  optionalParameters: [
    { name: "target", type: CustomCommandParamType.PlayerSelector },
  ],
};

export const callback = (target?: PlayerSelector) => CustomCommandResult {
  if (target === undefined) {
    world.sendMessage(`Hello ${target}`);
  } else {
    world.sendMessage("Hello");
  }
  return CustomCommandResult.Success;
}
