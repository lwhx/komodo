import { BuildActionStates, DeployActionStates } from "@monitor/types";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";

interface ActionState {
  add(id: string): void;
  delete(id: string): void;
  set(id: string, type: string, state: boolean): void;
  get(id: string, type: string): boolean;
  getMultiple(id: string, types: string[]): boolean;
}

declare module "fastify" {
  interface FastifyInstance {
    buildActionStates: ActionState;
    deployActionStates: ActionState;
  }
}

export const PULLING = "pulling";
export const BUILDING = "building";
export const DEPLOYING = "deploying";
export const STARTING = "starting";
export const STOPPING = "stopping";
export const DELETING = "deleting";

const actionStates = fp((app: FastifyInstance, _: {}, done: () => void) => {
  const buildActionStates: BuildActionStates = {};
  const deployActionStates: DeployActionStates = {};

  app.decorate("buildActionStates", {
    add: (buildID: string) => {
      buildActionStates[buildID] = {
        pulling: false,
        building: false,
      };
    },
    delete: (buildID: string) => {
      delete buildActionStates[buildID];
    },
    set: (buildID: string, type: string, state: boolean) => {
      buildActionStates[buildID][type] = state;
    },
    get: (buildID: string, type: string) => {
      return buildActionStates[buildID][type];
    },
    getMultiple: (buildID: string, types: string[]) => {
      for (const type of types) {
        if (buildActionStates[buildID][type]) return true;
      }
      return false;
    },
  });

  app.decorate("deployActionStates", {
    add: (deploymentID: string) => {
      deployActionStates[deploymentID] = {
        deploying: false,
        deleting: false,
        starting: false,
        stopping: false,
      };
    },
    delete: (deploymentID: string) => {
      delete deployActionStates[deploymentID];
    },
    set: (deploymentID: string, type: string, state: boolean) => {
      deployActionStates[deploymentID][type] = state;
    },
    get: (deploymentID: string, type: string) => {
      return deployActionStates[deploymentID][type];
    },
    getMultiple: (deploymentID: string, types: string[]) => {
      for (const type of types) {
        if (deployActionStates[deploymentID][type]) return true;
      }
      return false;
    },
  });

  app.builds.find({}, { _id: true }).then((builds) => {
    builds.forEach((build) => app.buildActionStates.add(build._id!));
  });
  app.deployments.find({}, { _id: true }).then((deployments) => {
    deployments.forEach((deployment) =>
      app.deployActionStates.add(deployment._id!)
    );
  });

  done();
});

export default actionStates;