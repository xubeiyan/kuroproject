import {
  writeTextFile,
  readFile,
  mkdir,
  exists,
  BaseDirectory,
} from "@tauri-apps/plugin-fs";

// 生成配置文件
export const generateConfigFile = async ({ savePath, hostname }) => {
  const config = {
    asyncMode: "mobile",
    savePath,
    hostname,
  };

  // Tauri v2 这个API真的难用，参考自 https://github.com/tauri-apps/tauri/discussions/12954#discussioncomment-12463610
  if (!(await exists(".", { baseDir: BaseDirectory.AppConfig }))) {
    await mkdir("", { baseDir: BaseDirectory.AppConfig });
  }

  await writeTextFile("config.json", JSON.stringify(config), {
    baseDir: BaseDirectory.AppConfig,
  });
};

// 检测是否有配置文件有则读取
export const getConfigFile = async () => {
  if (
    (await exists(".", { baseDir: BaseDirectory.AppConfig })) &&
    (await exists("./config.json", { baseDir: BaseDirectory.AppConfig }))
  ) {

    // Tauri v2 这个readTextFile API死活读不出来这个文本文件，原谅我用readFile再decoder
    const content = await readFile("config.json", {
      baseDir: BaseDirectory.AppConfig,
    });

    const decodeContent = new TextDecoder().decode(content);

    const configObj = JSON.parse(decodeContent);
    if (configObj != undefined) {
      const { asyncMode, savePath, hostname } = configObj;
      return { asyncMode, asyncFolder: savePath, deviceName: hostname };
    }

    return false;
  } else {
    return false;
  }
};
