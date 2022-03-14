import {
  getBooleanFromEnv,
  getNumberFromEnv,
  getStringFromEnv,
  readJSONFile,
} from "@monitor/util";

export const CORE_SERVER_NAME = getStringFromEnv("CORE_SERVER_NAME", "Monitor Core");
export const SECRETS = readJSONFile("secrets.json");
export const LOG = getBooleanFromEnv("LOG", false);
export const PORT = getNumberFromEnv("PORT", 8000);
export const HOST = getStringFromEnv("HOST", "http://localhost:" + PORT);
export const MONGO_URL = getStringFromEnv(
  "MONGO_URL",
  "mongodb://localhost:27017/monitor"
);
export const TOKEN_EXPIRES_IN = getNumberFromEnv("TOKEN_EXPIRES_IN", 3000);
export const PASSWORD_SALT_ROUNDS = getNumberFromEnv("PASSWORD_SALT_ROUNDS", 8);
export const SYSROOT = getStringFromEnv("SYSROOT", "/home/ubuntu/"); // the root folder monitor has access to, prepends volumes mounted using useSysroot
export const ROOT = getStringFromEnv("ROOT", "/rootDir/"); // the root folder in the container that SYSROOT is mounted on
export const DEPLOYDATA_ROOT = "deployments/";
export const REPO_PATH = ROOT + "builds/";
export const REGISTRY_URL = getStringFromEnv("REGISTRY_URL", "localhost:5000/");
export const FRONTEND_PATH = getStringFromEnv("FRONTEND_PATH", "/frontend")