import React, { ReactNode, useState } from "react";
import { Newline, render, Text, Box } from "ink";
import init from "./util/init";
import Intro from "./components/Intro";
import Docker from "./components/docker/Docker";
import Periphery from "./components/Periphery";
import Confirm from "./components/Confirm";
import { createUseConfig, createUseSequence } from "./util/state";
import { Config } from "./types";
import Mongo from "./components/deployment-config/Mongo";
import Registry from "./components/deployment-config/Registry";

export const useMainSequence = createUseSequence();
export const useConfig = createUseConfig<Config>({});

init().then(({ flags, dockerInstalled }) => {
  const App = () => {
    const { current, next } = useMainSequence();
    const [periphery, setPeriphery] = useState<boolean | undefined>(
      flags.core ? false : flags.periphery ? true : undefined
    );
    const [installing, setInstalling] = useState(false);

    const corePages: ReactNode[] =
      periphery === false ? [<Mongo />, <Registry />] : [];

    const peripheryPages: ReactNode[] = periphery ? [] : [];

    const pages: ReactNode[] = [
      <Intro />,
      dockerInstalled ? undefined : <Docker />,
      !flags.core && !flags.periphery ? (
        <Periphery setPeriphery={setPeriphery} />
      ) : undefined,
      ...peripheryPages,
      ...corePages,
      <Confirm
        next={() => {
          setInstalling(true);
          next();
        }}
      />,
    ].filter((val) => (val ? true : false));

    return (
      <Box flexDirection="column">
        <Newline />
        <Text color="blue" bold underline>
          Monitor CLI
        </Text>
        <Newline />
        {pages[current]}
      </Box>
    );
  };

  render(<App />);
});