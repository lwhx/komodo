import { useSetRecentlyViewed } from "@hooks";
import { Resource } from "@layouts/resource";
import { BuildName, BuildVersion } from "./util";
import { useParams } from "react-router-dom";
import { RebuildBuild } from "./components/actions";

export const Build = () => {
  const { buildId } = useParams();
  const push = useSetRecentlyViewed();

  if (!buildId) return null;
  push("Build", buildId);

  return (
    <Resource
      title={<BuildName id={buildId} />}
      info={
        <div className="text-muted-foreground">
          <BuildVersion id={buildId} />
        </div>
      }
      actions={<RebuildBuild buildId={buildId} />}
    />
  );
};
