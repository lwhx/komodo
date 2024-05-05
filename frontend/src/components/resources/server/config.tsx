import { Config } from "@components/config";
import { useInvalidate, useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { useState } from "react";

export const ServerConfig = ({ id }: { id: string }) => {
  const perms = useRead("GetPermissionLevel", {
    target: { type: "Server", id },
  }).data;
  const invalidate = useInvalidate();
  const config = useRead("GetServer", { server: id }).data?.config;
  const [update, set] = useState<Partial<Types.ServerConfig>>({});
  const { mutateAsync } = useWrite("UpdateServer", {
    onSuccess: () => {
      // In case of disabling to resolve unreachable alert
      invalidate(["ListAlerts"]);
    },
  });
  if (!config) return null;

  const disabled = perms !== Types.PermissionLevel.Write;

  return (
    <Config
      disabled={disabled}
      config={config}
      update={update}
      set={set}
      onSave={async () => {
        await mutateAsync({ id, config: update });
      }}
      components={{
        general: {
          general: {
            address: true,
            region: true,
            enabled: true,
            stats_monitoring: true,
            auto_prune: true,
          },
        },
        alerts: {
          alerts: {
            send_unreachable_alerts: true,
            send_cpu_alerts: true,
            send_disk_alerts: true,
            send_mem_alerts: true,
          },
        },
        warnings: {
          cpu: {
            cpu_warning: true,
            cpu_critical: true,
          },
          memory: {
            mem_warning: true,
            mem_critical: true,
          },
          disk: {
            disk_warning: true,
            disk_critical: true,
          },
        },
      }}
    />
  );
};