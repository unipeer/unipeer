import {ContractFactory} from "ethers";

export const getInitializerData = (
  ImplFactory: ContractFactory,
  args: unknown[],
  initializer?: string,
): string => {
  const allowNoInitialization = initializer === undefined && args.length === 0;
  initializer = initializer != undefined ? initializer : "initialize";

  try {
    const fragment = ImplFactory.interface.getFunction(initializer);
    return ImplFactory.interface.encodeFunctionData(fragment, args);
  } catch (e: unknown) {
    if (e instanceof Error) {
      if (allowNoInitialization && e.message.includes("no matching function")) {
        return "0x";
      }
    }
    throw e;
  }
};
