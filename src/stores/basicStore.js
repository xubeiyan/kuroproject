import { writable } from "svelte/store";

export const basicStore = writable({
  async_mode: null,
  async_folder: null,
  device_name: null,
});
