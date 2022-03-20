import React, { Fragment, useEffect, useState } from "react";
import { Box, Newline, Text } from "ink";
import { useConfig } from "../cli";
import deploy, { Stage, Update } from "../util/helpers/deploy";
import { Config } from "../types";

const Setup = () => {
  const { config } = useConfig();
  const [updates, setUpdates] = useState<Update[]>([getInitialUpdate(config)]);
  const [finished, setFinished] = useState(false);

  useEffect(() => {
    deploy(config, (update) =>
      setUpdates((updates) => {
        const newUpdates = [update, getNextUpdate(update)].filter(
          (val) => val
        ) as Update[];
        return [...updates, ...newUpdates];
      })
    ).then(() => setFinished(true));
  }, []);

  useEffect(() => {
    if (finished) process.exit();
  }, [finished]);

  return (
    <Box flexDirection="column">
      <Text>
        setting up{" "}
        {config.core ? (
          <Text color="cyan">monitor core</Text>
        ) : (
          <Text color="red">monitor periphery</Text>
        )}
        ...
      </Text>
      <Newline />
      {updates.map(({ stage, result, description }, i) => (
        <Fragment key={i}>
          <Text>
            {description} -{" "}
            <Text color="gray">
              ({getStageNumber(stage)} of {config.core ? 4 : 1})
            </Text>
          </Text>
          {result && (
            <Box marginLeft={2} flexDirection="column">
              <Text color="green">
                command: <Text color="white">{result.command}</Text>
              </Text>
              {result.log.stdout ? (
                <Text color="green">
                  stdout: <Text color="white">{result.log.stdout}</Text>
                </Text>
              ) : undefined}
              {result.log.stderr ? (
                <Text color="red">
                  stderr: <Text color="white">{result.log.stderr}</Text>
                </Text>
              ) : undefined}
            </Box>
          )}
        </Fragment>
      ))}
      {finished && (
        <Fragment>
          <Newline />
          <Text>
            setup <Text color="green">finished</Text>.
          </Text>
        </Fragment>
      )}
      <Newline />
    </Box>
  );
};

function getInitialUpdate(config: Config): Update {
  if (config.core) {
    if (config.mongo?.startConfig) {
      return {
        stage: "mongo",
        description: "starting mongo",
      };
    } else if (config.registry?.startConfig) {
      return {
        stage: "periphery",
        description: "starting registry",
      };
    } else {
      return {
        stage: "core",
        description: "starting monitor core",
      };
    }
  } else {
    return {
      stage: "periphery",
      description: "starting monitor periphery",
    };
  }
}

function getNextUpdate({ stage }: Update): Update | undefined {
  switch (stage) {
    case "mongo":
      return {
        stage: "periphery",
        description: "starting registry...",
      };

    case "registry":
      return {
        stage: "core",
        description: "starting monitor core...",
      };

    case "core":
      return {
        stage: "docs",
        description: "adding configurations to db...",
      };
  }
}

function getStageNumber(stage: Stage) {
  switch (stage) {
    case "mongo":
      return 1;
    case "registry":
      return 2;
    case "core":
      return 3;
    case "docs":
      return 4;
    case "periphery":
      return 1;
  }
}

export default Setup;