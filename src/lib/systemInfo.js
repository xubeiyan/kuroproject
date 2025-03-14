import { hostname } from "@tauri-apps/plugin-os";

export const getHostname = async () => {
  const host = await hostname();
  return host;
};
