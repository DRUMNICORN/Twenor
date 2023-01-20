import React, { useState } from "react";
import { appWindow } from "@tauri-apps/api/window";

//tauri add emit event
import { emit } from "@tauri-apps/api/event";

import Button from "./util/Button";
import "../styles/Titlebar.scss";
import Api from "../Api";

function TitleBar() {
  const [isReloading, setReloading] = useState(false);

  return (
    <div className="titlebar" data-tauri-drag-region>
      <div className="titlebar-left" data-tauri-drag-region>
        <Button name="import" link="ant-design/import-outlined" onClick={() => requestImportXml()} />
        <Button name="export" link="ant-design/export-outlined" onClick={() => requestExportXml()} />
      </div>

      <div className="titlebar-right" data-tauri-drag-region>
        <Button name="reload" link="ant-design/reload-outlined" onClick={() => requestReload()} />
        <Button name="setting" link="ant-design/setting-outlined" onClick={() => emit("toggleSetting")} />
        <Button name="minimize" link="codicon/chrome-minimize" onClick={() => appWindow.minimize()} />
        <Button name="maximize" link="codicon/chrome-maximize" onClick={() => appWindow.toggleMaximize()} />
        <Button name="close" link="ant-design/close-outlined" onClick={() => appWindow.close()} />
      </div>
    </div>
  );

  async function requestImportXml() {
    // open file dialog and select xml file to import into the track library

    let xml_path = await Api.requestOpenFileDialog();
    console.log(` <= Import Xml: ${xml_path}`);
    Api.requestImportXml(xml_path);
  }

  function requestExportXml() {}

  function requestReload() {
    if (isReloading) {
      return;
    }

    setReloading(true);
    let reloadButton = document.getElementById("reload");
    reloadButton?.classList.add("spin");
    Api.requestReload();

    setTimeout(() => {
      reloadButton?.classList.remove("spin");
      setReloading(false);
    }, 1000);
  }
}

export default TitleBar;
